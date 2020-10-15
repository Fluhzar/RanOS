/*! ****************************************************************************
\file             RGB.hpp
\author           Fluhzar
\par    Email:    fluhzar\@.pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __RGB_HPP
#define __RGB_HPP

// Include Files                ////////////////////////////////////////////////

#include "Macro.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Forward Namespace References ////////////////////////////////////////////////

// Public Namespace Enums       ////////////////////////////////////////////////

// Public Namespace Objects     ////////////////////////////////////////////////

namespace RanOS
{
    /*! ************************************************************************
    \brief
        Plain-old-data struct to contain red, green, and blue color values.
    ***************************************************************************/
    struct RGB
    {
        // Members              ///////////////////////

            /// Anonymous union for red color value.
        union
        {
            uint8_t r;
            uint8_t red;
        };
            /// Anonymous union for green color value.
        union
        {
            uint8_t g;
            uint8_t green;
        };
            /// Anonymous unioun for blue color value.
        union
        {
            uint8_t b;
            uint8_t blue;
        };

        // Con-/De- structors   ///////////////////////

        /*! ********************************************************************
        \brief
            Default constructor.

        \param r
            Red color value.

        \param g
            Green color value.

        \param b
            Blue color value.
        ***********************************************************************/
        inline RGB(uint8_t r = 0x0, uint8_t g = 0x0, uint8_t b = 0x0) :
            red(r), green(g), blue(b) {};
    };  // RGB
}   // namespace RanOS

// Public Namespace Functions   ////////////////////////////////////////////////

#endif // __RGB_HPP
