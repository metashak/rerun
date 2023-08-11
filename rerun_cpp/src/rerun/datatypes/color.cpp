// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/color.fbs"

#include "color.hpp"

#include <arrow/api.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& Color::to_arrow_datatype() {
            static const auto datatype = arrow::uint32();
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::UInt32Builder>> Color::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(std::make_shared<arrow::UInt32Builder>(memory_pool));
        }

        arrow::Status Color::fill_arrow_array_builder(
            arrow::UInt32Builder* builder, const Color* elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            static_assert(sizeof(*elements) == sizeof(elements->rgba));
            ARROW_RETURN_NOT_OK(builder->AppendValues(&elements->rgba, num_elements));

            return arrow::Status::OK();
        }
    } // namespace datatypes
} // namespace rerun
