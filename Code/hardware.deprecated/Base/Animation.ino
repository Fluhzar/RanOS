/*! ****************************************************************************
\file             Animation.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <cstdint>
#include <cstring>

#include "RGB.hpp"
#include "LEDs.hpp"
#include "Animation.hpp"

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace RanOS
{
    Frame::Frame(Reset_f reset, Update_f update, uint16_t size, float duration) :
        m_Reset(reset), m_Update(update), m_Colors(), m_Size(size), m_FrameDuration(duration)
    {
    }
}

namespace RanOS
{
    Animation::Animation(Frame::Reset_f reset, Frame::Update_f update, uint16_t num_panels,
                         float duration, Controller * controller) :
        m_Controller(controller),
        m_Frame(reset, update, num_panels*PANEL_SIZE, duration),
        m_Time(0)
    {
        controller->m_Buffer = m_Frame.m_Colors;

        Reset();
    }

    void Animation::Update(float dt)
    {
        m_Time += dt;

        while(m_Time >= m_Frame.m_FrameDuration)
        {
            AdvanceFrame();
        }
    }

    void Animation::Reset()
    {
        m_Frame.Reset();
        m_Frame.Update();

        m_Time = 0;

        m_Controller->Display();
    }
}   // namespace RanOS

// Private Functions                      //////////////////////////////////////

namespace RanOS
{
    void Animation::AdvanceFrame()
    {
        m_Time -= m_Frame.m_FrameDuration;

        m_Frame.Update();

        m_Controller->Display();
    }
}   // namespace RanOS
