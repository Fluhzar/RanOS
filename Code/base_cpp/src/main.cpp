#include <Arduino.h>

#include "animation/Animation.hpp"
#include "animation/Breath.hpp"
#include "animation/Rainbow.hpp"
#include "animation/Strobe.hpp"
#include "draw/APA102CDraw.hpp"
#include "draw/DrawStats.hpp"

#define DATA_PIN ::RanOS::Pin(0)
#define CLOCK_PIN ::RanOS::Pin(1)
#define NUM_LEDS 16
#define TARGET_DT ::RanOS::Option<::RanOS::Duration>::Some(::RanOS::Duration(1.0 / 144.0))
#define BRIGHTNESS 0.125
#define RANDOM false
#define WAIT_TIME ::RanOS::Duration(30)

RanOS::APA102CDraw drawer(DATA_PIN, CLOCK_PIN, TARGET_DT);
RanOS::Vec<RanOS::RGB> colors{RanOS::RGB::from_hsv(0.0, 1.0, 1.0), RanOS::RGB::from_hsv(30.0, 1.0, 1.0), RanOS::RGB::from_hsv(60.0, 1.0, 1.0), RanOS::RGB::from_hsv(120.0, 1.0, 1.0), RanOS::RGB::from_hsv(210.0, 1.0, 1.0), RanOS::RGB::from_hsv(280.0, 1.0,1.0)};
RanOS::Option<RanOS::Vec<RanOS::RGB>> order(RANDOM ? RanOS::Option<RanOS::Vec<RanOS::RGB>>::None() : RanOS::Option<RanOS::Vec<RanOS::RGB>>::Some(colors) );
RanOS::Box<RanOS::Breath> breath = NEW_BOX(RanOS::Breath, RanOS::Duration(4*6), RanOS::Duration(4), BRIGHTNESS, NUM_LEDS, order);
RanOS::Box<RanOS::Rainbow> rainbow = NEW_BOX(RanOS::Rainbow, RanOS::Duration(32), RanOS::Duration(16), BRIGHTNESS, NUM_LEDS, 1.0, 1.0, 0.0, 1);
RanOS::Box<RanOS::Strobe> strobe = NEW_BOX(RanOS::Strobe, RanOS::Duration(8), BRIGHTNESS, NUM_LEDS, RanOS::RGB::from_code(0x00'00'FF'FF, RanOS::O_RGB), RanOS::Duration(0.0625), 0.25);

bool enable_serial = true;

void setup()
{
    drawer.stop(NUM_LEDS*NUM_LEDS);

    Serial.begin(9600);
    RanOS::Instant before = TIME_NOW();
    while(!Serial) {
        if(TIME_NOW() - before > WAIT_TIME) {
            enable_serial = false;
            break;
        }
    }
}

void loop() {
    // drawer.push_queue(NEW_RC(RanOS::Strobe, *strobe));
    drawer.push_queue(NEW_RC(RanOS::Breath, *breath));
    drawer.push_queue(NEW_RC(RanOS::Rainbow, *rainbow));
    drawer.run();

    if(enable_serial) {
        Serial.println(drawer.get_stats().to_string().c_str());
    }
}
