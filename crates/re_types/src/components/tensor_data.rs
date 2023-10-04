// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/tensor_data.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// **Component**: A multi-dimensional `Tensor` with optionally named arguments.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct TensorData(pub crate::datatypes::TensorData);

impl<T: Into<crate::datatypes::TensorData>> From<T> for TensorData {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::TensorData> for TensorData {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::TensorData {
        &self.0
    }
}

impl std::ops::Deref for TensorData {
    type Target = crate::datatypes::TensorData;

    #[inline]
    fn deref(&self) -> &crate::datatypes::TensorData {
        &self.0
    }
}

impl<'a> From<TensorData> for ::std::borrow::Cow<'a, TensorData> {
    #[inline]
    fn from(value: TensorData) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a TensorData> for ::std::borrow::Cow<'a, TensorData> {
    #[inline]
    fn from(value: &'a TensorData) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for TensorData {
    type Name = crate::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.TensorData".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "shape".to_owned(),
                data_type: DataType::List(Box::new(Field {
                    name: "item".to_owned(),
                    data_type: <crate::datatypes::TensorDimension>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                })),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "buffer".to_owned(),
                data_type: <crate::datatypes::TensorBuffer>::arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
        ])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
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
            {
                _ = data0_bitmap;
                crate::datatypes::TensorData::to_arrow_opt(data0)?
            }
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok(crate::datatypes::TensorData::from_arrow_opt(arrow_data)
            .with_context("rerun.components.TensorData#data")?
            .into_iter()
            .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.TensorData#data")
            .with_context("rerun.components.TensorData")?)
    }
}
