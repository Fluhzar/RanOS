/*! ****************************************************************************
\file             RGB.hpp
\author           Fluhzar
\par    Email:    fluhzar\@gmail.com
\par    Project:  RanOS

\copyright        Copyright © 2018 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __RGB_HPP
#define __RGB_HPP

// Include Files                ////////////////////////////////////////////////

#include <cstdint>

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

namespace RanOS
{

// Forward Namespace References ////////////////////////////////////////////////

// Public Namespace Enums       ////////////////////////////////////////////////

// Public Namespace Objects     ////////////////////////////////////////////////

struct RGB
{

  // Members              ///////////////////////

  union
  {
    uint8_t r;
    uint8_t red;
  };
  union
  {
    uint8_t g;
    uint8_t green;
  };
  union
  {
    uint8_t b;
    uint8_t blue;
  };

  // Con-/De- structors   ///////////////////////

  inline RGB(uint8_t r = 0x0, uint8_t g = 0x0, uint8_t b = 0x0) :
    red(r), green(g), blue(b) {};

  // Operators            ///////////////////////

  inline uint8_t & operator[](uint8_t index)
  {
    switch(index)
    {
      case 2:          return b;
      case 1:          return g;
      case 0: default: return r;
    };
  };

  inline uint8_t operator[](uint8_t index) const
  {
    switch(index)
    {
      case 2:          return b;
      case 1:          return g;
      case 0: default: return r;
    };
  };

}; // RGB

// Public Namespace Functions   ////////////////////////////////////////////////

} // namespace RanOS

#endif // __RGB_HPP
