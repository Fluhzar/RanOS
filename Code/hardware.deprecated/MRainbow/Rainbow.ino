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
        Frame Output;
        float Hue;
        float Saturation;
        float Value;
        float dH;
    };

    static RainbowOptions options;
} // namespace RanOS

// Private Function Declarations          //////////////////////////////////////

static RanOS::RGB HSV2RGB(float H, float S, float V);

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    void RainbowInit(uint16_t num_panels, float frame_duration, float rainbow_length, float saturation, float value)
    {
        options.Output = Frame(num_panels * PANEL_SIZE, frame_duration);
        options.Hue = 0;
        options.Saturation = saturation;
        options.Value = value;
        options.dH = 360.f * frame_duration / rainbow_length;
    }

    void Rainbow(void)
    {
        options.Hue += options.dH;

        if(options.Hue >= 360)
        {
            options.Hue -= 360;
        }

        static uint16_t i;
        for(i = 0; i < options.Output.m_Size; ++i)
        {
            options.Output.m_Frame[i] =
                HSV2RGB(
                    options.Hue + float(i)/float(options.Output.m_Size)*360.f,
                    options.Saturation,
                    options.Value
            );
        }
    }

    Frame const * RainbowFrame()
    {
        return &options.Output;
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
