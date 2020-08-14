#include <cmath>
#include <cstdlib>
#include <algorithm>

#include "RGB.hpp"

namespace RanOS
{
    u8 RGB::get_red() const {
        return self.red;
    }
    u8 RGB::get_green() const {
        return self.green;
    }
    u8 RGB::get_blue() const {
        return self.blue;
    }

    void RGB::set_red(u8 c) {
        self.red = c;
    }
    void RGB::set_green(u8 c) {
        self.green = c;
    }
    void RGB::set_blue(u8 c) {
        self.blue = c;
    }

    RGB RGB::from_code(u32 x, RGBOrder o) {
        RGB out;

        switch (o)
        {
        case O_RGB:
            out.red   = u8((x & 0x00FF0000) >> 16);
            out.green = u8((x & 0x0000FF00) >> 8);
            out.blue  = u8((x & 0x000000FF) >> 0);
            break;
        case O_RBG:
            out.red   = u8((x & 0x00FF0000) >> 16);
            out.blue  = u8((x & 0x0000FF00) >> 8);
            out.green = u8((x & 0x000000FF) >> 0);
            break;
        case O_GRB:
            out.green = u8((x & 0x00FF0000) >> 16);
            out.red   = u8((x & 0x0000FF00) >> 8);
            out.blue  = u8((x & 0x000000FF) >> 0);
            break;
        case O_GBR:
            out.green = u8((x & 0x00FF0000) >> 16);
            out.blue  = u8((x & 0x0000FF00) >> 8);
            out.red   = u8((x & 0x000000FF) >> 0);
            break;
        case O_BRG:
            out.blue  = u8((x & 0x00FF0000) >> 16);
            out.red   = u8((x & 0x0000FF00) >> 8);
            out.green = u8((x & 0x000000FF) >> 0);
            break;
        case O_BGR:
            out.blue  = u8((x & 0x00FF0000) >> 16);
            out.green = u8((x & 0x0000FF00) >> 8);
            out.red   = u8((x & 0x000000FF) >> 0);
            break;
        }

        return out;
    }

    RGB RGB::from_tuple(std::tuple<u8,u8,u8> x, RGBOrder o) {
        return RGB::from_code((u32(std::get<0>(x)) << 16) | (u32(std::get<1>(x)) << 8) | (u32(std::get<2>(x)) << 0), o);
    }

    RGB RGB::random() {
        RGB out;

        out.red = u8(std::rand() % 256);
        out.green = u8(std::rand() % 256);
        out.blue = u8(std::rand() % 256);

        return out;
    }

    RGB RGB::from_hsv(f32 h, f32 s, f32 v) {
        h = fmodf(h, 360.0);

        auto c = v * s;
        auto x = c * (1.0 - abs(fmodf(h / 60.0, 2.0) - 1.0));
        auto m = v - c;

        f32 r, g, b;

        if (h >= 0.0 && h < 60.0) {
            r = c;
            g = x;
            b = 0.0;
        } else if (h >= 60.0 && h < 120.0) {
            r = x;
            g = c;
            b = 0.0;
        } else if (h >= 120.0 && h < 180.0) {
            r = 0.0;
            g = c;
            b = x;
        } else if (h >= 180.0 && h < 240.0) {
            r = 0.0;
            g = x;
            b = c;
        } else if (h >= 240.0 && h < 300.0) {
            r = x;
            g = 0.0;
            b = c;
        } else {
            r = c;
            g = 0.0;
            b = x;
        }

        RGB out;

        out.red = u8((r + m) * 255.0);
        out.green = u8((g + m) * 255.0);
        out.blue = u8((b + m) * 255.0);

        return out;
    }

    std::tuple<f32, f32, f32> RGB::into_hsv() const {
        auto r = f32(self.red) / 255.0;
        auto g = f32(self.green) / 255.0;
        auto b = f32(self.blue) / 255.0;

        auto cmax = std::max(r, std::max(g, b));
        auto cmin = std::min(r, std::min(g, b));

        auto delta = cmax - cmin;

        auto h = 0.0;
        if(delta == 0.0) {
            h = 0.0;
        } else if(cmax == r) {
            h = 60.0 * fmodf((g - b) / delta, 6.0);
        } else if(cmax == g) {
            h = 60.0 * (((b - r) / delta) + 2.0);
        } else {
            h = 60.0 * (((r - g) / delta) + 4.0);
        }

        auto s = 0.0;
        if(cmax == 0.0) {
            s = 0.0;
        } else {
            s = delta / cmax;
        }

        auto v = cmax;

        return std::make_tuple(h, s, v);
    }

    RGB RGB::scale(f32 s) const {
        RGB out;

        out.red = u8(std::max(std::min(f32(self.red) * s, f32(255.0)), f32(0.0)));
        out.green = u8(std::max(std::min(f32(self.green) * s, f32(255.0)), f32(0.0)));
        out.blue = u8(std::max(std::min(f32(self.blue) * s, f32(255.0)), f32(0.0)));

        return out;
    }

    std::tuple<u8, u8, u8> RGB::into_tuple(RGBOrder o) {
        switch (o)
        {
        case O_RGB:
            return std::make_tuple(self.red, self.green, self.blue);
            break;
        case O_RBG:
            return std::make_tuple(self.red, self.blue, self.green);
            break;
        case O_GRB:
            return std::make_tuple(self.green, self.red, self.blue);
            break;
        case O_GBR:
            return std::make_tuple(self.green, self.blue, self.red);
            break;
        case O_BRG:
            return std::make_tuple(self.blue, self.red, self.green);
            break;
        case O_BGR:
            return std::make_tuple(self.blue, self.green, self.red);
            break;
        };

        return std::make_tuple(0, 0, 0);
    }
}
