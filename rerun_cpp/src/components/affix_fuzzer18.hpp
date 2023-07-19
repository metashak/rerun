// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs"

#pragma once

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

#include "../datatypes/affix_fuzzer4.hpp"

namespace rr {
    namespace components {
        struct AffixFuzzer18 {
            std::optional<std::vector<rr::datatypes::AffixFuzzer4>> many_optional_unions;

            AffixFuzzer18(
                std::optional<std::vector<rr::datatypes::AffixFuzzer4>> many_optional_unions)
                : many_optional_unions(std::move(many_optional_unions)) {}
        };
    } // namespace components
} // namespace rr
