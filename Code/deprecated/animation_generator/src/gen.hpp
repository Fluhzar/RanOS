/*! ****************************************************************************
\file             gen.hpp
\author           Fluhzar
\par    Email:    fluhzar\@gmail.com
\par    Project:  AnimationGenerator

\copyright        Copyright © 2018 Fluhzar
*******************************************************************************/

#ifndef __GEN_HPP
#define __GEN_HPP

// Include Files                ////////////////////////////////////////////////

#include <functional>
#include <string>
#include <tuple>
#include <vector>

#include "Animation.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace RanOS
{

  /*! **************************************************************************
  \brief
  *****************************************************************************/
  struct GeneratorOptions
  {
  public:

    // Members              ///////////////////////

    std::function<Frame()> m_CallBack;
    uint16_t m_SizeMask;

    // Con-/De- structors   ///////////////////////

    inline GeneratorOptions(std::function<Frame()> const & callback, uint16_t count, bool looping) :
      m_CallBack(callback), m_SizeMask((looping ? count | (1 << 15): count))
    { if(count > 0x7FFF) throw "Count cannot be larger than largest positive 16-bit signed value"; };

    inline GeneratorOptions(GeneratorOptions const &) = default;
    inline GeneratorOptions(GeneratorOptions &&) noexcept = default;

    // Operators            ///////////////////////

    GeneratorOptions & operator=(GeneratorOptions const &) = default;
    GeneratorOptions & operator=(GeneratorOptions &&) noexcept = default;

    // Accossors/Mutators   ///////////////////////

    // Functions            ///////////////////////

  private:

    // Functions                  ///////////////////////

  };  // struct GeneratorOptions

} // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

namespace RanOS
{

  std::tuple<std::string,std::vector<uint8_t>> GenerateAnimationData(GeneratorOptions const & options, uint16_t index);

} // namespace RanOS

#endif // __GEN_HPP
