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
        Frame Output;
        RGB * ColorOptions;
        uint16_t ColorSize;
        uint16_t ColorInd;
        float Brightness;
        float Acc;
        float Vel;
        float Vel0;
    };

    static Breath options;
}

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    void BreathingInit(uint16_t num_panels, float frame_duration, float breath_length,
                    RGB const * color_options, uint16_t size)
    {
        options.Output = Frame(num_panels * PANEL_SIZE, frame_duration);
        options.ColorOptions = new RGB[size];
        options.ColorSize = size;
        std::memcpy(options.ColorOptions, color_options, size*sizeof(RGB));
        options.ColorInd = 0;
        options.Brightness = 0;
            // Based on the arc of a parabola and realistic physics
        options.Acc = -8.f/(breath_length*breath_length);
        options.Vel0 = options.Vel = 4.f/breath_length;
    }

    void Breathing(void)
    {
            // Update the velocity and brightness
        options.Brightness += (options.Vel += options.Acc * options.Output.m_FrameDuration) * options.Output.m_FrameDuration;

            // If the end of this color has been reached
        if(options.Brightness <= 0.f && options.Vel < 0)
        {
                // Update the values for the next color
            ++options.ColorInd %= options.ColorSize;
            options.Brightness = -options.Brightness;
            options.Vel = options.Vel0;
        }

            // Calculate the current color values
        static RGB InterColor;
        InterColor = options.ColorOptions[options.ColorInd];
        InterColor.r *= options.Brightness;
        InterColor.g *= options.Brightness;
        InterColor.b *= options.Brightness;

            // Set the color values to all values in the array
        static uint16_t i;
        for(i = 0; i < options.Output.m_Size; ++i)
        {
            std::memcpy(options.Output.m_Frame+i, &InterColor, sizeof(RanOS::RGB));
        }
    }

    Frame const * BreathingFrame(void)
    {
        return &options.Output;
    }
}   // namespace RanOS

// Private Functions                      //////////////////////////////////////
