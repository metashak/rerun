// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/quaternion.fbs"

#pragma once

#include <cstdint>

namespace rr {
    namespace datatypes {
        /// A Quaternion represented by 4 real numbers.
        struct Quaternion {
            float xyzw[4];
        };
    } // namespace datatypes
} // namespace rr
