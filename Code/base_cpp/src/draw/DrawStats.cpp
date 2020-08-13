#include <sstream>

#include "DrawStats.hpp"

namespace RanOS
{
    DrawStats::DrawStats(): ds_start(TIME_NOW()), ds_end(TIME_NOW()), ds_frames(0) {}

    void DrawStats::inc_frames() {
        self.ds_frames += 1;
    }

    void DrawStats::end() {
        self.ds_end = TIME_NOW();
    }

    String DrawStats::to_string() const {
        auto duration = self.ds_end - self.ds_start;

        std::stringstream ss;
        ss << "Drawing statistics: \n"
           << duration
           << "s \tFrame count: "
           << self.ds_frames
           << " \nAvg updates per second: "
           << (self.ds_frames / duration)
           << " UPS\n";

        return ss.str();
    }
} // namespace RanOS
