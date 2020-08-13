#include <Arduino.h>

#include "animation/Animation.hpp"
#include "animation/Breath.hpp"
#include "draw/APA102CDraw.hpp"
#include "draw/DrawStats.hpp"

#define DATA_PIN RanOS::Pin(0)
#define CLOCK_PIN RanOS::Pin(1)
#define NUM_LEDS 256
#define RANDOM false

RanOS::APA102CDraw drawer(DATA_PIN, CLOCK_PIN);
RanOS::Vec<RanOS::RGB> colors{RanOS::RGB::from_hsv(0.0, 1.0, 1.0), RanOS::RGB::from_hsv(30.0, 1.0, 1.0), RanOS::RGB::from_hsv(60.0, 1.0, 1.0), RanOS::RGB::from_hsv(120.0, 1.0, 1.0), RanOS::RGB::from_hsv(210.0, 1.0, 1.0), RanOS::RGB::from_hsv(280.0, 1.0,1.0)};
RanOS::Option<RanOS::Vec<RanOS::RGB>> order(RANDOM ? RanOS::Option<RanOS::Vec<RanOS::RGB>>::None() : RanOS::Option<RanOS::Vec<RanOS::RGB>>::Some(colors) );
RanOS::Breath * breath = new RanOS::Breath(RanOS::Duration(4*6), RanOS::Duration(4), 0.125, NUM_LEDS, order);

void setup()
{
    Serial.begin(9600);
    while(!Serial) {}
}


void loop() {
    drawer.push_queue(new RanOS::Breath(*breath));
    drawer.run();
    drawer.stop(NUM_LEDS+1);

    Serial.println(drawer.get_stats().to_string().c_str());
}
