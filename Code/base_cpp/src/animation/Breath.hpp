#ifndef __BREATH_HPP
#define __BREATH_HPP

#include <cstdint>

#include "../Types.hpp"
#include "Animation.hpp"
#include "../util/Frame.hpp"
#include "../util/RGB.hpp"

namespace RanOS
{
    class Breath: public Animation
    {
    private:

        // Members              ///////////////////////

        Duration b_remaining;
        Frame b_frame;

        Option<Vec<RGB>> b_order;
        usize b_ind;
        RGB b_current_color;

        f32 b_brightness;

        f32 b_acc;
        f32 b_vel;
        f32 b_vel0;

    public:

        // Con-/De- structors   ///////////////////////

        Breath(Duration duration, Duration breath_duration, f32 brightness, usize size, Option<Vec<RGB>> order);

        virtual ~Breath() = default;

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

#endif // __BREATH_HPP
