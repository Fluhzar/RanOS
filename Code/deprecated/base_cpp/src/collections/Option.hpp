#ifndef __OPTION_HPP
#define __OPTION_HPP

#include <Arduino.h>

#include <cstdint>

#include "../Types.hpp"

namespace RanOS
{
    template<typename T>
    class Option
    {
    private:

        // Members              ///////////////////////

        T * val;

    public:

        static Option<T> Some(T t) {
            return Option(new T(t));
        };
        static Option<T> None() {
            return Option(null);
        };

        // Con-/De- structors   ///////////////////////

        Option(Option const & rhs) {
            if(rhs.val != null) {
                self.val = new T(*rhs.val);
            } else {
                self.val = null;
            }
        };
        ~Option() {
            if(self.val) {
                delete self.val;
            }
        };

        // Operators            ///////////////////////

        // Accessors/Mutators   ///////////////////////

        bool is_some() { return self.val != null; };
        bool is_none() { return self.val == null; };

        T & unwrap() {
            if(self.val == null) {
                exit(-1);
            }

            return *self.val;
        }

        T const & unwrap() const {
            if(self.val == null) {
                exit(-1);
            }

            return *self.val;
        }

        // Functions            ///////////////////////

    private:

        // Functions                  ///////////////////////

        Option(T *t) : val(t){};

    };
}

#endif // __OPTION_HPP
