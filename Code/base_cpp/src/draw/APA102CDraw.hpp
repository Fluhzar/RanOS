#ifndef __APA102CDRAW_HPP
#define __APA102CDRAW_HPP

#include <cstdint>
#include <queue>

#include "../Types.hpp"
#include "../animation/Animation.hpp"
#include "../util/Timer.hpp"
#include "DrawStats.hpp"

namespace RanOS
{
    class APA102CDraw
    {
    private:

        // Members              ///////////////////////

        Pin data;
        Pin clock;

        Queue<Animation *> queue;
        Timer timer;

        usize known_len;

        DrawStats stats;

    public:

        // Con-/De- structors   ///////////////////////

        APA102CDraw(Pin data, Pin clock);
        ~APA102CDraw();

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        void push_queue(Animation * ani);
        usize queue_len() const;

        DrawStats get_stats() const;

        // Functions            ///////////////////////

        void run();
        void stop(usize len);

    private:

        // Functions                  ///////////////////////

        void set_pins_low();

        void start_frame();
        void end_frame(usize len);

        void write_byte(u8 byte);
        void write_frame(Frame const & frame);
    };
}

#endif // __APA102CDRAW_HPP
