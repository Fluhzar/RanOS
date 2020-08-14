#include <Arduino.h>

#include "APA102CDraw.hpp"

#define NUM_BUSY_LOOPS 16

namespace RanOS
{
    APA102CDraw::APA102CDraw(Pin d, Pin c, Option<Duration> target_dt) :
        data(d),
        clock(c),
        queue(),
        timer(target_dt),
        known_len(0),
        stats()
    {
        pinMode(self.data, OUTPUT);
        pinMode(self.clock, OUTPUT);
    }

    APA102CDraw::~APA102CDraw() {
        self.stop(known_len);
    }

    void APA102CDraw::push_queue(Rc<Animation> ani) {
        self.queue.push_back(ani);
    }

    usize APA102CDraw::queue_len() const {
        return self.queue.size();
    }

    DrawStats APA102CDraw::get_stats() const {
        return self.stats;
    }

    void APA102CDraw::run() {
        self.timer.reset();
        self.stats.reset();

        auto zero_duration = Duration();

        while(self.queue.size() > 0) {
            auto ani = self.queue.front();
            self.queue.pop_front();

            if(ani->frame().get_leds().size() > self.known_len) {
                self.known_len = ani->frame().get_leds().size();
            }

            while(ani->time_remaining() > zero_duration) {
                ani->update(self.timer.ping());

                for (usize i = 0; i < NUM_BUSY_LOOPS; ++i) {
                    Frame const &interp = ani->frame();

                    self.write_frame(interp);
                    self.stats.inc_frames();
                }
            }
        }

        self.stats.end();
    }

    void APA102CDraw::set_pins_low() {
        digitalWrite(self.data, LOW);
        digitalWrite(self.clock, LOW);
    }

    void APA102CDraw::start_frame() {
        self.set_pins_low();

        self.write_byte(0x00);
        self.write_byte(0x00);
        self.write_byte(0x00);
        self.write_byte(0x00);
    }

    void APA102CDraw::end_frame(usize len) {
        for (usize i = 0; i < (len>>4); ++i) {
            self.write_byte(0x00);
        }
    }

    void APA102CDraw::stop(usize len) {
        self.start_frame();

        for (usize i = 0; i < len; ++i) {
            self.write_byte(0xE0);
            self.write_byte(0x00);
            self.write_byte(0x00);
            self.write_byte(0x00);
        }

        self.end_frame(len);
    }

    void APA102CDraw::write_byte(u8 byte) {
        digitalWriteFast(self.data, byte >> 7 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);

        digitalWriteFast(self.data, byte >> 6 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);

        digitalWriteFast(self.data, byte >> 5 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);

        digitalWriteFast(self.data, byte >> 4 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);

        digitalWriteFast(self.data, byte >> 3 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);
        
        digitalWriteFast(self.data, byte >> 2 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);

        digitalWriteFast(self.data, byte >> 1 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);

        digitalWriteFast(self.data, byte >> 0 & 1);
        digitalToggleFast(self.clock);
        digitalToggleFast(self.clock);
    }

    void APA102CDraw::write_frame(Frame const & frame) {
        self.start_frame();

        for (auto led : frame.get_leds()) {
            self.write_byte(0xE0 | frame.get_brightness_APA102C());
            auto color = led.into_tuple(O_BGR);
            self.write_byte(std::get<0>(color));
            self.write_byte(std::get<1>(color));
            self.write_byte(std::get<2>(color));
        }

        self.end_frame(frame.get_leds().size());
    }
}
