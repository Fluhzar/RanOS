/*! ****************************************************************************
\file             MRainbow.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <Arduino.h>

#include "Macro.hpp"

#include "Animation.hpp"
#include "Rainbow.hpp"
#include "Time.hpp"

// Private Macros                         //////////////////////////////////////

#define CLOCK 3
#define DATA 4

#define NUM_PANELS 1
#define BRIGHTNESS 0x1F // range: 0 to 31, 0x00 to 0x1F

#define SAT 1.0f
#define VAL 1.0f

#define RAINBOW_LENGTH 10.f
#define FRAME_DURATION 1.f/float(POVHZ)

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

static RanOS::Animation * rainbow;

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

void setup()
{
    pinMode(CLOCK, OUTPUT);
    pinMode(DATA,  OUTPUT);

    RanOS::RainbowInit(NUM_PANELS, FRAME_DURATION, RAINBOW_LENGTH, SAT, VAL);

    rainbow = new RanOS::Animation(RanOS::Rainbow, RanOS::RainbowFrame(), NUM_PANELS, DATA, CLOCK);
    rainbow->GetController().Brightness(BRIGHTNESS);

    RanOS::ResetTimer();
}

void loop()
{
    RanOS::UpdateTimer();

    rainbow->Update(RanOS::DT());
}

// Private Functions                      //////////////////////////////////////
