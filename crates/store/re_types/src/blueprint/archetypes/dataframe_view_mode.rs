// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/dataframe_view_mode.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Configuration for the dataframe view
#[derive(Clone, Debug, Copy)]
pub struct DataframeViewMode {
    /// The kind of table to display
    pub mode: Option<crate::blueprint::components::DataframeViewMode>,
}

impl ::re_types_core::SizeBytes for DataframeViewMode {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.mode.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::DataframeViewMode>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.DataframeViewModeIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.DataframeViewMode".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.DataframeViewModeIndicator".into(),
            "rerun.blueprint.components.DataframeViewMode".into(),
        ]
    });

static REQUIRED_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static OPTIONAL_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("DataframeViewMode".into()),
            component_name: "rerun.blueprint.components.DataframeViewMode".into(),
            tag: Some("mode".into()),
        }]
    });

static ALL_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("DataframeViewMode".into()),
            component_name: "rerun.blueprint.components.DataframeViewMode".into(),
            tag: Some("mode".into()),
        }]
    });

impl DataframeViewMode {
    /// The total number of components in the archetype: 0 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`DataframeViewMode`] [`::re_types_core::Archetype`]
pub type DataframeViewModeIndicator = ::re_types_core::GenericIndicatorComponent<DataframeViewMode>;

impl ::re_types_core::Archetype for DataframeViewMode {
    type Indicator = DataframeViewModeIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.DataframeViewMode".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Dataframe view mode"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: DataframeViewModeIndicator = DataframeViewModeIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn required_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn recommended_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn optional_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn all_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let mode = if let Some(array) =
            arrays_by_name.get("rerun.blueprint.components.DataframeViewMode")
        {
            <crate::blueprint::components::DataframeViewMode>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.DataframeViewMode#mode")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { mode })
    }
}

impl ::re_types_core::AsComponents for DataframeViewMode {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.mode
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for DataframeViewMode {}

impl DataframeViewMode {
    /// Create a new `DataframeViewMode`.
    #[inline]
    pub fn new() -> Self {
        Self { mode: None }
    }

    /// The kind of table to display
    #[inline]
    pub fn with_mode(
        mut self,
        mode: impl Into<crate::blueprint::components::DataframeViewMode>,
    ) -> Self {
        self.mode = Some(mode.into());
        self
    }
}
