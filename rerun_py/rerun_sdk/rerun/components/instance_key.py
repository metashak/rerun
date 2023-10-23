# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/instance_key.fbs".

# You can extend this class by creating a "InstanceKeyExt" class in "instance_key_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin
from .instance_key_ext import InstanceKeyExt

__all__ = ["InstanceKey", "InstanceKeyArrayLike", "InstanceKeyBatch", "InstanceKeyLike", "InstanceKeyType"]


@define(init=False)
class InstanceKey(InstanceKeyExt):
    """**Component**: A unique numeric identifier for each individual instance within a batch."""

    def __init__(self: Any, value: InstanceKeyLike):
        """Create a new instance of the InstanceKey component."""

        # You can define your own __init__ function as a member of InstanceKeyExt in instance_key_ext.py
        self.__attrs_init__(value=value)

    value: int = field(converter=int)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of InstanceKeyExt in instance_key_ext.py
        return np.asarray(self.value, dtype=dtype)

    def __int__(self) -> int:
        return int(self.value)


if TYPE_CHECKING:
    InstanceKeyLike = InstanceKey | int
else:
    InstanceKeyLike = Any

InstanceKeyArrayLike = InstanceKey | Sequence[InstanceKeyLike] | int | npt.NDArray[np.uint64]


class InstanceKeyType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.InstanceKey"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.uint64(), self._TYPE_NAME)


class InstanceKeyBatch(BaseBatch[InstanceKeyArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = InstanceKeyType()

    @staticmethod
    def _native_to_pa_array(data: InstanceKeyArrayLike, data_type: pa.DataType) -> pa.Array:
        return InstanceKeyExt.native_to_pa_array_override(data, data_type)
