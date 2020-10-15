/*! ****************************************************************************
\file             Animation.hpp
\author           Fluhzar
\par    Email:    fluhzar\@gmail.com
\par    Project:  AnimationGenerator

\copyright        Copyright © 2018 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __ANIMATION_HPP
#define __ANIMATION_HPP

// Include Files                ////////////////////////////////////////////////

#include <cstdint>

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace RanOS
{

  struct RGB;

  /*! **************************************************************************
  \brief
  *****************************************************************************/
  struct Frame
  {
  public:

    // Members              ///////////////////////

    RGB * m_Frame;
    uint16_t m_Size;

    float m_FrameDuration;

    // Con-/De- structors   ///////////////////////

    inline Frame() = default;

    inline Frame(RGB * frame, uint16_t size, float duration) :
      m_Frame(frame), m_Size(size), m_FrameDuration(duration) {};

    inline Frame(Frame const &) = default;
    inline Frame(Frame &&) noexcept = default;

    inline ~Frame() = default;

    // Operators            ///////////////////////

    inline Frame & operator=(Frame const &) = default;
    inline Frame & operator=(Frame &&) noexcept = default;

  }; // struct Frame

} // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __ANIMATION_HPP
