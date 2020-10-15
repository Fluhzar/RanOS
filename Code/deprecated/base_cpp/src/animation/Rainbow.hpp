#ifndef __RAINBOW_HPP
#define __RAINBOW_HPP

#include <cstdint>

#include "../Types.hpp"
#include "Animation.hpp"

namespace RanOS
{
    class Rainbow: public Animation
    {
    private:

        // Members              ///////////////////////

        Duration r_remaining;
        Frame r_frame;
        f32 r_hue;
        f32 r_sat;
        f32 r_val;
        f32 r_dh;
        f32 r_arc;
        usize r_step;

    public:

        // Con-/De- structors   ///////////////////////

        Rainbow(Duration duration, Duration rainbow_length, f32 brightness, usize size, f32 saturation, f32 value, f32 arc, usize step);
        Rainbow(Rainbow const &) = default;
        virtual ~Rainbow() = default;

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        // Functions            ///////////////////////

        virtual void update(Duration dt);
        virtual Frame const & frame() const;
        virtual Duration time_remaining() const;

    private:

        // Functions                  ///////////////////////

    };
}

#endif // __RAINBOW_HPP
