# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer1Ext" class in "affix_fuzzer1_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field
from rerun._baseclasses import Archetype

from .. import components, datatypes

__all__ = ["AffixFuzzer1"]


@define(str=False, repr=False, init=False)
class AffixFuzzer1(Archetype):
    def __init__(
        self: Any,
        fuzz1001: datatypes.AffixFuzzer1Like,
        fuzz1002: datatypes.AffixFuzzer1Like,
        fuzz1003: datatypes.AffixFuzzer1Like,
        fuzz1004: datatypes.AffixFuzzer1Like,
        fuzz1005: datatypes.AffixFuzzer1Like,
        fuzz1006: datatypes.AffixFuzzer1Like,
        fuzz1007: components.AffixFuzzer7Like,
        fuzz1008: components.AffixFuzzer8Like,
        fuzz1009: components.AffixFuzzer9Like,
        fuzz1010: components.AffixFuzzer10Like,
        fuzz1011: components.AffixFuzzer11Like,
        fuzz1012: components.AffixFuzzer12Like,
        fuzz1013: components.AffixFuzzer13Like,
        fuzz1014: datatypes.AffixFuzzer3Like,
        fuzz1015: datatypes.AffixFuzzer3Like,
        fuzz1016: components.AffixFuzzer16Like,
        fuzz1017: components.AffixFuzzer17Like,
        fuzz1018: components.AffixFuzzer18Like,
        fuzz1019: datatypes.AffixFuzzer5Like,
        fuzz1020: datatypes.AffixFuzzer20Like,
        fuzz1021: datatypes.AffixFuzzer21Like,
        fuzz1101: datatypes.AffixFuzzer1ArrayLike,
        fuzz1102: datatypes.AffixFuzzer1ArrayLike,
        fuzz1103: datatypes.AffixFuzzer1ArrayLike,
        fuzz1104: datatypes.AffixFuzzer1ArrayLike,
        fuzz1105: datatypes.AffixFuzzer1ArrayLike,
        fuzz1106: datatypes.AffixFuzzer1ArrayLike,
        fuzz1107: components.AffixFuzzer7ArrayLike,
        fuzz1108: components.AffixFuzzer8ArrayLike,
        fuzz1109: components.AffixFuzzer9ArrayLike,
        fuzz1110: components.AffixFuzzer10ArrayLike,
        fuzz1111: components.AffixFuzzer11ArrayLike,
        fuzz1112: components.AffixFuzzer12ArrayLike,
        fuzz1113: components.AffixFuzzer13ArrayLike,
        fuzz1114: datatypes.AffixFuzzer3ArrayLike,
        fuzz1115: datatypes.AffixFuzzer3ArrayLike,
        fuzz1116: components.AffixFuzzer16ArrayLike,
        fuzz1117: components.AffixFuzzer17ArrayLike,
        fuzz1118: components.AffixFuzzer18ArrayLike,
        fuzz2001: datatypes.AffixFuzzer1Like | None = None,
        fuzz2002: datatypes.AffixFuzzer1Like | None = None,
        fuzz2003: datatypes.AffixFuzzer1Like | None = None,
        fuzz2004: datatypes.AffixFuzzer1Like | None = None,
        fuzz2005: datatypes.AffixFuzzer1Like | None = None,
        fuzz2006: datatypes.AffixFuzzer1Like | None = None,
        fuzz2007: components.AffixFuzzer7Like | None = None,
        fuzz2008: components.AffixFuzzer8Like | None = None,
        fuzz2009: components.AffixFuzzer9Like | None = None,
        fuzz2010: components.AffixFuzzer10Like | None = None,
        fuzz2011: components.AffixFuzzer11Like | None = None,
        fuzz2012: components.AffixFuzzer12Like | None = None,
        fuzz2013: components.AffixFuzzer13Like | None = None,
        fuzz2014: datatypes.AffixFuzzer3Like | None = None,
        fuzz2015: datatypes.AffixFuzzer3Like | None = None,
        fuzz2016: components.AffixFuzzer16Like | None = None,
        fuzz2017: components.AffixFuzzer17Like | None = None,
        fuzz2018: components.AffixFuzzer18Like | None = None,
        fuzz2101: datatypes.AffixFuzzer1ArrayLike | None = None,
        fuzz2102: datatypes.AffixFuzzer1ArrayLike | None = None,
        fuzz2103: datatypes.AffixFuzzer1ArrayLike | None = None,
        fuzz2104: datatypes.AffixFuzzer1ArrayLike | None = None,
        fuzz2105: datatypes.AffixFuzzer1ArrayLike | None = None,
        fuzz2106: datatypes.AffixFuzzer1ArrayLike | None = None,
        fuzz2107: components.AffixFuzzer7ArrayLike | None = None,
        fuzz2108: components.AffixFuzzer8ArrayLike | None = None,
        fuzz2109: components.AffixFuzzer9ArrayLike | None = None,
        fuzz2110: components.AffixFuzzer10ArrayLike | None = None,
        fuzz2111: components.AffixFuzzer11ArrayLike | None = None,
        fuzz2112: components.AffixFuzzer12ArrayLike | None = None,
        fuzz2113: components.AffixFuzzer13ArrayLike | None = None,
        fuzz2114: datatypes.AffixFuzzer3ArrayLike | None = None,
        fuzz2115: datatypes.AffixFuzzer3ArrayLike | None = None,
        fuzz2116: components.AffixFuzzer16ArrayLike | None = None,
        fuzz2117: components.AffixFuzzer17ArrayLike | None = None,
        fuzz2118: components.AffixFuzzer18ArrayLike | None = None,
    ):
        """Create a new instance of the AffixFuzzer1 archetype."""

        # You can define your own __init__ function as a member of AffixFuzzer1Ext in affix_fuzzer1_ext.py
        self.__attrs_init__(
            fuzz1001=fuzz1001,
            fuzz1002=fuzz1002,
            fuzz1003=fuzz1003,
            fuzz1004=fuzz1004,
            fuzz1005=fuzz1005,
            fuzz1006=fuzz1006,
            fuzz1007=fuzz1007,
            fuzz1008=fuzz1008,
            fuzz1009=fuzz1009,
            fuzz1010=fuzz1010,
            fuzz1011=fuzz1011,
            fuzz1012=fuzz1012,
            fuzz1013=fuzz1013,
            fuzz1014=fuzz1014,
            fuzz1015=fuzz1015,
            fuzz1016=fuzz1016,
            fuzz1017=fuzz1017,
            fuzz1018=fuzz1018,
            fuzz1019=fuzz1019,
            fuzz1020=fuzz1020,
            fuzz1021=fuzz1021,
            fuzz1101=fuzz1101,
            fuzz1102=fuzz1102,
            fuzz1103=fuzz1103,
            fuzz1104=fuzz1104,
            fuzz1105=fuzz1105,
            fuzz1106=fuzz1106,
            fuzz1107=fuzz1107,
            fuzz1108=fuzz1108,
            fuzz1109=fuzz1109,
            fuzz1110=fuzz1110,
            fuzz1111=fuzz1111,
            fuzz1112=fuzz1112,
            fuzz1113=fuzz1113,
            fuzz1114=fuzz1114,
            fuzz1115=fuzz1115,
            fuzz1116=fuzz1116,
            fuzz1117=fuzz1117,
            fuzz1118=fuzz1118,
            fuzz2001=fuzz2001,
            fuzz2002=fuzz2002,
            fuzz2003=fuzz2003,
            fuzz2004=fuzz2004,
            fuzz2005=fuzz2005,
            fuzz2006=fuzz2006,
            fuzz2007=fuzz2007,
            fuzz2008=fuzz2008,
            fuzz2009=fuzz2009,
            fuzz2010=fuzz2010,
            fuzz2011=fuzz2011,
            fuzz2012=fuzz2012,
            fuzz2013=fuzz2013,
            fuzz2014=fuzz2014,
            fuzz2015=fuzz2015,
            fuzz2016=fuzz2016,
            fuzz2017=fuzz2017,
            fuzz2018=fuzz2018,
            fuzz2101=fuzz2101,
            fuzz2102=fuzz2102,
            fuzz2103=fuzz2103,
            fuzz2104=fuzz2104,
            fuzz2105=fuzz2105,
            fuzz2106=fuzz2106,
            fuzz2107=fuzz2107,
            fuzz2108=fuzz2108,
            fuzz2109=fuzz2109,
            fuzz2110=fuzz2110,
            fuzz2111=fuzz2111,
            fuzz2112=fuzz2112,
            fuzz2113=fuzz2113,
            fuzz2114=fuzz2114,
            fuzz2115=fuzz2115,
            fuzz2116=fuzz2116,
            fuzz2117=fuzz2117,
            fuzz2118=fuzz2118,
        )

    fuzz1001: components.AffixFuzzer1Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer1Batch,  # type: ignore[misc]
    )
    fuzz1002: components.AffixFuzzer2Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer2Batch,  # type: ignore[misc]
    )
    fuzz1003: components.AffixFuzzer3Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer3Batch,  # type: ignore[misc]
    )
    fuzz1004: components.AffixFuzzer4Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer4Batch,  # type: ignore[misc]
    )
    fuzz1005: components.AffixFuzzer5Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer5Batch,  # type: ignore[misc]
    )
    fuzz1006: components.AffixFuzzer6Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer6Batch,  # type: ignore[misc]
    )
    fuzz1007: components.AffixFuzzer7Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer7Batch,  # type: ignore[misc]
    )
    fuzz1008: components.AffixFuzzer8Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer8Batch,  # type: ignore[misc]
    )
    fuzz1009: components.AffixFuzzer9Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer9Batch,  # type: ignore[misc]
    )
    fuzz1010: components.AffixFuzzer10Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer10Batch,  # type: ignore[misc]
    )
    fuzz1011: components.AffixFuzzer11Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer11Batch,  # type: ignore[misc]
    )
    fuzz1012: components.AffixFuzzer12Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer12Batch,  # type: ignore[misc]
    )
    fuzz1013: components.AffixFuzzer13Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer13Batch,  # type: ignore[misc]
    )
    fuzz1014: components.AffixFuzzer14Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer14Batch,  # type: ignore[misc]
    )
    fuzz1015: components.AffixFuzzer15Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer15Batch,  # type: ignore[misc]
    )
    fuzz1016: components.AffixFuzzer16Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer16Batch,  # type: ignore[misc]
    )
    fuzz1017: components.AffixFuzzer17Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer17Batch,  # type: ignore[misc]
    )
    fuzz1018: components.AffixFuzzer18Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer18Batch,  # type: ignore[misc]
    )
    fuzz1019: components.AffixFuzzer19Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer19Batch,  # type: ignore[misc]
    )
    fuzz1020: components.AffixFuzzer20Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer20Batch,  # type: ignore[misc]
    )
    fuzz1021: components.AffixFuzzer21Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer21Batch,  # type: ignore[misc]
    )
    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
