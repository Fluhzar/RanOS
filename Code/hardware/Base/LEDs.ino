/*! ****************************************************************************
\file             LEDs.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <cstring>

#include <Arduino.h>

#include "Macro.hpp"

#include "LEDs.hpp"

// Private Macros                         //////////////////////////////////////

#ifndef NOP
    #define NOP() //asm("nop\n");
#endif

// Private Enums                          //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    Controller::Controller(uint8_t data, uint8_t clock, RGB const * LEDs, uint16_t num_panels, uint8_t b) :
        m_Data(data), m_Clock(clock), m_Brightness(b>0x1F ? 0x1F : b),
        m_EndFrame((num_panels * PANEL_SIZE > 64) ? &Controller::LargeEndFrame : &Controller::SmallEndFrame),
        m_Buffer(LEDs), m_BufSize(num_panels * PANEL_SIZE)
    {
    }

    void Controller::Display() const
    {
            // Send start frame
        StartFrame();
            // Send data
        for(uint32_t i = 0; i < m_BufSize; ++i)
        {
          Write(0b11100000 | m_Brightness);
          Write(m_Buffer[i].r);
          Write(m_Buffer[i].b);
          Write(m_Buffer[i].g);
        }
            // Send end frame
        (this->*Controller::m_EndFrame)();
    }

    // Private Functions                    ////////////////////////////////////

    inline void Controller::StartFrame() const
    {
        digitalWrite(m_Data, LOW);
        digitalWrite(m_Clock, LOW);

        Write(0x00);
        Write(0x00);
        Write(0x00);
        Write(0x00);
    }

    void Controller::SmallEndFrame() const
    {
        Write(0xFF);
        Write(0xFF);
        Write(0xFF);
        Write(0xFF);
    }

    void Controller::LargeEndFrame() const
    {
        for(uint16_t i = 0; i < (m_BufSize >> 4); ++i)
        {
            Write(0xFF);
        }
    }

    inline void Controller::Write(uint8_t const & byte) const
    {
        // NOP added to let clock stay high for a bit of time, can (probably should) set define to nothing

        digitalWrite(m_Data, (byte>>7) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, (byte>>6) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, (byte>>5) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, (byte>>4) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, (byte>>3) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, (byte>>2) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, (byte>>1) & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);

        digitalWrite(m_Data, byte & 0x1);
        digitalWrite(m_Clock, HIGH);
        NOP()
        digitalWrite(m_Clock, LOW);
    }
} // namespace RanOS
