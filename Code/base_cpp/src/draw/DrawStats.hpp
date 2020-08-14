#ifndef __DRAWSTATS_HPP
#define __DRAWSTATS_HPP

#include <cstdint>
#include <chrono>
#include <ostream>

#include "../Types.hpp"

namespace RanOS
{
    class DrawStats
    {
    private:

        // Members              ///////////////////////

        Instant ds_start;
        Instant ds_end;
        usize ds_frames;

    public:

        // Con-/De- structors   ///////////////////////

        DrawStats();
        DrawStats(DrawStats const &) = default;
        ~DrawStats() = default;

        // Operators            ///////////////////////

        DrawStats & operator=(DrawStats const &) = default;

        // Accessors/Mutators   ///////////////////////

        // Functions            ///////////////////////

        void inc_frames();
        void end();
        String to_string() const;
        void reset();

    private:

        // Functions                  ///////////////////////

    };
}

#endif // __DRAWSTATS_HPP
