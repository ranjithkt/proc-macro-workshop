// Compile-time checks for bitfield validity

// Marker types for modular arithmetic checks
pub enum ZeroMod8 {}
pub enum OneMod8 {}
pub enum TwoMod8 {}
pub enum ThreeMod8 {}
pub enum FourMod8 {}
pub enum FiveMod8 {}
pub enum SixMod8 {}
pub enum SevenMod8 {}

/// Trait that is only implemented for ZeroMod8
pub trait TotalSizeIsMultipleOfEightBits {
    const CHECK: () = ();
}
impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {}

/// Helper trait to get the modular type for a given value
pub trait ModuloEight {
    type Mod;
}

// Type-level modulo calculation helper
pub struct Modulo<const N: usize>;

impl ModuloEight for Modulo<0> {
    type Mod = ZeroMod8;
}
impl ModuloEight for Modulo<1> {
    type Mod = OneMod8;
}
impl ModuloEight for Modulo<2> {
    type Mod = TwoMod8;
}
impl ModuloEight for Modulo<3> {
    type Mod = ThreeMod8;
}
impl ModuloEight for Modulo<4> {
    type Mod = FourMod8;
}
impl ModuloEight for Modulo<5> {
    type Mod = FiveMod8;
}
impl ModuloEight for Modulo<6> {
    type Mod = SixMod8;
}
impl ModuloEight for Modulo<7> {
    type Mod = SevenMod8;
}

// Type-level booleans for discriminant checks
pub enum True {}
pub enum False {}

pub trait DiscriminantInRange {
    const CHECK: () = ();
}
impl DiscriminantInRange for True {}

// Check if discriminant is in range helper
pub struct CheckDiscriminantInRange<const IN_RANGE: bool>;

pub trait GetBoolType {
    type Type;
}

impl GetBoolType for CheckDiscriminantInRange<true> {
    type Type = True;
}

impl GetBoolType for CheckDiscriminantInRange<false> {
    type Type = False;
}
