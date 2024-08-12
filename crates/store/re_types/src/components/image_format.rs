// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/image_format.fbs".

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

/// **Component**: The metadata describing the contents of a [`components::ImageBuffer`][crate::components::ImageBuffer].
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ImageFormat(pub crate::datatypes::ImageFormat);

impl ::re_types_core::SizeBytes for ImageFormat {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::ImageFormat>::is_pod()
    }
}

impl<T: Into<crate::datatypes::ImageFormat>> From<T> for ImageFormat {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::ImageFormat> for ImageFormat {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::ImageFormat {
        &self.0
    }
}

impl std::ops::Deref for ImageFormat {
    type Target = crate::datatypes::ImageFormat;

    #[inline]
    fn deref(&self) -> &crate::datatypes::ImageFormat {
        &self.0
    }
}

impl std::ops::DerefMut for ImageFormat {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::ImageFormat {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(ImageFormat);

impl ::re_types_core::Loggable for ImageFormat {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.ImageFormat".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::ImageFormat::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::ImageFormat::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::ImageFormat::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}