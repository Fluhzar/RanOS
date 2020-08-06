/*! ****************************************************************************
\file             Time.hpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __TIME_HPP
#define __TIME_HPP

// Include Files                ////////////////////////////////////////////////

#include "Macro.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace RanOS
{
    void ResetTimer();
    void UpdateTimer();

    float DT();
}   // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __TIME_HPP
