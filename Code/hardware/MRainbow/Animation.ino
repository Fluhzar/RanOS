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
    Frame::Frame(uint16_t size, float duration) :
        m_Frame(), m_Size(size), m_FrameDuration(duration)
    {
    }
}

namespace RanOS
{
    Animation::Animation(FrameCallback_f sequencer, Frame const * frame, uint16_t num_panels, uint8_t data, uint8_t clock) :
        m_Controller(data, clock, frame->m_Frame, num_panels),
        m_Sequencer(sequencer), m_Frame(frame),
        m_Time(0)
    {
        Play();
    }

    void Animation::Update(float dt)
    {
        m_Time += dt;

        while(m_Time >= m_Frame->m_FrameDuration)
        {
            AdvanceFrame();
        }
    }
}   // namespace RanOS

// Private Functions                      //////////////////////////////////////

namespace RanOS
{
    void Animation::Play()
    {
        m_Time = 0;

        m_Sequencer();

        m_Controller.m_Buffer = m_Frame->m_Frame;
        m_Controller.m_BufSize = m_Frame->m_Size;
        m_Controller.Display();
    }

    void Animation::AdvanceFrame()
    {
        m_Time -= m_Frame->m_FrameDuration;

        m_Sequencer();

        m_Controller.Display();
    }
}   // namespace RanOS
