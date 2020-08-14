#include <Arduino.h>

#include "animation/Animation.hpp"
#include "animation/Breath.hpp"
#include "animation/Rainbow.hpp"
#include "draw/APA102CDraw.hpp"
#include "draw/DrawStats.hpp"

#define DATA_PIN ::RanOS::Pin(0)
#define CLOCK_PIN ::RanOS::Pin(1)
#define NUM_LEDS 16
#define TARGET_DT ::RanOS::Option<::RanOS::Duration>::Some(::RanOS::Duration(1.0 / 144.0))
#define BRIGHTNESS 0.125
#define RANDOM false

RanOS::APA102CDraw drawer(DATA_PIN, CLOCK_PIN, TARGET_DT);
RanOS::Vec<RanOS::RGB> colors{RanOS::RGB::from_hsv(0.0, 1.0, 1.0), RanOS::RGB::from_hsv(30.0, 1.0, 1.0), RanOS::RGB::from_hsv(60.0, 1.0, 1.0), RanOS::RGB::from_hsv(120.0, 1.0, 1.0), RanOS::RGB::from_hsv(210.0, 1.0, 1.0), RanOS::RGB::from_hsv(280.0, 1.0,1.0)};
RanOS::Option<RanOS::Vec<RanOS::RGB>> order(RANDOM ? RanOS::Option<RanOS::Vec<RanOS::RGB>>::None() : RanOS::Option<RanOS::Vec<RanOS::RGB>>::Some(colors) );
RanOS::Box<RanOS::Breath> breath = NEW_BOX(RanOS::Breath, RanOS::Duration(4*6), RanOS::Duration(4), BRIGHTNESS, NUM_LEDS, order);
RanOS::Box<RanOS::Rainbow> rainbow = NEW_BOX(RanOS::Rainbow, RanOS::Duration(4 * 6), RanOS::Duration(4), BRIGHTNESS, NUM_LEDS, 1.0, 1.0, 1.0, 1);

void setup()
{
    drawer.stop(NUM_LEDS*NUM_LEDS);

    Serial.begin(9600);
    while(!Serial) {}
}

void loop() {
    drawer.push_queue(NEW_RC(RanOS::Breath, *breath));
    drawer.push_queue(NEW_RC(RanOS::Rainbow, *rainbow));
    drawer.run();

    Serial.println(drawer.get_stats().to_string().c_str());
}
