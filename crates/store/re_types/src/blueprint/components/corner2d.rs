// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/corner_2d.fbs".

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

/// **Component**: One of four 2D corners, typically used to align objects.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub enum Corner2D {
    /// Left top corner.
    LeftTop = 1,

    /// Right top corner.
    RightTop = 2,

    /// Left bottom corner.
    LeftBottom = 3,

    /// Right bottom corner.
    #[default]
    RightBottom = 4,
}

impl ::re_types_core::reflection::Enum for Corner2D {
    #[inline]
    fn variants() -> &'static [Self] {
        &[
            Self::LeftTop,
            Self::RightTop,
            Self::LeftBottom,
            Self::RightBottom,
        ]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::LeftTop => "Left top corner.",
            Self::RightTop => "Right top corner.",
            Self::LeftBottom => "Left bottom corner.",
            Self::RightBottom => "Right bottom corner.",
        }
    }
}

impl ::re_types_core::SizeBytes for Corner2D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}

impl std::fmt::Display for Corner2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftTop => write!(f, "LeftTop"),
            Self::RightTop => write!(f, "RightTop"),
            Self::LeftBottom => write!(f, "LeftBottom"),
            Self::RightBottom => write!(f, "RightBottom"),
        }
    }
}

::re_types_core::macros::impl_into_cow!(Corner2D);

impl ::re_types_core::Loggable for Corner2D {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.blueprint.components.Corner2D".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Union(
            std::sync::Arc::new(vec![
                Field::new("_null_markers", DataType::Null, true),
                Field::new("LeftTop", DataType::Null, true),
                Field::new("RightTop", DataType::Null, true),
                Field::new("LeftBottom", DataType::Null, true),
                Field::new("RightBottom", DataType::Null, true),
            ]),
            Some(std::sync::Arc::new(vec![0i32, 1i32, 2i32, 3i32, 4i32])),
            UnionMode::Sparse,
        )
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            // Sparse Arrow union
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            let num_variants = 4usize;
            let types = data
                .iter()
                .map(|a| match a.as_deref() {
                    None => 0,
                    Some(value) => *value as i8,
                })
                .collect();
            let fields: Vec<_> =
                std::iter::repeat(NullArray::new(DataType::Null, data.len()).boxed())
                    .take(1 + num_variants)
                    .collect();
            UnionArray::new(Self::arrow_datatype(), types, fields, None).boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.blueprint.components.Corner2D")?;
            let arrow_data_types = arrow_data.types();
            arrow_data_types
                .iter()
                .map(|typ| match typ {
                    0 => Ok(None),
                    1 => Ok(Some(Self::LeftTop)),
                    2 => Ok(Some(Self::RightTop)),
                    3 => Ok(Some(Self::LeftBottom)),
                    4 => Ok(Some(Self::RightBottom)),
                    _ => Err(DeserializationError::missing_union_arm(
                        Self::arrow_datatype(),
                        "<invalid>",
                        *typ as _,
                    )),
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.blueprint.components.Corner2D")?
        })
    }
}
