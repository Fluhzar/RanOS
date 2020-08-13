#include "Timer.hpp"

namespace RanOS
{
    Timer::Timer(Option<Duration> target):
        ctime(TIME_NOW()),
        ptime(TIME_NOW()),
        dt(),
        target_dt(target)
    {
    }

    Duration Timer::ping() {
        self.ptime = self.ctime;

        if(self.target_dt.is_some()) {
            auto t_dt = self.target_dt.unwrap();

            while((self.ctime - self.ptime) < t_dt) {
                self.ctime = TIME_NOW();
            }
        } else {
            self.ctime = TIME_NOW();
        }

        self.dt = self.ctime - self.ptime;
        return self.dt;
    }
}
