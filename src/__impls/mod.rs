// TODO: Add ReflectedImpls for all types

#[macro_use]
pub mod __helpers;

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

/// Used to represent a type which is private in the external implementation.
/// Uninhabited, it's impossible to create an instance of this value
#[crate::rebound(crate_name = "crate")]
pub(crate) enum PrivateTy {}
