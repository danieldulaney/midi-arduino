extern crate byteorder;

pub mod chunk;
pub mod header;
mod varlen;

pub use chunk::{Chunk, Parser};
pub use header::{Division, Format, Header};
