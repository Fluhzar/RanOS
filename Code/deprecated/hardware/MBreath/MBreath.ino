/*! ****************************************************************************
\file             MBreath.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2018 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <Arduino.h>

#include <cstdint>

#include "LEDs.hpp"
#include "Time.hpp"
#include "Breathing.hpp"
#include "Animation.hpp"

// Private Macros                         //////////////////////////////////////

#define DATA_PIN 4
#define CLOCK_PIN 3

#define NUM_PANELS 1
#define BRIGHTNESS 0x1F                         // range: 0 to 31, 0x00 to 0x1F

#define BREATH_LENGTH (float(10)/float(1))      // in seconds
#define FRAME_DURATION float(1)/float(POVHZ)    // in seconds

#define MIN_VALUE 0x00
#define MAX_VALUE 0x80

// Private Enums                          //////////////////////////////////////

// Private Function Declarations          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

static RanOS::RGB const Colors[] = {
    RanOS::RGB(MAX_VALUE,    MIN_VALUE,    MIN_VALUE),    // Red
    RanOS::RGB(MAX_VALUE>>1, MAX_VALUE>>1, MIN_VALUE),    // Yellow
    RanOS::RGB(MIN_VALUE,    MAX_VALUE,    MIN_VALUE),    // Green
    RanOS::RGB(MIN_VALUE,    MAX_VALUE>>1, MAX_VALUE>>1), // Cyan
    RanOS::RGB(MIN_VALUE,    MIN_VALUE,    MAX_VALUE),    // Blue
    RanOS::RGB(MAX_VALUE>>1, MIN_VALUE,    MAX_VALUE>>1)  // Magenta
};

static RanOS::Animation * breath = nullptr;

// Public Functions                       //////////////////////////////////////

void setup()
{
    pinMode(CLOCK_PIN, OUTPUT);
    pinMode(DATA_PIN,  OUTPUT);

    RanOS::BreathingInit(NUM_PANELS, FRAME_DURATION, BREATH_LENGTH, Colors, sizeof(Colors)/sizeof(*Colors));

    breath = new RanOS::Animation(RanOS::Breathing, RanOS::BreathingFrame(), NUM_PANELS, DATA_PIN, CLOCK_PIN);
    breath->GetController().Brightness(BRIGHTNESS);

    RanOS::ResetTimer();
}

void loop()
{
    RanOS::UpdateTimer();

    breath->Update(RanOS::DT());
}

// Private Functions                      //////////////////////////////////////
