// Implementations for base types, such as `char` or `*const T`
mod base;

// Implementations for core types
#[cfg(feature = "core")]
mod core;

// Implementations for alloc types
#[cfg(feature = "alloc")]
mod alloc;

// Implementations for std types
#[cfg(feature = "std")]
mod std;

/// Used to represent a type which is private in the external implementation
#[crate::rebound(crate_name = "crate")]
pub(crate) struct PrivateTy;
