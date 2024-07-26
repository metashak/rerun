// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/ellipsoids.fbs".

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
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: 3D ellipsoids or spheres.
///
/// This archetype is for ellipsoids or spheres whose size is a key part of the data
/// (e.g. a bounding sphere).
/// For points whose radii are for the sake of visualization, use [`archetypes::Points3D`][crate::archetypes::Points3D] instead.
///
/// Currently, ellipsoids are always rendered as wireframes.
/// Opaque and transparent rendering will be supported later.
///
/// The axis of the ellipsoid are aligned with axes of the coordinate system.
/// Use [`archetypes.Transform3D`] to rotate the ellipsoid(s) freely.
/// If you have several ellipsoids, you can transform them individually by logging arrays of transform components
/// (this will automatically enable out-of-tree transform, meaning that transformation won't affect the children of this entity).
#[derive(Clone, Debug, PartialEq)]
pub struct Ellipsoids {
    /// For each ellipsoid, half of its size on its three axes.
    ///
    /// If all components are equal, then it is a sphere with that radius.
    pub half_sizes: Vec<crate::components::HalfSize3D>,

    /// Optional center positions of the ellipsoids.
    ///
    /// If not specified, the centers will be at (0, 0, 0).
    pub centers: Option<Vec<crate::components::Position3D>>,

    /// Optional colors for the ellipsoids.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
    pub line_radii: Option<Vec<crate::components::Radius>>,

    /// Optionally choose whether the ellipsoids are drawn with lines or solid.
    pub fill_mode: Option<crate::components::FillMode>,

    /// Optional text labels for the ellipsoids.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional [`components::ClassId`][crate::components::ClassId]s for the ellipsoids.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,
}

impl ::re_types_core::SizeBytes for Ellipsoids {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.half_sizes.heap_size_bytes()
            + self.centers.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.line_radii.heap_size_bytes()
            + self.fill_mode.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::HalfSize3D>>::is_pod()
            && <Option<Vec<crate::components::Position3D>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<crate::components::FillMode>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.HalfSize3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Position3D".into(),
            "rerun.components.Color".into(),
            "rerun.components.EllipsoidsIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Radius".into(),
            "rerun.components.FillMode".into(),
            "rerun.components.Text".into(),
            "rerun.components.ClassId".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.HalfSize3D".into(),
            "rerun.components.Position3D".into(),
            "rerun.components.Color".into(),
            "rerun.components.EllipsoidsIndicator".into(),
            "rerun.components.Radius".into(),
            "rerun.components.FillMode".into(),
            "rerun.components.Text".into(),
            "rerun.components.ClassId".into(),
        ]
    });

impl Ellipsoids {
    /// The total number of components in the archetype: 1 required, 3 recommended, 4 optional
    pub const NUM_COMPONENTS: usize = 8usize;
}

/// Indicator component for the [`Ellipsoids`] [`::re_types_core::Archetype`]
pub type EllipsoidsIndicator = ::re_types_core::GenericIndicatorComponent<Ellipsoids>;

impl ::re_types_core::Archetype for Ellipsoids {
    type Indicator = EllipsoidsIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Ellipsoids".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Ellipsoids"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: EllipsoidsIndicator = EllipsoidsIndicator::DEFAULT;
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
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let half_sizes = {
            let array = arrays_by_name
                .get("rerun.components.HalfSize3D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Ellipsoids#half_sizes")?;
            <crate::components::HalfSize3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Ellipsoids#half_sizes")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Ellipsoids#half_sizes")?
        };
        let centers = if let Some(array) = arrays_by_name.get("rerun.components.Position3D") {
            Some({
                <crate::components::Position3D>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Ellipsoids#centers")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Ellipsoids#centers")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Ellipsoids#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Ellipsoids#colors")?
            })
        } else {
            None
        };
        let line_radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Ellipsoids#line_radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Ellipsoids#line_radii")?
            })
        } else {
            None
        };
        let fill_mode = if let Some(array) = arrays_by_name.get("rerun.components.FillMode") {
            <crate::components::FillMode>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Ellipsoids#fill_mode")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Ellipsoids#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Ellipsoids#labels")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Ellipsoids#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Ellipsoids#class_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            half_sizes,
            centers,
            colors,
            line_radii,
            fill_mode,
            labels,
            class_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Ellipsoids {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.half_sizes as &dyn ComponentBatch).into()),
            self.centers
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.line_radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fill_mode
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Ellipsoids {
    /// Create a new `Ellipsoids`.
    #[inline]
    pub(crate) fn new(
        half_sizes: impl IntoIterator<Item = impl Into<crate::components::HalfSize3D>>,
    ) -> Self {
        Self {
            half_sizes: half_sizes.into_iter().map(Into::into).collect(),
            centers: None,
            colors: None,
            line_radii: None,
            fill_mode: None,
            labels: None,
            class_ids: None,
        }
    }

    /// Optional center positions of the ellipsoids.
    ///
    /// If not specified, the centers will be at (0, 0, 0).
    #[inline]
    pub fn with_centers(
        mut self,
        centers: impl IntoIterator<Item = impl Into<crate::components::Position3D>>,
    ) -> Self {
        self.centers = Some(centers.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the ellipsoids.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
    #[inline]
    pub fn with_line_radii(
        mut self,
        line_radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.line_radii = Some(line_radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optionally choose whether the ellipsoids are drawn with lines or solid.
    #[inline]
    pub fn with_fill_mode(mut self, fill_mode: impl Into<crate::components::FillMode>) -> Self {
        self.fill_mode = Some(fill_mode.into());
        self
    }

    /// Optional text labels for the ellipsoids.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional [`components::ClassId`][crate::components::ClassId]s for the ellipsoids.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }
}
