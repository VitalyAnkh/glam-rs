// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

use core::fmt;
use core::ops::*;

/// Creates a 3-dimensional `bool` vector mask.
#[inline(always)]
#[must_use]
pub const fn bvec3(x: bool, y: bool, z: bool) -> BVec3 {
    BVec3::new(x, y, z)
}

/// A 3-dimensional `bool` vector mask.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(1))]
pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec3 {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a new vector mask.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    /// Creates a vector mask with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: bool) -> Self {
        Self::new(v, v, v)
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// Returns a bitmask with the lowest 3 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn bitmask(self) -> u32 {
        (self.x as u32) | ((self.y as u32) << 1) | ((self.z as u32) << 2)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        self.x || self.y || self.z
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        self.x && self.y && self.z
    }

    /// Tests the value at `index`.
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("index out of bounds"),
        }
    }

    /// Sets the element at `index`.
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    #[must_use]
    fn into_bool_array(self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    #[must_use]
    fn into_u32_array(self) -> [u32; 3] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
        ]
    }
}

impl Default for BVec3 {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl BitAnd for BVec3 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
        }
    }
}

impl BitAnd<&Self> for BVec3 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &Self) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&BVec3> for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn bitand(self, rhs: &BVec3) -> BVec3 {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<BVec3> for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn bitand(self, rhs: BVec3) -> BVec3 {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign for BVec3 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&Self> for BVec3 {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr for BVec3 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }
}

impl BitOr<&Self> for BVec3 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &Self) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&BVec3> for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn bitor(self, rhs: &BVec3) -> BVec3 {
        (*self).bitor(*rhs)
    }
}

impl BitOr<BVec3> for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn bitor(self, rhs: BVec3) -> BVec3 {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign for BVec3 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&Self> for BVec3 {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor for BVec3 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }
}

impl BitXor<&Self> for BVec3 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&BVec3> for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn bitxor(self, rhs: &BVec3) -> BVec3 {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<BVec3> for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn bitxor(self, rhs: BVec3) -> BVec3 {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign for BVec3 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&Self> for BVec3 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }
}

impl Not for BVec3 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

impl Not for &BVec3 {
    type Output = BVec3;
    #[inline]
    fn not(self) -> BVec3 {
        (*self).not()
    }
}

impl fmt::Debug for BVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x})",
            stringify!(BVec3),
            arr[0],
            arr[1],
            arr[2]
        )
    }
}

impl fmt::Display for BVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}]", arr[0], arr[1], arr[2])
    }
}

impl From<[bool; 3]> for BVec3 {
    #[inline]
    fn from(a: [bool; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec3> for [bool; 3] {
    #[inline]
    fn from(mask: BVec3) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec3> for [u32; 3] {
    #[inline]
    fn from(mask: BVec3) -> Self {
        mask.into_u32_array()
    }
}
