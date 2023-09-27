// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/mesh_properties.fbs".

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeshProperties(pub crate::datatypes::MeshProperties);

impl<T: Into<crate::datatypes::MeshProperties>> From<T> for MeshProperties {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::MeshProperties> for MeshProperties {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::MeshProperties {
        &self.0
    }
}

impl std::ops::Deref for MeshProperties {
    type Target = crate::datatypes::MeshProperties;

    #[inline]
    fn deref(&self) -> &crate::datatypes::MeshProperties {
        &self.0
    }
}

impl<'a> From<MeshProperties> for ::std::borrow::Cow<'a, MeshProperties> {
    #[inline]
    fn from(value: MeshProperties) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a MeshProperties> for ::std::borrow::Cow<'a, MeshProperties> {
    #[inline]
    fn from(value: &'a MeshProperties) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for MeshProperties {
    type Name = crate::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.MeshProperties".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![Field {
            name: "vertex_indices".to_owned(),
            data_type: DataType::List(Box::new(Field {
                name: "item".to_owned(),
                data_type: DataType::UInt32,
                is_nullable: false,
                metadata: [].into(),
            })),
            is_nullable: true,
            metadata: [].into(),
        }])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
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
            {
                _ = data0_bitmap;
                crate::datatypes::MeshProperties::to_arrow_opt(data0)?
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
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok(crate::datatypes::MeshProperties::from_arrow_opt(arrow_data)
            .with_context("rerun.components.MeshProperties#props")?
            .into_iter()
            .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.MeshProperties#props")
            .with_context("rerun.components.MeshProperties")?)
    }
}
