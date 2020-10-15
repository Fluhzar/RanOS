#include "Breath.hpp"

namespace RanOS
{
    Breath::Breath(Duration duration, Duration breath_duration, f32 brightness, usize size, Option<Vec<RGB>> order):
        b_remaining(duration),
        b_frame(brightness, size),
        b_order(order),
        b_ind(0),
        b_current_color(),
        b_brightness(0.0),
        b_acc(-8.0 / powf(breath_duration, 2.0)),
        b_vel(4.0 / breath_duration),
        b_vel0(4.0 / breath_duration)
    {
        if(b_order.is_some() && b_order.unwrap().size() > 0) {
            self.b_current_color = b_order.unwrap()[self.b_ind];
        } else {
            self.b_current_color = RGB::random();
        }
    }

    void Breath::update(Duration dt) {
        self.b_remaining -= dt;
        if(self.b_remaining < 0) {
            self.b_remaining = Duration(0);
        }

        self.b_vel += self.b_acc * dt;
        self.b_brightness += self.b_vel * dt;

        if(self.b_brightness <= 0.0 && self.b_vel < 0.0) {
            self.b_brightness = 0.0;
            self.b_vel = self.b_vel0;

            if(self.b_order.is_some()) {
                self.b_ind += 1;
                self.b_ind %= self.b_order.unwrap().size();
                self.b_current_color = self.b_order.unwrap()[self.b_ind];
            } else {
                self.b_current_color = RGB::random();
            }
        }

        for(auto & led : self.b_frame.get_leds()) {
            led = self.b_current_color.scale(self.b_brightness);
        }
    }

    Frame const & Breath::frame() const {
        return self.b_frame;
    }

    Duration Breath::time_remaining() const {
        return self.b_remaining;
    }
}
