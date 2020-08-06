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

    \param num_panels
        The number of panels running.

    \param frame_duration
        The duration of a single animation frame.

    \param breath_length
        The length of a single breath on a single color.

    \param color_options
        The colors that the breath will cycle through linearly.

    \param size
        The number of colors that will be cycled through.
    ***************************************************************************/
    void BreathingInit(uint16_t num_panels, float frame_duration, float breath_length,
                       RGB const * color_options, uint16_t size);

    /*! ************************************************************************
    \brief
        Updates the Frame with new values. It is assumed that this function is
        called at the rate of the frame_duration passed into BreathingInit.
    ***************************************************************************/
    void Breathing(void);

    /*! ************************************************************************
    \brief
        Returns a pointer to the internal Frame cointaining the data for the
        Breathing animation.

    \return
        Pointer to the internal Frame.
    ***************************************************************************/
    Frame const * BreathingFrame(void);
}   // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __BREATHING_HPP
