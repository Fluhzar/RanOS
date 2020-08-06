/*! ****************************************************************************
\file             stub.ino
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
#include "Breathing.hpp"
#include "Time.hpp"

// Private Macros                         //////////////////////////////////////

#define CLOCK_PIN 3
#define DATA_PIN 4

#define FRAME_DURATION (1.f/POVHZ)
#define NUM_PANELS 1

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

    // Construct a controller object with the given parameters
static RanOS::Controller controller(DATA_PIN, CLOCK_PIN, nullptr, NUM_PANELS);
    // Construct a animation object with the animation functions and the controller pointer
static RanOS::Animation breath(RanOS::BreathingReset, RanOS::BreathingUpdate, NUM_PANELS, FRAME_DURATION, &controller);

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

void setup()
{
        // Set the pins to output
    pinMode(CLOCK_PIN, OUTPUT);
    pinMode(DATA_PIN,  OUTPUT);

        // Optional, all values are default initialized at the start of the program
    RanOS::ResetTimer();
}

void loop()
{
        // Update the dt value with the new frame
    RanOS::UpdateTimer();

        // Update the animation with the current dt value
    breath.Update(RanOS::DT());
}

// Private Functions                      //////////////////////////////////////
