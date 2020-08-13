#include "Frame.hpp"

namespace RanOS
{
    Frame::Frame(f32 b, usize s):
        brightness(b),
        leds(s, RGB())
    {
    }

    f32 Frame::get_brightness() const {
        return self.brightness;
    }

    u8 Frame::get_brightness_APA102C() const {
        return u8(std::max(f32(0.0), std::min(f32(1.0), self.brightness)) * f32(0x1F));
    }

    u8 Frame::get_brightness_SK9822() const {
        return self.get_brightness_APA102C();
    }

    Vec<RGB> const & Frame::get_leds() const {
        return self.leds;
    }

    Vec<RGB> & Frame::get_leds() {
        return self.leds;
    }
}
