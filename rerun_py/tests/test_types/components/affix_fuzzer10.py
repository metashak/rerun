# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer10Ext" class in "affix_fuzzer10_ext.py".

from __future__ import annotations

from typing import Any, Sequence

import pyarrow as pa
from attrs import define, field
from rerun._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin
from rerun._converters import (
    str_or_none,
)

__all__ = ["AffixFuzzer10", "AffixFuzzer10ArrayLike", "AffixFuzzer10Batch", "AffixFuzzer10Like", "AffixFuzzer10Type"]


@define(init=False)
class AffixFuzzer10:
    def __init__(self: Any, single_string_optional: str | None = None):
        """Create a new instance of the AffixFuzzer10 component."""

        # You can define your own __init__ function as a member of AffixFuzzer10Ext in affix_fuzzer10_ext.py
        self.__attrs_init__(single_string_optional=single_string_optional)

    single_string_optional: str | None = field(default=None, converter=str_or_none)


AffixFuzzer10Like = AffixFuzzer10
AffixFuzzer10ArrayLike = AffixFuzzer10 | Sequence[AffixFuzzer10Like]


class AffixFuzzer10Type(BaseExtensionType):
    _TYPE_NAME: str = "rerun.testing.components.AffixFuzzer10"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.utf8(), self._TYPE_NAME)


class AffixFuzzer10Batch(BaseBatch[AffixFuzzer10ArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = AffixFuzzer10Type()

    @staticmethod
    def _native_to_pa_array(data: AffixFuzzer10ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in affix_fuzzer10_ext.py
