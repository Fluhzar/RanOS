/*! ****************************************************************************
\file             Time.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include "Macro.hpp"

#include <Arduino.h>

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Function Declarations          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

namespace RanOS
{
    static unsigned long ctime = 0;
    static unsigned long ptime = 0;
    static float dt = 0;
}   //namespace RanOS

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    void ResetTimer()
    {
        ptime = ctime = micros();
        dt = 0;
    }

    void UpdateTimer()
    {
        ptime = ctime;
        ctime = micros();

        dt = (ctime - ptime)/1'000'000.0f;
    }

    float DT()
    {
        return dt;
    }
}   // namespace RanOS

// Private Functions                      //////////////////////////////////////
