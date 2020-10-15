#include <Arduino.h>
#include <sstream>

#include "Strobe.hpp"

namespace RanOS
{
    Strobe::Strobe(Duration duration, f32 brightness, usize size, RGB color, Duration period, f32 duty):
        s_remaining(duration),
        s_frame(brightness, size),
        s_color(color),
        s_period(period),
        s_duty(duty),
        s_t(0)
    {
    }

    void Strobe::update(Duration dt) {
        self.s_remaining -= dt;
        if(self.s_remaining < 0) {
            self.s_remaining = Duration(0);
        }

        self.s_t = fmodf(self.s_t + dt, self.s_period);

        auto r = self.s_t / self.s_period;
        auto t = r - floorf(r);

        auto color = (t < self.s_duty) ? self.s_color : RGB();
        for(auto & led : self.s_frame.get_leds()) {
            led = color;
        }
    }

    Frame const & Strobe::frame() const {
        return self.s_frame;
    }

    Duration Strobe::time_remaining() const {
        return self.s_remaining;
    }
}
