/*! ****************************************************************************
\file             Breathing.hpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __BREATHING_HPP
#define __BREATHING_HPP

// Include Files                ////////////////////////////////////////////////

#include "Macro.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

namespace RanOS
{
    struct Frame;
    struct RGB;
}   // namespace RanOS

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace RanOS
{
    /*! ************************************************************************
    \brief
        Initializes the values that run the Breath animation.

    \param frame_duration
        The duration of a single animation frame.

    \param breath_length
        The length of a single breath on a single color.

    \param color_options
        The colors that the breath will cycle through linearly.

    \param size
        The number of colors that will be cycled through.
    ***************************************************************************/
    void BreathingInit(float frame_duration, float breath_length, RGB const * color_options, uint16_t size);

    /*! ****************************************************************************
    \brief
        Resets the animation to it's original state so that an animation can be
        restarted.

    \param frame
        Pointer to the frame where the color values are reset.
    *******************************************************************************/
    void BreathingReset(Frame * frame);

    /*! ************************************************************************
    \brief
        Updates the Frame with new values. It is assumed that this function is
        called at the rate of the frame_duration passed into `BreathingInit`.

    \param frame
        Pointer to the frame where the color values are updated.
    ***************************************************************************/
    void BreathingUpdate(Frame * frame);
}   // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __BREATHING_HPP
