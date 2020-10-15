#ifndef __TIMER_HPP
#define __TIMER_HPP

#include <cstdint>

#include "../Types.hpp"

namespace RanOS
{
    class Timer
    {
    private:

        // Members              ///////////////////////

        Instant ctime;
        Instant ptime;
        Duration dt;
        Option<Duration> target_dt;

    public:

        static void Wait(Duration t);

        // Con-/De- structors   ///////////////////////

        Timer(Option<Duration> target);

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        // Functions            ///////////////////////

        Duration ping();
        void reset();

    private:

        // Functions                  ///////////////////////

    };
}

#endif // __TIMER_HPP
