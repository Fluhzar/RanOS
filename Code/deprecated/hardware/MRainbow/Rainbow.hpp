/*! ****************************************************************************
\file             Rainbow.hpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __RAINBOW_HPP
#define __RAINBOW_HPP

// Include Files                ////////////////////////////////////////////////

#include <cstdint>

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

namespace RanOS
{
    struct Frame; // Animation.hpp
    struct RGB;   // RGB.hpp
} // namespace RanOS

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

// Public Functions             ////////////////////////////////////////////////

namespace RanOS
{
    void RainbowInit(uint16_t num_panels, float frame_duration, float rainbow_length, float saturation, float value);

    void Rainbow(void);

    Frame const * RainbowFrame(void);
} // namespace RanOS

#endif // __RAINBOW_HPP
