use crate::{components, view_coordinates::ViewDir};

use super::ViewCoordinates;

macro_rules! define_coordinates {
    ($docstring:literal, $name:ident => ($x:ident, $y:ident, $z:ident) ) => {
        #[doc = $docstring]
        pub const $name: Self = Self {
            xyz: components::ViewCoordinates::new(ViewDir::$x, ViewDir::$y, ViewDir::$z),
        };
    };
}

impl ViewCoordinates {
    // <BEGIN_GENERATED:declarations>
    // This section is generated by running `scripts/generate_view_coordinate_defs.py --rust`
    define_coordinates!("X=Up, Y=Left, Z=Forward", ULF => (Up, Left, Forward));
    define_coordinates!("X=Up, Y=Forward, Z=Left", UFL => (Up, Forward, Left));
    define_coordinates!("X=Left, Y=Up, Z=Forward", LUF => (Left, Up, Forward));
    define_coordinates!("X=Left, Y=Forward, Z=Up", LFU => (Left, Forward, Up));
    define_coordinates!("X=Forward, Y=Up, Z=Left", FUL => (Forward, Up, Left));
    define_coordinates!("X=Forward, Y=Left, Z=Up", FLU => (Forward, Left, Up));
    define_coordinates!("X=Up, Y=Left, Z=Back", ULB => (Up, Left, Back));
    define_coordinates!("X=Up, Y=Back, Z=Left", UBL => (Up, Back, Left));
    define_coordinates!("X=Left, Y=Up, Z=Back", LUB => (Left, Up, Back));
    define_coordinates!("X=Left, Y=Back, Z=Up", LBU => (Left, Back, Up));
    define_coordinates!("X=Back, Y=Up, Z=Left", BUL => (Back, Up, Left));
    define_coordinates!("X=Back, Y=Left, Z=Up", BLU => (Back, Left, Up));
    define_coordinates!("X=Up, Y=Right, Z=Forward", URF => (Up, Right, Forward));
    define_coordinates!("X=Up, Y=Forward, Z=Right", UFR => (Up, Forward, Right));
    define_coordinates!("X=Right, Y=Up, Z=Forward", RUF => (Right, Up, Forward));
    define_coordinates!("X=Right, Y=Forward, Z=Up", RFU => (Right, Forward, Up));
    define_coordinates!("X=Forward, Y=Up, Z=Right", FUR => (Forward, Up, Right));
    define_coordinates!("X=Forward, Y=Right, Z=Up", FRU => (Forward, Right, Up));
    define_coordinates!("X=Up, Y=Right, Z=Back", URB => (Up, Right, Back));
    define_coordinates!("X=Up, Y=Back, Z=Right", UBR => (Up, Back, Right));
    define_coordinates!("X=Right, Y=Up, Z=Back", RUB => (Right, Up, Back));
    define_coordinates!("X=Right, Y=Back, Z=Up", RBU => (Right, Back, Up));
    define_coordinates!("X=Back, Y=Up, Z=Right", BUR => (Back, Up, Right));
    define_coordinates!("X=Back, Y=Right, Z=Up", BRU => (Back, Right, Up));
    define_coordinates!("X=Down, Y=Left, Z=Forward", DLF => (Down, Left, Forward));
    define_coordinates!("X=Down, Y=Forward, Z=Left", DFL => (Down, Forward, Left));
    define_coordinates!("X=Left, Y=Down, Z=Forward", LDF => (Left, Down, Forward));
    define_coordinates!("X=Left, Y=Forward, Z=Down", LFD => (Left, Forward, Down));
    define_coordinates!("X=Forward, Y=Down, Z=Left", FDL => (Forward, Down, Left));
    define_coordinates!("X=Forward, Y=Left, Z=Down", FLD => (Forward, Left, Down));
    define_coordinates!("X=Down, Y=Left, Z=Back", DLB => (Down, Left, Back));
    define_coordinates!("X=Down, Y=Back, Z=Left", DBL => (Down, Back, Left));
    define_coordinates!("X=Left, Y=Down, Z=Back", LDB => (Left, Down, Back));
    define_coordinates!("X=Left, Y=Back, Z=Down", LBD => (Left, Back, Down));
    define_coordinates!("X=Back, Y=Down, Z=Left", BDL => (Back, Down, Left));
    define_coordinates!("X=Back, Y=Left, Z=Down", BLD => (Back, Left, Down));
    define_coordinates!("X=Down, Y=Right, Z=Forward", DRF => (Down, Right, Forward));
    define_coordinates!("X=Down, Y=Forward, Z=Right", DFR => (Down, Forward, Right));
    define_coordinates!("X=Right, Y=Down, Z=Forward", RDF => (Right, Down, Forward));
    define_coordinates!("X=Right, Y=Forward, Z=Down", RFD => (Right, Forward, Down));
    define_coordinates!("X=Forward, Y=Down, Z=Right", FDR => (Forward, Down, Right));
    define_coordinates!("X=Forward, Y=Right, Z=Down", FRD => (Forward, Right, Down));
    define_coordinates!("X=Down, Y=Right, Z=Back", DRB => (Down, Right, Back));
    define_coordinates!("X=Down, Y=Back, Z=Right", DBR => (Down, Back, Right));
    define_coordinates!("X=Right, Y=Down, Z=Back", RDB => (Right, Down, Back));
    define_coordinates!("X=Right, Y=Back, Z=Down", RBD => (Right, Back, Down));
    define_coordinates!("X=Back, Y=Down, Z=Right", BDR => (Back, Down, Right));
    define_coordinates!("X=Back, Y=Right, Z=Down", BRD => (Back, Right, Down));
    define_coordinates!("X=Up, Y=Left, Z=Unused", UL => (Up, Left, Unused));
    define_coordinates!("X=Left, Y=Up, Z=Unused", LU => (Left, Up, Unused));
    define_coordinates!("X=Up, Y=Right, Z=Unused", UR => (Up, Right, Unused));
    define_coordinates!("X=Right, Y=Up, Z=Unused", RU => (Right, Up, Unused));
    define_coordinates!("X=Down, Y=Left, Z=Unused", DL => (Down, Left, Unused));
    define_coordinates!("X=Left, Y=Down, Z=Unused", LD => (Left, Down, Unused));
    define_coordinates!("X=Down, Y=Right, Z=Unused", DR => (Down, Right, Unused));
    define_coordinates!("X=Right, Y=Down, Z=Unused", RD => (Right, Down, Unused));
    define_coordinates!("X=Up, Y=Right, Z=Forward", RIGHT_HAND_X_UP => (Up, Right, Forward));
    define_coordinates!("X=Down, Y=Right, Z=Back", RIGHT_HAND_X_DOWN => (Down, Right, Back));
    define_coordinates!("X=Right, Y=Up, Z=Back", RIGHT_HAND_Y_UP => (Right, Up, Back));
    define_coordinates!("X=Right, Y=Down, Z=Forward", RIGHT_HAND_Y_DOWN => (Right, Down, Forward));
    define_coordinates!("X=Right, Y=Forward, Z=Up", RIGHT_HAND_Z_UP => (Right, Forward, Up));
    define_coordinates!("X=Right, Y=Back, Z=Down", RIGHT_HAND_Z_DOWN => (Right, Back, Down));
    define_coordinates!("X=Up, Y=Right, Z=Back", LEFT_HAND_X_UP => (Up, Right, Back));
    define_coordinates!("X=Down, Y=Right, Z=Forward", LEFT_HAND_X_DOWN => (Down, Right, Forward));
    define_coordinates!("X=Right, Y=Up, Z=Forward", LEFT_HAND_Y_UP => (Right, Up, Forward));
    define_coordinates!("X=Right, Y=Down, Z=Back", LEFT_HAND_Y_DOWN => (Right, Down, Back));
    define_coordinates!("X=Right, Y=Back, Z=Up", LEFT_HAND_Z_UP => (Right, Back, Up));
    define_coordinates!("X=Right, Y=Forward, Z=Down", LEFT_HAND_Z_DOWN => (Right, Forward, Down));
    // <END_GENERATED:declarations>
}
