#include "Rainbow.hpp"

namespace RanOS
{
    Rainbow::Rainbow(Duration duration, Duration rainbow_length, f32 brightness, usize size, f32 saturation, f32 value, f32 arc, usize step):
        r_remaining(duration),
        r_frame(brightness, size),
        r_hue(0),
        r_sat(saturation),
        r_val(value),
        r_dh(360.0 / rainbow_length),
        r_arc(arc),
        r_step(step)
    {}

    void Rainbow::update(Duration dt) {
        self.r_remaining -= dt;
        if(self.r_remaining < 0) {
            self.r_remaining = 0;
        }

        self.r_hue += fmodf(self.r_dh * dt, 360.0);

        auto & frame = self.r_frame.get_leds();
        auto len = f32(frame.size());
        // auto brightness = self.r_frame.get_brightness();
        for (usize i = 0; i < len; ++i) {
            auto step = floorf(f32(i) / f32(self.r_step)) * f32(self.r_step) / len * 360.0 * self.r_arc;
            frame[i] = RGB::from_hsv(self.r_hue + step, self.r_sat, self.r_val)/*.scale(brightness)*/;
        }
    }

    Frame const & Rainbow::frame() const {
        return self.r_frame;
    }

    Duration Rainbow::time_remaining() const {
        return self.r_remaining;
    }
}
