#![deny(clippy::all)]

mod buffer;
mod reader;

pub use buffer::ByteBuffer;
pub use reader::ByteReader;

/// An enum to represent the byte order of the ByteBuffer object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}
