#ifndef __FRAME_HPP
#define __FRAME_HPP

#include <cstdint>

#include "../Types.hpp"
#include "RGB.hpp"

namespace RanOS
{
    class Frame
    {
    private:

        // Members              ///////////////////////

        f32 brightness;
        Vec<RGB> leds;

    public:

        // Con-/De- structors   ///////////////////////

        Frame(f32 b, usize s);

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        f32 get_brightness() const;
        u8 get_brightness_APA102C() const;
        u8 get_brightness_SK9822() const;

        Vec<RGB> const & get_leds() const;
        Vec<RGB> & get_leds();

        // Functions            ///////////////////////

    private:

        // Functions                  ///////////////////////

    };
}

#endif // __FRAME_HPP
