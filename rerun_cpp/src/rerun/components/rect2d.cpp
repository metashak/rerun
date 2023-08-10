// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/rect2d.fbs"

#include "rect2d.hpp"

#include "../arrow.hpp"
#include "../datatypes/vec4d.hpp"

#include <arrow/api.h>

namespace rerun {
    namespace components {
        const char *Rect2D::NAME = "rerun.rect2d";

        const std::shared_ptr<arrow::DataType> &Rect2D::to_arrow_datatype() {
            static const auto datatype = rerun::datatypes::Vec4D::to_arrow_datatype();
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::FixedSizeListBuilder>> Rect2D::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(
                rerun::datatypes::Vec4D::new_arrow_array_builder(memory_pool).ValueOrDie()
            );
        }

        arrow::Status Rect2D::fill_arrow_array_builder(
            arrow::FixedSizeListBuilder *builder, const Rect2D *elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            static_assert(sizeof(rerun::datatypes::Vec4D) == sizeof(Rect2D));
            ARROW_RETURN_NOT_OK(rerun::datatypes::Vec4D::fill_arrow_array_builder(
                builder,
                reinterpret_cast<const rerun::datatypes::Vec4D *>(elements),
                num_elements
            ));

            return arrow::Status::OK();
        }

        arrow::Result<rerun::DataCell> Rect2D::to_data_cell(
            const Rect2D *instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool *pool = arrow::default_memory_pool();

            ARROW_ASSIGN_OR_RAISE(auto builder, Rect2D::new_arrow_array_builder(pool));
            if (instances && num_instances > 0) {
                ARROW_RETURN_NOT_OK(
                    Rect2D::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema =
                arrow::schema({arrow::field(Rect2D::NAME, Rect2D::to_arrow_datatype(), false)});

            rerun::DataCell cell;
            cell.component_name = Rect2D::NAME;
            ARROW_ASSIGN_OR_RAISE(
                cell.buffer,
                rerun::ipc_from_table(*arrow::Table::Make(schema, {array}))
            );

            return cell;
        }
    } // namespace components
} // namespace rerun
