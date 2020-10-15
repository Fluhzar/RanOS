#ifndef __STROBE_HPP
#define __STROBE_HPP

#include <cstdint>

#include "../Types.hpp"
#include "Animation.hpp"

namespace RanOS
{
    class Strobe: public Animation 
    {
    private:

        // Members              ///////////////////////

        Duration s_remaining;
        Frame s_frame;

        RGB s_color;

        Duration s_period;
        f32 s_duty;

        Duration s_t;

    public:

        // Con-/De- structors   ///////////////////////

        Strobe(Duration duration, f32 brightness, usize size, RGB color, Duration period, f32 duty);
        virtual ~Strobe() = default;

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

#endif // __STROBE_HPP
