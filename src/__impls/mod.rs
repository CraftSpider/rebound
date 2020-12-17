
// Implementations for base types, such as `char` or `*const T`
mod base;

// Implementations for core types
#[cfg(feature = "core")]
mod core;

// Implementations for std types
#[cfg(feature = "std")]
mod std;
