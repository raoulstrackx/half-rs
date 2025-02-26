//! A crate that provides support for half-precision 16-bit floating point types.
//!
//! This crate provides the [`f16`] type, which is an implementation of the IEEE 754-2008 standard
//! [`binary16`] a.k.a `half` floating point type. This 16-bit floating point type is intended for
//! efficient storage where the full range and precision of a larger floating point value is not
//! required. This is especially useful for image storage formats.
//!
//! This crate also provides a [`bf16`] type, an alternative 16-bit floating point format. The
//! [`bfloat16`] format is a truncated IEEE 754 standard `binary32` float that preserves the
//! exponent to allow the same range as [`f32`] but with only 8 bits of precision (instead of 11
//! bits for [`f16`]). See the [`bf16`] type for details.
//!
//! Because [`f16`] and [`bf16`] are primarily for efficient storage, floating point operations such
//! as addition, multiplication, etc. are not implemented. Operations should be performed with
//! [`f32`] or higher-precision types and converted to/from [`f16`] or [`bf16`] as necessary.
//!
//! This crate also provides a [`mod@slice`] module for zero-copy in-place conversions of [`u16`]
//! slices to both [`f16`] and [`bf16`], as well as efficient vectorized conversions of larger
//! buffers of floating point values to and from these half formats.
//!
//! A [`prelude`] module is provided for easy importing of available utility traits.
//!
//! Some hardware architectures provide support for 16-bit floating point conversions. Enable the
//! `use-intrinsics` feature to use LLVM intrinsics for hardware conversions. This crate does no
//! checks on whether the hardware supports the feature. This feature currently only works on
//! nightly Rust due to a compiler feature gate. When this feature is enabled and the hardware
//! supports it, the [`mod@slice`] trait conversions will use vectorized SIMD intructions for
//! increased efficiency.
//!
//! Support for [`serde`] crate `Serialize` and `Deserialize` traits is provided when the `serde`
//! feature is enabled. This adds a dependency on [`serde`] crate so is an optional cargo feature.
//! Support for [`bytemuck`] crate `Zeroable` and `Pod` traits is provided with the `bytemuck`
//! feature. Support for the [`num-traits`] crate `ToPrimitive`, `FromPrimitive`, and `AsPrimitive`
//! traits is provided with the `num-traits` feature.
//!
//! The crate uses `#[no_std]` by default, so can be used in embedded environments without using the
//! Rust `std` library. A `std` feature is available, which enables additional utilities using the
//! `std` library, such as the [`mod@vec`] module that provides zero-copy `Vec` conversions. The
//! `alloc` feature may be used to enable the [`mod@vec`] module without adding a dependency to the
//! `std` library.
//!
//! [`binary16`]: https://en.wikipedia.org/wiki/Half-precision_floating-point_format
//! [`bfloat16`]: https://en.wikipedia.org/wiki/Bfloat16_floating-point_format
//! [`serde`]: https://crates.io/crates/serde
//! [`bytemuck`]: https://crates.io/crates/bytemuck
//! [`num-traits`]: https://crates.io/crates/num-traits

#![warn(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_numeric_casts,
    future_incompatible
)]
#![allow(clippy::verbose_bit_mask, clippy::cast_lossless)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(
        feature = "use-intrinsics",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    feature(stdsimd, f16c_target_feature)
)]
#![doc(html_root_url = "https://docs.rs/half/1.7.1")]
#![doc(test(attr(deny(warnings), allow(unused))))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

mod bfloat;
mod binary16;
#[cfg(feature = "num-traits")]
mod num_traits;

pub mod slice;
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod vec;

pub use bfloat::bf16;
#[doc(hidden)]
#[allow(deprecated)]
pub use binary16::consts;
pub use binary16::f16;

/// A collection of the most used items and traits in this crate for easy importing.
///
/// # Examples
///
/// ```rust
/// use half::prelude::*;
/// ```
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        bf16, f16,
        slice::{HalfBitsSliceExt, HalfFloatSliceExt},
    };

    #[cfg(any(feature = "alloc", feature = "std"))]
    #[doc(no_inline)]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub use crate::vec::{HalfBitsVecExt, HalfFloatVecExt};
}

// Keep this module private to crate
pub(crate) mod private {
    use crate::{bf16, f16};

    pub trait SealedHalf {}

    impl SealedHalf for f16 {}
    impl SealedHalf for bf16 {}
}
