# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/datatypes/angle.fbs".

# You can extend this class by creating a "AngleExt" class in "angle_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Literal, Sequence

import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType
from .angle_ext import AngleExt

__all__ = ["Angle", "AngleArrayLike", "AngleBatch", "AngleLike", "AngleType"]


@define(init=False)
class Angle(AngleExt):
    """**Datatype**: Angle in either radians or degrees."""

    # __init__ can be found in angle_ext.py

    inner: float = field(converter=float)
    """
    Radians (float):
        3D rotation angle in radians. Only one of `degrees` or `radians` should be set.

    Degrees (float):
        3D rotation angle in degrees. Only one of `degrees` or `radians` should be set.
    """

    kind: Literal["radians", "degrees"] = field(default="radians")


if TYPE_CHECKING:
    AngleLike = Angle | float
    AngleArrayLike = Angle | float | Sequence[AngleLike]
else:
    AngleLike = Any
    AngleArrayLike = Any


class AngleType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.Angle"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.dense_union(
                [
                    pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                    pa.field("Radians", pa.float32(), nullable=False, metadata={}),
                    pa.field("Degrees", pa.float32(), nullable=False, metadata={}),
                ]
            ),
            self._TYPE_NAME,
        )


class AngleBatch(BaseBatch[AngleArrayLike]):
    _ARROW_TYPE = AngleType()

    @staticmethod
    def _native_to_pa_array(data: AngleArrayLike, data_type: pa.DataType) -> pa.Array:
        return AngleExt.native_to_pa_array_override(data, data_type)
