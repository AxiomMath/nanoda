//! Placeholder:
//! ```ignore
//! Doc comment example
//! ```
#![allow(clippy::too_many_arguments)]
#![deny(clippy::cast_possible_truncation)]

pub mod debug_printer;
pub mod env;
pub mod expr;
pub mod inductive;
pub mod level;
pub mod name;
pub mod parser;
pub mod pretty_printer;
pub mod quot;
pub mod tc;
#[cfg(test)]
mod tests;
pub mod unique_hasher;
pub mod util;

pub(crate) const STACK_SIZE: usize = 16_777_216;

/// Runs `f`, first switching to a new heap-allocated 256 MiB stack segment (via
/// `stacker::maybe_grow`) if less than 1 MiB remains on the current stack, so deep recursion in the
/// type checker grows the stack instead of overflowing it.
#[inline]
pub(crate) fn grow<R>(f: impl FnOnce() -> R) -> R { stacker::maybe_grow(1 << 20, 256 << 20, f) }
