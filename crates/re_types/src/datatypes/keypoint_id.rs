// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A 16-bit ID representing a type of semantic keypoint within a class.
///
/// `KeypointId`s are only meaningful within the context of a `crate::components::ClassDescription`.
///
/// Used to look up an `crate::components::AnnotationInfo` for a Keypoint within the `crate::components::AnnotationContext`.
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct KeypointId(pub u16);

impl<'a> From<KeypointId> for ::std::borrow::Cow<'a, KeypointId> {
    #[inline]
    fn from(value: KeypointId) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a KeypointId> for ::std::borrow::Cow<'a, KeypointId> {
    #[inline]
    fn from(value: &'a KeypointId) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for KeypointId {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = <Vec<Self::Item<'a>> as IntoIterator>::IntoIter;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.KeypointId".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::UInt16
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                {
                    _ = extension_wrapper;
                    DataType::Extension(
                        "rerun.datatypes.KeypointId".to_owned(),
                        Box::new(DataType::UInt16),
                        None,
                    )
                    .to_logical_type()
                    .clone()
                },
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok(data
            .as_any()
            .downcast_ref::<UInt16Array>()
            .ok_or_else(|| {
                crate::DeserializationError::datatype_mismatch(
                    DataType::UInt16,
                    data.data_type().clone(),
                )
            })
            .with_context("rerun.datatypes.KeypointId#id")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.datatypes.KeypointId#id")
            .with_context("rerun.datatypes.KeypointId")?)
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Self::try_from_arrow_opt(data)?.into_iter())
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for KeypointId {}
