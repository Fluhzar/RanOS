/*! ****************************************************************************
\file             Macros.hpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __MACROS_HPP
#define __MACROS_HPP

// Include Files                ////////////////////////////////////////////////

#include <cstdint>

// Public Macros                ////////////////////////////////////////////////

#ifndef MAX_LEDS
    #define MAX_LEDS (PANEL_SIZE*80) ///< Maximum supported LEDs. 80 for Teensy 3.6 RAM limits, leaving plenty extra for other uses.
#endif

#ifndef PANEL_SIZE
    #define PANEL_SIZE 64 ///< The number of LEDs for panel. Currently set for a panel with 8x8 LEDs
#endif

#define POVHZ 400   ///< Minimum cycle rate to have continuous persistence of vision. For a nice binary number, increase to 512; default: 400

#endif  // __MACROS_HPP
