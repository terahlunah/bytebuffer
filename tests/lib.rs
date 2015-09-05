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


#[test]
fn test_wpos() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0);
    buffer.set_wpos(1);
    buffer.write_u8(0xFF);
    buffer.write_u8(0x11);
    assert_eq!(buffer.read_u32(), 0x00FF1100);
}

#[test]
fn test_rpos() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0x0000FF00);
    buffer.set_rpos(2);
    assert_eq!(buffer.read_u8(), 0xFF);
}

#[test]
fn test_to_bytes() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFF);
    assert_eq!(buffer.to_bytes(), vec![0xFF]);
}