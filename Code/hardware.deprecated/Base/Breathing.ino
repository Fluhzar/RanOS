/*! ****************************************************************************
\file             Breathing.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <cstdint>

#include "Animation.hpp"
#include "RGB.hpp"
#include "Breathing.hpp"

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

namespace RanOS
{
    struct Breath
    {
        RGB * ColorOptions;
        uint16_t ColorSize;
        uint16_t ColorInd;
        float Brightness;
        float Acc;
        float Vel;
        float Vel0;
    };

    static Breath breathOpt;
}

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    void BreathingInit(float frame_duration, float breath_length, RGB const * color_options, uint16_t size)
    {
            // Set the color options and size
        breathOpt.ColorOptions = new RGB[size];
        breathOpt.ColorSize = size;
        std::memcpy(breathOpt.ColorOptions, color_options, size*sizeof(RGB));
            // Set the starting values for different tracking variables
        breathOpt.ColorInd = 0;
        breathOpt.Brightness = 0;
            // Set the acceleration and velocity based on the arc of a parabola and realistic physics
        breathOpt.Acc = -8.f/(breath_length*breath_length);
        breathOpt.Vel0 = breathOpt.Vel = 4.f/breath_length;
    }

    void BreathingReset(Frame * frame)
    {
        breathOpt.ColorInd = 0;
        breathOpt.Brightness = 0;
        breathOpt.Vel = 0;

        std::memset(frame->m_Colors, 0, frame->m_Size * sizeof(RGB));
    }

    void BreathingUpdate(Frame * frame)
    {
            // Update the velocity and brightness
        breathOpt.Brightness += (breathOpt.Vel += breathOpt.Acc * frame->m_FrameDuration) * frame->m_FrameDuration;

            // If the end of this color has been reached
        if(breathOpt.Brightness <= 0.f && breathOpt.Vel < 0)
        {
                // Update the values for the next color
            ++breathOpt.ColorInd %= breathOpt.ColorSize;
            breathOpt.Brightness = -breathOpt.Brightness;
            breathOpt.Vel = breathOpt.Vel0;
        }

            // Calculate the current color values
        static RGB InterColor;
        InterColor = breathOpt.ColorOptions[breathOpt.ColorInd];
        InterColor.r *= breathOpt.Brightness;
        InterColor.g *= breathOpt.Brightness;
        InterColor.b *= breathOpt.Brightness;

            // Set the color values to all values in the array
        static uint16_t i;
        for(i = 0; i < frame->m_Size; ++i)
        {
            std::memcpy(frame->m_Colors+i, &InterColor, sizeof(RanOS::RGB));
        }
    }
}   // namespace RanOS

// Private Functions                      //////////////////////////////////////
