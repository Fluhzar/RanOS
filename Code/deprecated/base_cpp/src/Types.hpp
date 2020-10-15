#ifndef __TYPES_HPP
#define __TYPES_HPP

#include <Arduino.h>

#include <cstdint>
#include <deque>
#include <memory>
#include <string>
#include <vector>

#define self (*this)
#define null nullptr

#define POP_FIRST(x, ...) x
#define POP_REST(x, ...) __VA_ARGS__

#define TIME_NOW() ::RanOS::f32(micros()) / ::RanOS::f32(1'000'000)
#define NEW_BOX(T, ...) std::make_unique<T>(__VA_ARGS__)
#define NEW_RC(T, ...) std::make_shared<T>(__VA_ARGS__)

#define UNREFERENCED_PARAMATER(x) (void)(x)

#include "collections/Option.hpp"

namespace RanOS
{
    using i8 = int_fast8_t;
    using i16 = int_fast16_t;
    using i32 = int_fast32_t;
    using i64 = int_fast64_t;
    using isize = i64;

    using u8 = uint_fast8_t;
    using u16 = uint_fast16_t;
    using u32 = uint_fast32_t;
    using u64 = uint_fast64_t;
    using usize = u64;

    using f32 = float;
    using f64 = double;

    using Pin = int;

    template <typename T>
    using Box = std::unique_ptr<T>;

    template <typename T>
    using Rc = std::shared_ptr<T>;

    template <typename T>
    using Queue = std::deque<T>;

    template <typename T>
    using Vec = std::vector<T>;

    using String = std::string;

    using Duration = f32;
    using Instant = f32;
} // namespace RanOS

#endif // __TYPES_HPP
