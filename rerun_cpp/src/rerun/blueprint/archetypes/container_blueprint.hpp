// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/container_blueprint.fbs".

#pragma once

#include "../../blueprint/components/active_tab.hpp"
#include "../../blueprint/components/column_shares.hpp"
#include "../../blueprint/components/container_kind.hpp"
#include "../../blueprint/components/grid_columns.hpp"
#include "../../blueprint/components/included_contents.hpp"
#include "../../blueprint/components/name.hpp"
#include "../../blueprint/components/row_shares.hpp"
#include "../../blueprint/components/visible.hpp"
#include "../../collection.hpp"
#include "../../compiler_utils.hpp"
#include "../../data_cell.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: The top-level description of the Viewport.
    struct ContainerBlueprint {
        /// The class of the view.
        rerun::blueprint::components::ContainerKind container_kind;

        /// The name of the container.
        std::optional<rerun::blueprint::components::Name> display_name;

        /// `ContainerIds`s or `SpaceViewId`s that are children of this container.
        std::optional<rerun::blueprint::components::IncludedContents> contents;

        /// The layout shares of each column in the container.
        ///
        /// For `Horizontal` containers, the length of this list should always match the number of contents.
        ///
        /// Ignored for `Horizontal` containers.
        std::optional<rerun::blueprint::components::ColumnShares> col_shares;

        /// The layout shares of each row of the container.
        ///
        /// For `Horizontal` containers, the length of this list should always match the number of contents.
        ///
        /// Ignored for `Vertical` containers.
        std::optional<rerun::blueprint::components::RowShares> row_shares;

        /// Which tab is active.
        ///
        /// Only applies to `Tabs` containers.
        std::optional<rerun::blueprint::components::ActiveTab> active_tab;

        /// Whether this container is visible.
        ///
        /// Defaults to true if not specified.
        std::optional<rerun::blueprint::components::Visible> visible;

        /// How many columns this grid should have.
        ///
        /// If unset, the grid layout will be auto.
        ///
        /// Ignored for `Horizontal`/`Vertical` containers.
        std::optional<rerun::blueprint::components::GridColumns> grid_columns;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.ContainerBlueprintIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        ContainerBlueprint() = default;
        ContainerBlueprint(ContainerBlueprint&& other) = default;

        explicit ContainerBlueprint(rerun::blueprint::components::ContainerKind _container_kind)
            : container_kind(std::move(_container_kind)) {}

        /// The name of the container.
        ContainerBlueprint with_display_name(rerun::blueprint::components::Name _display_name) && {
            display_name = std::move(_display_name);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// `ContainerIds`s or `SpaceViewId`s that are children of this container.
        ContainerBlueprint with_contents(rerun::blueprint::components::IncludedContents _contents
        ) && {
            contents = std::move(_contents);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// The layout shares of each column in the container.
        ///
        /// For `Horizontal` containers, the length of this list should always match the number of contents.
        ///
        /// Ignored for `Horizontal` containers.
        ContainerBlueprint with_col_shares(rerun::blueprint::components::ColumnShares _col_shares
        ) && {
            col_shares = std::move(_col_shares);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// The layout shares of each row of the container.
        ///
        /// For `Horizontal` containers, the length of this list should always match the number of contents.
        ///
        /// Ignored for `Vertical` containers.
        ContainerBlueprint with_row_shares(rerun::blueprint::components::RowShares _row_shares) && {
            row_shares = std::move(_row_shares);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Which tab is active.
        ///
        /// Only applies to `Tabs` containers.
        ContainerBlueprint with_active_tab(rerun::blueprint::components::ActiveTab _active_tab) && {
            active_tab = std::move(_active_tab);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Whether this container is visible.
        ///
        /// Defaults to true if not specified.
        ContainerBlueprint with_visible(rerun::blueprint::components::Visible _visible) && {
            visible = std::move(_visible);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// How many columns this grid should have.
        ///
        /// If unset, the grid layout will be auto.
        ///
        /// Ignored for `Horizontal`/`Vertical` containers.
        ContainerBlueprint with_grid_columns(rerun::blueprint::components::GridColumns _grid_columns
        ) && {
            grid_columns = std::move(_grid_columns);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Returns the number of primary instances of this archetype.
        size_t num_instances() const {
            return 1;
        }
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::ContainerBlueprint> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(
            const blueprint::archetypes::ContainerBlueprint& archetype
        );
    };
} // namespace rerun
