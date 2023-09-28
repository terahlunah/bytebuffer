use bytebuffer::{ByteBuffer, ByteReader, Endian};
use std::io::ErrorKind;

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
    if cfg!(feature = "half") {
        buffer.write_bf16(half::bf16::from_f32(12.5));
        buffer.write_f16(half::f16::from_f32(12.5));
    }
    buffer.write_f32(0.1);
    buffer.write_f64(0.1);
    buffer.write_string("Hello");
    buffer.write_bit(true);
    buffer.write_bits(4, 3);
    buffer.flush_bits();

    let mut reader = ByteReader::from(buffer.as_bytes());
    let _ = reader.read_bytes(3);
    let _ = reader.read_u8();
    let _ = reader.read_i8();
    let _ = reader.read_u16();
    let _ = reader.read_i16();
    let _ = reader.read_u32();
    let _ = reader.read_i32();
    let _ = reader.read_u64();
    let _ = reader.read_i64();
    if cfg!(feature = "half") {
        let _ = reader.read_bf16();
        let _ = reader.read_f16();
    }
    let _ = reader.read_f32();
    let _ = reader.read_f64();
    let _ = reader.read_string();
    let _ = reader.read_bit();
    let _ = reader.read_bits(3);
}

#[test]
fn test_empty() {
    let mut buffer = ByteBuffer::new();
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.is_empty(), true);
    buffer.write_u8(1);
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.is_empty(), false);
}

#[test]
fn test_len() {
    let mut buffer = ByteBuffer::new();
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.len(), 0);
    buffer.write_u8(1);
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.len(), 1);
    buffer.write_u32(1);
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.len(), 5);
}

#[test]
fn test_u8() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xF0);
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_u8().unwrap(), 0xF0);
}

#[test]
fn test_u16() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u16(0xF0E1);
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_u16().unwrap(), 0xF0E1);
}

#[test]
fn test_u16_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_u16(0xF0E1);
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_endian(Endian::LittleEndian);
    assert_eq!(reader.read_u16().unwrap(), 0xF0E1);
}

#[test]
fn test_u32() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0xF0E1D2C3);
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_u32().unwrap(), 0xF0E1D2C3);
}

#[test]
fn test_u32_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_u32(0xF0E1D2C3);
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_endian(Endian::LittleEndian);
    assert_eq!(reader.read_u32().unwrap(), 0xF0E1D2C3);
}

#[test]
fn test_u64() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u64(0xF0E1D2C3B4A59687);
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_u64().unwrap(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_u64_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_u64(0xF0E1D2C3B4A59687);
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_endian(Endian::LittleEndian);
    assert_eq!(reader.read_u64().unwrap(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_signed() {
    let mut buffer = ByteBuffer::new();
    buffer.write_i8(-1);
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_u8().unwrap(), 0xFF);
}

#[test]
fn test_signed_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_i8(-1);
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_endian(Endian::LittleEndian);
    assert_eq!(reader.read_u8().unwrap(), 0xFF);
}

#[test]
#[cfg(feature = "half")]
fn test_f16() {
    let mut buffer = ByteBuffer::new();
    buffer.write_f16(half::f16::from_f32(0.1));
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_f16().unwrap(), half::f16::from_f32(0.1));
}

#[test]
#[cfg(feature = "half")]
fn test_f16_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_f16(half::f16::from_f32(0.1));
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_endian(Endian::LittleEndian);
    assert_eq!(reader.read_f16().unwrap(), half::f16::from_f32(0.1));
}

#[test]
#[cfg(feature = "half")]
fn test_bf16() {
    let mut buffer = ByteBuffer::new();
    buffer.write_bf16(half::bf16::from_f32(0.1));
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_bf16().unwrap(), half::bf16::from_f32(0.1));
}

#[test]
#[cfg(feature = "half")]
fn test_bf16_little_endian() {
    let mut buffer = ByteBuffer::new();
    buffer.set_endian(Endian::LittleEndian);
    buffer.write_bf16(half::bf16::from_f32(0.1));
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_endian(Endian::LittleEndian);
    assert_eq!(reader.read_bf16().unwrap(), half::bf16::from_f32(0.1));
}

#[test]
fn test_string() {
    let mut buffer = ByteBuffer::new();
    buffer.write_string("hello");
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_string().unwrap(), "hello");
}

#[test]
fn test_mixed() {
    let mut buffer = ByteBuffer::new();
    buffer.write_i16(-1);
    buffer.write_string("hello");
    buffer.write_u64(0xF0E1D2C3B4A59687);
    let mut reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.read_i16().unwrap(), -1);
    assert_eq!(reader.read_string().unwrap(), "hello");
    assert_eq!(reader.read_u64().unwrap(), 0xF0E1D2C3B4A59687);
}

#[test]
fn test_string_overread_protection() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(2);
    buffer.write_bytes(&[0x65]);
    let mut reader = ByteReader::from(buffer.as_bytes());
    let result = reader.read_string();
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn test_to_string() {
    let mut buffer = ByteBuffer::new();
    buffer.write_string("hello");
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(
        reader.to_hex_dump(),
        "0x00 0x00 0x00 0x05 0x68 0x65 0x6c 0x6c 0x6f"
    );
}

#[test]
fn test_rpos() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u32(0x0000FF00);
    let mut reader = ByteReader::from(buffer.as_bytes());
    reader.set_rpos(2);
    assert_eq!(reader.read_u8().unwrap(), 0xFF);
}

#[test]
fn test_as_bytes() {
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(0xFE);
    buffer.write_u8(0xFF);
    let reader = ByteReader::from(buffer.as_bytes());
    assert_eq!(reader.as_bytes(), [0xFE, 0xFF]);
}

#[test]
fn test_from_bytes() {
    let data = vec![1, 2];
    let mut reader = ByteReader::from_bytes(&data);
    assert_eq!(reader.read_u8().unwrap() + reader.read_u8().unwrap(), 3);
}

#[test]
fn test_read_bit() {
    let data = vec![128];
    let mut reader = ByteReader::from_bytes(&data);
    let bit1 = reader.read_bit().unwrap();
    assert_eq!(bit1, true);
    let bit2 = reader.read_bit().unwrap();
    assert_eq!(bit2, false);
}

#[test]
fn test_cannot_read_bit_outside_data() {
    let mut reader = ByteReader::from(&[] as &[u8]);
    let result = reader.read_bit();
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn test_read_bits() {
    let data = vec![128];
    let mut reader = ByteReader::from_bytes(&data);
    let value = reader.read_bits(3).unwrap();
    assert_eq!(value, 4);
}

#[test]
fn test_cannot_read_more_than_64_bits() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut reader = ByteReader::from_bytes(&data);
    let result = reader.read_bits(73);
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::InvalidInput);
}

#[test]
fn test_flush_bit() {
    let data = vec![0xFF, 0x01];
    let mut reader = ByteReader::from_bytes(&data);
    let bit = reader.read_bit().unwrap();
    let number = reader.read_i8().unwrap();

    assert_eq!(bit, true);
    assert_eq!(number, 1);
}

#[test]
fn test_flush_bits() {
    let data = vec![0xFF, 0x01];
    let mut reader = ByteReader::from_bytes(&data);
    let bit1 = reader.read_bit().unwrap();
    let rpos1 = reader.get_rpos();
    /*
     * 11111111 | 00000001
     *  ^
     */
    assert_eq!(bit1, true);
    assert_eq!(rpos1, 0);

    reader.flush_bits();

    let bit2 = reader.read_bit().unwrap();
    let rpos2 = reader.get_rpos();
    /*
     * 11111111 | 00000001
     *             ^
     */
    assert_eq!(bit2, false);
    assert_eq!(rpos2, 1)
}

#[test]
fn cloning_and_read() {
    let mut buffer = ByteBuffer::new();
    for i in 0..10u8 {
        buffer.write_u8(i);
    }
    let reader = ByteReader::from(buffer.as_bytes());

    let mut clone = reader.clone();
    for i in 0..10u8 {
        assert_eq!(i, clone.read_u8().unwrap());
    }
}

#[test]
fn cursors_reset() {
    let mut buffer = ByteBuffer::new();
    for i in 0..10u8 {
        buffer.write_u8(i);
    }

    let mut reader = ByteReader::from(buffer.as_bytes());

    for i in 0..10u8 {
        assert_eq!(i, reader.read_u8().unwrap());
    }

    reader.reset_cursors();

    for i in 0..10u8 {
        assert_eq!(i, reader.read_u8().unwrap());
    }
}

#[test]
fn test_debug() {
    let mut reader = ByteReader::from_bytes(&[0x1, 0xFF, 0x45]);
    reader.read_u8().unwrap();
    let debug_string = format!("{:?}", reader);
    assert_eq!(
        &debug_string,
        "ByteReader { remaining_data: [255, 69], total_data: [1, 255, 69], rpos: 1, endian: BigEndian }"
    );
}

#[test]
fn test_debug_with_bit_reads() {
    let mut reader = ByteReader::from_bytes(&[0x1, 0xFF, 0x45]);
    let first_four_bits = reader.read_bits(4).unwrap();
    let debug_string = format!("{:?}", reader);
    assert_eq!(reader.get_rpos(), 0);
    let next_four_bits = reader.read_bits(4).unwrap();
    assert_eq!(reader.get_rpos(), 1);
    let remaining = reader.read_bits(16).unwrap();
    assert_eq!(
        &debug_string,
        "ByteReader { remaining_data: [255, 69], total_data: [1, 255, 69], rpos: 0, endian: BigEndian }"
    );
    assert_eq!(first_four_bits, 0);
    assert_eq!(next_four_bits, 1);
    assert_eq!(remaining, 65349);
    assert_eq!(reader.get_rpos(), 3);
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
    overread_bytes: ByteReader::from(&[] as &[u8]).read_bytes(1),
    overread_u8: ByteReader::from(&[] as &[u8]).read_u8(),
    overread_i8: ByteReader::from(&[] as &[u8]).read_i8(),
    overread_u16: ByteReader::from(&[] as &[u8]).read_u16(),
    overread_i16: ByteReader::from(&[] as &[u8]).read_i16(),
    overread_u32: ByteReader::from(&[] as &[u8]).read_u32(),
    overread_i32: ByteReader::from(&[] as &[u8]).read_i32(),
    overread_u64: ByteReader::from(&[] as &[u8]).read_u64(),
    overread_i64: ByteReader::from(&[] as &[u8]).read_i64(),
    overread_f32: ByteReader::from(&[] as &[u8]).read_f32(),
    overread_f64: ByteReader::from(&[] as &[u8]).read_f64(),
    overread_bit: ByteReader::from(&[] as &[u8]).read_bit(),
    overread_bits: ByteReader::from(&[] as &[u8]).read_bits(1),
}
