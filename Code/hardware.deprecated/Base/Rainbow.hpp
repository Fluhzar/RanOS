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
    /*! ************************************************************************
    \brief
        Initializes the values that run the Rainbow animation.

    \param frame_duration
        The duration of a single animation frame.

    \param rainbow_length
        The length of time it takes for the rainbow to cycle through all of the
        colors of the rainbow.

    \param saturation
        Saturation for the HSV2RGB calculation.

    \param value
        Value for the HSV2RGB calculation.
    ***************************************************************************/
    void RainbowInit(float frame_duration, float rainbow_length, float saturation, float value);

    /*! ************************************************************************
    \brief
        Resets the animation to it's original state so that an animation can be
        restarted.

    \param frame
        Pointer to the frame where the color values are reset.
    ***************************************************************************/
    void RainbowReset(Frame * frame);

    /*! ************************************************************************
    \brief
        Updates the Frame with new values. It is assumed that this function is
        called at the rate of the frame_duration passed into `RainbowInit`.
    \param frame
        Pointer to the frame where the color values are updated.
    ***************************************************************************/
    void RainbowUpdate(Frame * frame);
} // namespace RanOS

#endif // __RAINBOW_HPP
