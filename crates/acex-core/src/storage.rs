//! Bounded-collection storage, backed by `heapless::Vec` (inline, no_std)
//! or `alloc::vec::Vec` (heap-backed) depending on the `alloc` feature.
//! Capacity `N` is enforced identically in both modes; only where the
//! bytes live differs.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "alloc"))]
pub use heapless::Vec;

#[cfg(feature = "alloc")]
pub type Vec<T, const N: usize> = alloc::vec::Vec<T>;
