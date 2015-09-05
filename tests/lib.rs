extern crate bytebuffer;

use bytebuffer::*;

#[test]
fn test_empty() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(1);
    assert_eq!(buffer.len(), 1);
}

#[test]
fn test_u8() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xF0);
    assert_eq!(buffer.read_u8(), 0xF0);
}

#[test]
fn test_u16() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u16(0xF0E1);
    assert_eq!(buffer.read_u16(), 0xF0E1);
}

#[test]
fn test_u32() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0xF0E1D2C3);
    assert_eq!(buffer.read_u32(), 0xF0E1D2C3);
}

#[test]
fn test_u64() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u64(0xF0E1D2C3B4A59687);
    assert_eq!(buffer.read_u64(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_signed() {
    let mut buffer = ByteBuffer::new();
    buffer.write_i8(-1);
    assert_eq!(buffer.read_u8(), 0xFF);
}

#[test]
fn test_string() {
    let mut buffer = ByteBuffer::new();
    buffer.write_string("hello");
    assert_eq!(buffer.read_string(), "hello");
}


#[test]
fn test_mixed() {
    let mut buffer = ByteBuffer::new();
    buffer.write_i16(-1);
    buffer.write_string("hello");
    buffer.write_u64(0xF0E1D2C3B4A59687);
    assert_eq!(buffer.read_i16(), -1);
    assert_eq!(buffer.read_string(), "hello");
    assert_eq!(buffer.read_u64(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_to_string() {
    let mut buffer = ByteBuffer::new();
    buffer.write_string("hello");
    assert_eq!(buffer.to_string(), "0x00 0x00 0x00 0x05 0x68 0x65 0x6c 0x6c 0x6f");
}
