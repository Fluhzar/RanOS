/*! ****************************************************************************
\file             Rainbow.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <cmath>
#include <cstdint>

#include "Animation.hpp"
#include "RGB.hpp"
#include "Rainbow.hpp"

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

namespace RanOS
{
    struct RainbowOptions
    {
        float Hue;
        float Saturation;
        float Value;
        float dH;
    };

    static RainbowOptions rainbowOpt;
} // namespace RanOS

// Private Function Declarations          //////////////////////////////////////

static RanOS::RGB HSV2RGB(float H, float S, float V);

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    void RainbowInit(float frame_duration, float rainbow_length, float saturation, float value)
    {
            // Set the HSV values for the HSV2RGB calculation
        rainbowOpt.Hue = 0;
        rainbowOpt.Saturation = saturation;
        rainbowOpt.Value = value;
            // Set up the delta-angle for the Hue of the HSV2RGB calculation
        rainbowOpt.dH = 360.f * frame_duration / rainbow_length;
    }

    void RainbowReset(Frame * frame)
    {
        rainbowOpt.Hue = 0;

        std::memset(frame->m_Colors, 0, frame->m_Size * sizeof(RGB));
    }

    void RainbowUpdate(Frame * frame)
    {
            // Increment the hue value
        rainbowOpt.Hue += rainbowOpt.dH;

            // Wrap the hue value if needed
        if(rainbowOpt.Hue >= 360)
        {
            rainbowOpt.Hue -= 360;
        }

            // Compute the RGB value for each LED
        static uint16_t i;
        for(i = 0; i < frame->m_Size; ++i)
        {
            frame->m_Colors[i] =
                HSV2RGB(
                    rainbowOpt.Hue + float(i)/float(frame->m_Size)*360.f,
                    rainbowOpt.Saturation,
                    rainbowOpt.Value
            );
        }
    }
} // namespace RanOS

// Private Functions                      //////////////////////////////////////

static RanOS::RGB HSV2RGB(float H, float S, float V)
{
    H = fmodf(H, 360);

    float C = V * S;
    float X = C * (1 - std::abs(fmodf(H/60.f, 2) - 1));
    float m = V - C;
    float R, G, B;

    if(H >= 0 && H < 60)
    {
        R = C;
        G = X;
        B = 0;
    }
    else if(H >= 60 && H < 120)
    {
        R = X;
        G = C;
        B = 0;
    }
    else if(H >= 120 && H < 180)
    {
        R = 0;
        G = C;
        B = X;
    }
    else if(H >= 180 && H < 240)
    {
        R = 0;
        G = X;
        B = C;
    }
    else if(H >= 240 && H < 300)
    {
        R = X;
        G = 0;
        B = C;
    }
    else
    {
        R = C;
        G = 0;
        B = X;
    }

    return RanOS::RGB(uint8_t((R+m) * 255),uint8_t((G+m) * 255),uint8_t((B+m) * 255));
}
