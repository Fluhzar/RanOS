#ifndef __RGB_HPP
#define __RGB_HPP

#include <tuple>

#include "../Types.hpp"

namespace RanOS
{
    enum RGBOrder
    {
        O_RGB,
        O_RBG,
        O_GRB,
        O_GBR,
        O_BRG,
        O_BGR,
    };

    class RGB
    {
    private:

        // Members              ///////////////////////
        u8 red;
        u8 green;
        u8 blue;

    public:

        // Con-/De- structors   ///////////////////////

        RGB() = default;
        RGB(RGB const &) = default;
        ~RGB() = default;

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        u8 get_red() const;
        u8 get_green() const;
        u8 get_blue() const;

        void set_red(u8);
        void set_green(u8);
        void set_blue(u8);

        // Functions            ///////////////////////

        static RGB from_code(u32 x, RGBOrder o);
        static RGB from_tuple(std::tuple<u8, u8, u8> x, RGBOrder o);
        static RGB random();
        static RGB from_hsv(f32 h, f32 s, f32 v);

        std::tuple<f32, f32, f32> into_hsv() const;
        RGB scale(f32 s) const;
        std::tuple<u8, u8, u8> into_tuple(RGBOrder o);

    private:

        // Functions                  ///////////////////////

    };
}

#endif // __RGB_HPP
