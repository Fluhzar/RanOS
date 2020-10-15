#ifndef __ANIMATION_HPP
#define __ANIMATION_HPP

#include <cstdint>

#include "../Types.hpp"
#include "../util/Frame.hpp"

namespace RanOS
{
    class Animation
    {
    private:

        // Members              ///////////////////////

    public:

        // Con-/De- structors   ///////////////////////

        virtual ~Animation() = default;

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        // Functions            ///////////////////////

        virtual void update(Duration dt) = 0;
        virtual Frame const & frame() const = 0;
        virtual Duration time_remaining() const = 0;

    private:

        // Functions                  ///////////////////////

    };
}

#endif // __ANIMATION_HPP
