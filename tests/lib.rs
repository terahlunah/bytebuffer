use bytebuffer::*;
use std::io::{ErrorKind, Read, Write};

#[test]
fn test_api() {
    let mut buffer = ByteBuffer::new();
    buffer.write_bytes(&vec![0x1, 0xFF, 0x45]);
    buffer.write_u8(1);
    buffer.write_i8(1);
    buffer.write_u16(1);
    buffer.write_i16(1);
    buffer.write_u32(1);
    buffer.write_i32(1);
    buffer.write_u64(1);
    buffer.write_i64(1);
    buffer.write_f32(0.1);
    buffer.write_f64(0.1);
    buffer.write_string("Hello");
    buffer.write_bit(true);
    buffer.write_bits(4, 3);
    buffer.flush_bits();

    let data = buffer.into_bytes();

    let mut buffer = ByteBuffer::from_vec(data);
    let _ = buffer.read_bytes(3);
    let _ = buffer.read_u8();
    let _ = buffer.read_i8();
    let _ = buffer.read_u16();
    let _ = buffer.read_i16();
    let _ = buffer.read_u32();
    let _ = buffer.read_i32();
    let _ = buffer.read_u64();
    let _ = buffer.read_i64();
    let _ = buffer.read_f32();
    let _ = buffer.read_f64();
    let _ = buffer.read_string();
    let _ = buffer.read_bit();
    let _ = buffer.read_bits(3);
}

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
    assert_eq!(buffer.read_u8().unwrap(), 0xF0);
}

#[test]
fn test_u16() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u16(0xF0E1);
    assert_eq!(buffer.read_u16().unwrap(), 0xF0E1);
}

#[test]
fn test_u16_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_u16(0xF0E1);
    assert_eq!(buffer.read_u16().unwrap(), 0xF0E1);
}

#[test]
fn test_u32() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0xF0E1D2C3);
    assert_eq!(buffer.read_u32().unwrap(), 0xF0E1D2C3);
}

#[test]
fn test_u32_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_u32(0xF0E1D2C3);
    assert_eq!(buffer.read_u32().unwrap(), 0xF0E1D2C3);
}

#[test]
fn test_u64() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u64(0xF0E1D2C3B4A59687);
    assert_eq!(buffer.read_u64().unwrap(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_u64_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_u64(0xF0E1D2C3B4A59687);
    assert_eq!(buffer.read_u64().unwrap(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_signed() {
    let mut buffer = ByteBuffer::new();
    buffer.write_i8(-1);
    assert_eq!(buffer.read_u8().unwrap(), 0xFF);
}

#[test]
fn test_signed_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_i8(-1);
    assert_eq!(buffer.read_u8().unwrap(), 0xFF);
}

#[test]
fn test_string() {
    let mut buffer = ByteBuffer::new();
    buffer.write_string("hello");
    assert_eq!(buffer.read_string().unwrap(), "hello");
}

#[test]
fn test_mixed() {
    let mut buffer = ByteBuffer::new();
    buffer.write_i16(-1);
    buffer.write_string("hello");
    buffer.write_u64(0xF0E1D2C3B4A59687);
    assert_eq!(buffer.read_i16().unwrap(), -1);
    assert_eq!(buffer.read_string().unwrap(), "hello");
    assert_eq!(buffer.read_u64().unwrap(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_string_overread_protection() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(2);
    buffer.write_bytes(&[0x65]);
    let result = buffer.read_string();
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn test_to_string() {
    let mut buffer = ByteBuffer::new();
    buffer.write_string("hello");
    assert_eq!(
        buffer.to_hex_dump(),
        "0x00 0x00 0x00 0x05 0x68 0x65 0x6c 0x6c 0x6f"
    );
}

#[test]
fn test_wpos() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0);
    buffer.set_wpos(1);
    buffer.write_u8(0xFF);
    buffer.write_u8(0x11);
    assert_eq!(buffer.read_u32().unwrap(), 0x00FF1100);
}

#[test]
fn test_rpos() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0x0000FF00);
    buffer.set_rpos(2);
    assert_eq!(buffer.read_u8().unwrap(), 0xFF);
}

#[test]
fn test_to_bytes() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFF);
    assert_eq!(buffer.into_bytes(), vec![0xFF]);
}

#[test]
fn test_from_bytes() {
    let mut buffer = ByteBuffer::from_bytes(&vec![1, 2]);
    assert_eq!(buffer.read_u8().unwrap() + buffer.read_u8().unwrap(), 3);
}

#[test]
fn test_read_bit() {
    let mut buffer = ByteBuffer::from_bytes(&vec![128]);
    let bit1 = buffer.read_bit().unwrap();
    assert_eq!(bit1, true);
    let bit2 = buffer.read_bit().unwrap();
    assert_eq!(bit2, false);
}

#[test]
fn test_cannot_read_bit_outside_data() {
    let mut buffer = ByteBuffer::new();
    let result = buffer.read_bit();
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn test_read_bits() {
    let mut buffer = ByteBuffer::from_bytes(&vec![128]);
    let value = buffer.read_bits(3).unwrap();
    assert_eq!(value, 4);
}

#[test]
fn test_cannot_read_more_than_64_bits() {
    let mut buffer = ByteBuffer::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let result = buffer.read_bits(73);
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::InvalidInput);
}

#[test]
fn test_write_bit() {
    let mut buffer = ByteBuffer::new();
    buffer.write_bit(true);
    buffer.write_bit(true);
    buffer.write_bit(false);
    assert_eq!(buffer.into_bytes()[0], 128 + 64);
}

#[test]
fn test_write_bits() {
    let mut buffer = ByteBuffer::new();
    buffer.write_bits(6, 3); // 110b
    assert_eq!(buffer.into_bytes()[0], 128 + 64);
}

#[test]
fn test_flush_bit() {
    let mut buffer = ByteBuffer::new();
    buffer.write_bit(true);
    buffer.write_i8(1);

    let buffer_result_1 = buffer.into_bytes();
    assert_eq!(buffer_result_1[0], 128);
    assert_eq!(buffer_result_1[1], 1);

    let mut buffer2 = ByteBuffer::from_bytes(&vec![0xFF, 0x01]);
    let bit1 = buffer2.read_bit().unwrap();
    let number1 = buffer2.read_i8().unwrap();

    assert_eq!(bit1, true);
    assert_eq!(number1, 1);
}

#[test]
fn test_read_empty_buffer() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFF);
    let mut res = [];
    buffer.read(&mut res).unwrap();
}

#[test]
fn test_read_exact_buffer() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFF);
    let mut res = [0; 1];
    buffer.read(&mut res).unwrap();
    assert_eq!(res[0], 0xFF);
}

#[test]
fn test_read_larger_buffer() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFF);
    let mut res = [0; 2];
    buffer.read(&mut res).unwrap();
    assert_eq!(res[0], 0xFF);
    assert_eq!(res[1], 0);
}

#[test]
fn test_read_larger_buffer_twice() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFF);
    let mut res = [0; 2];
    buffer.read(&mut res).unwrap();
    // Check for overflow on second read
    buffer.read(&mut res).unwrap();
    assert_eq!(res[0], 0xFF);
    assert_eq!(res[1], 0);
}

#[test]
fn test_write() {
    let mut buffer = ByteBuffer::new();
    buffer.write(&[0x1, 0xFF, 0x45]).unwrap();
    assert_eq!(buffer.read_bytes(3).unwrap(), &[0x1, 0xFF, 0x45]);
}

#[test]
fn test_flush() {
    let mut buffer = ByteBuffer::new();
    buffer.flush().unwrap();
}

#[test]
fn cloning_and_read() {
    let mut buffer = ByteBuffer::new();
    for i in 0..10u8 {
        buffer.write_u8(i);
    }

    let mut clone = buffer.clone();
    for i in 0..10u8 {
        assert_eq!(i, clone.read_u8().unwrap());
    }
}

#[test]
fn cursors_reset() {
    let mut buffer = ByteBuffer::new();
    for i in 0..10u8 {
        buffer.write_u8(i);
        buffer.read_u8().unwrap();
    }

    buffer.reset_cursors();

    for i in 0..10u8 {
        assert_eq!(i, buffer.read_u8().unwrap());
    }
}

#[test]
fn test_debug() {
    let mut buffer = ByteBuffer::from_bytes(&[0x1, 0xFF, 0x45]);
    buffer.read_u8().unwrap();
    let debug_string = format!("{:?}", buffer);
    assert_eq!(
        &debug_string,
        "ByteBuffer { remaining_data: [255, 69], total_data: [1, 255, 69], wpos: 3, rpos: 1, endian: BigEndian }"
    );
}

#[test]
fn test_debug_with_bit_reads() {
    let mut buffer = ByteBuffer::from_bytes(&[0x1, 0xFF, 0x45]);
    let first_four_bits = buffer.read_bits(4).unwrap();
    let debug_string = format!("{:?}", buffer);
    assert_eq!(buffer.get_rpos(), 0);
    let next_four_bits = buffer.read_bits(4).unwrap();
    assert_eq!(buffer.get_rpos(), 1);
    let remaining = buffer.read_bits(16).unwrap();
    assert_eq!(
        &debug_string,
        "ByteBuffer { remaining_data: [255, 69], total_data: [1, 255, 69], wpos: 3, rpos: 0, endian: BigEndian }"
    );
    assert_eq!(first_four_bits, 0);
    assert_eq!(next_four_bits, 1);
    assert_eq!(remaining, 65349);
    assert_eq!(buffer.get_rpos(), 3);
}

macro_rules! overread_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let result = $value;
                assert!(result.is_err());
                let error = result.err().unwrap();
                assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
            }
         )*
    }
}

overread_tests! {
    overread_bytes: ByteBuffer::new().read_bytes(1),
    overread_u8: ByteBuffer::new().read_u8(),
    overread_i8: ByteBuffer::new().read_i8(),
    overread_u16: ByteBuffer::new().read_u16(),
    overread_i16: ByteBuffer::new().read_i16(),
    overread_u32: ByteBuffer::new().read_u32(),
    overread_i32: ByteBuffer::new().read_i32(),
    overread_u64: ByteBuffer::new().read_u64(),
    overread_i64: ByteBuffer::new().read_i64(),
    overread_f32: ByteBuffer::new().read_f32(),
    overread_f64: ByteBuffer::new().read_f64(),
    overread_bit: ByteBuffer::new().read_bit(),
    overread_bits: ByteBuffer::new().read_bits(1),
}
