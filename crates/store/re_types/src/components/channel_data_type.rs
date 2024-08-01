// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/channel_data_type.fbs".

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

/// **Component**: The innermost datatype of an image.
///
/// How individual color channel components are encoded.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub enum ChannelDataType {
    /// 8-bit unsigned integer.
    #[default]
    U8 = 1,

    /// 16-bit unsigned integer.
    U16 = 2,

    /// 32-bit unsigned integer.
    U32 = 3,

    /// 64-bit unsigned integer.
    U64 = 4,

    /// 8-bit signed integer.
    I8 = 5,

    /// 16-bit signed integer.
    I16 = 6,

    /// 32-bit signed integer.
    I32 = 7,

    /// 64-bit signed integer.
    I64 = 8,

    /// 16-bit IEEE-754 floating point, also known as `half`.
    F16 = 9,

    /// 32-bit IEEE-754 floating point, also known as `float` or `single`.
    F32 = 10,

    /// 64-bit IEEE-754 floating point, also known as `double`.
    F64 = 11,
}

impl ::re_types_core::reflection::Enum for ChannelDataType {
    #[inline]
    fn variants() -> &'static [Self] {
        &[
            Self::U8,
            Self::U16,
            Self::U32,
            Self::U64,
            Self::I8,
            Self::I16,
            Self::I32,
            Self::I64,
            Self::F16,
            Self::F32,
            Self::F64,
        ]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::U8 => "8-bit unsigned integer.",
            Self::U16 => "16-bit unsigned integer.",
            Self::U32 => "32-bit unsigned integer.",
            Self::U64 => "64-bit unsigned integer.",
            Self::I8 => "8-bit signed integer.",
            Self::I16 => "16-bit signed integer.",
            Self::I32 => "32-bit signed integer.",
            Self::I64 => "64-bit signed integer.",
            Self::F16 => "16-bit IEEE-754 floating point, also known as `half`.",
            Self::F32 => "32-bit IEEE-754 floating point, also known as `float` or `single`.",
            Self::F64 => "64-bit IEEE-754 floating point, also known as `double`.",
        }
    }
}

impl ::re_types_core::SizeBytes for ChannelDataType {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}

impl std::fmt::Display for ChannelDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8 => write!(f, "U8"),
            Self::U16 => write!(f, "U16"),
            Self::U32 => write!(f, "U32"),
            Self::U64 => write!(f, "U64"),
            Self::I8 => write!(f, "I8"),
            Self::I16 => write!(f, "I16"),
            Self::I32 => write!(f, "I32"),
            Self::I64 => write!(f, "I64"),
            Self::F16 => write!(f, "F16"),
            Self::F32 => write!(f, "F32"),
            Self::F64 => write!(f, "F64"),
        }
    }
}

::re_types_core::macros::impl_into_cow!(ChannelDataType);

impl ::re_types_core::Loggable for ChannelDataType {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.ChannelDataType".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Union(
            std::sync::Arc::new(vec![
                Field::new("_null_markers", DataType::Null, true),
                Field::new("U8", DataType::Null, true),
                Field::new("U16", DataType::Null, true),
                Field::new("U32", DataType::Null, true),
                Field::new("U64", DataType::Null, true),
                Field::new("I8", DataType::Null, true),
                Field::new("I16", DataType::Null, true),
                Field::new("I32", DataType::Null, true),
                Field::new("I64", DataType::Null, true),
                Field::new("F16", DataType::Null, true),
                Field::new("F32", DataType::Null, true),
                Field::new("F64", DataType::Null, true),
            ]),
            Some(std::sync::Arc::new(vec![
                0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32, 11i32,
            ])),
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
            let num_variants = 11usize;
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
                .with_context("rerun.components.ChannelDataType")?;
            let arrow_data_types = arrow_data.types();
            arrow_data_types
                .iter()
                .map(|typ| match typ {
                    0 => Ok(None),
                    1 => Ok(Some(Self::U8)),
                    2 => Ok(Some(Self::U16)),
                    3 => Ok(Some(Self::U32)),
                    4 => Ok(Some(Self::U64)),
                    5 => Ok(Some(Self::I8)),
                    6 => Ok(Some(Self::I16)),
                    7 => Ok(Some(Self::I32)),
                    8 => Ok(Some(Self::I64)),
                    9 => Ok(Some(Self::F16)),
                    10 => Ok(Some(Self::F32)),
                    11 => Ok(Some(Self::F64)),
                    _ => Err(DeserializationError::missing_union_arm(
                        Self::arrow_datatype(),
                        "<invalid>",
                        *typ as _,
                    )),
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.components.ChannelDataType")?
        })
    }
}
