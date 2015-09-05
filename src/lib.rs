extern crate byteorder;

use byteorder::{ByteOrder, BigEndian};
use std::io::Write;

pub struct ByteBuffer {
    data : Vec<u8>,
    wpos : usize,
    rpos : usize,
}

impl ByteBuffer {

    pub fn new() -> ByteBuffer {
        ByteBuffer { 
            data : vec![],
            wpos : 0,
            rpos : 0 
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.wpos = 0;
        self.rpos = 0;
    }

    pub fn resize(&mut self, size : usize) {
        let diff = size - self.data.len();
        if diff > 0 {
            self.data.extend(std::iter::repeat(0).take(diff))
        }
    }

    // Write operations

    pub fn write_bytes(&mut self, array : &[u8]) {

        let size = array.len() + self.wpos;

        if size > self.data.len() {
            self.resize(size);
        }

        for v in array {
            self.data[self.wpos] = *v;
            self.wpos += 1;
        }
    }

    pub fn write_u8(&mut self, val : u8) {
        self.write_bytes(&[val]);
    }

    pub fn write_i8(&mut self, val : i8) {
        self.write_u8(val as u8);
    }

    pub fn write_u16(&mut self, val : u16) {
        let mut buf = [0; 2];
        BigEndian::write_u16(&mut buf, val);
        self.write_bytes(&buf);
    }

    pub fn write_i16(&mut self, val : i16) {
        self.write_u16(val as u16);
    }

    pub fn write_u32(&mut self, val : u32) {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, val);
        self.write_bytes(&buf);
    }

    pub fn write_i32(&mut self, val : i32) {
        self.write_u32(val as u32);
    }

    pub fn write_u64(&mut self, val : u64) {
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, val);
        self.write_bytes(&buf);
    }

    pub fn write_i64(&mut self, val : i64) {
        self.write_u64(val as u64);
    }

    pub fn write_f32(&mut self, val : f32) {
        let mut buf = [0; 4];
        BigEndian::write_f32(&mut buf, val);
        self.write_bytes(&buf);
    }

    pub fn write_f64(&mut self, val : f64) {
        let mut buf = [0; 8];
        BigEndian::write_f64(&mut buf, val);
        self.write_bytes(&buf);
    }

    pub fn write_string(&mut self, val : &str) {
        self.write_u32(val.len() as u32);
        self.write_bytes(val.as_bytes());
    }

    // Read operations

    pub fn read_bytes(&mut self, size : usize) -> Vec<u8> {
        assert!(self.rpos + size <= self.data.len());
        let range = self.rpos..self.rpos+size;
        let mut res = Vec::<u8>::new();
        res.write(&self.data[range]).unwrap();
        self.rpos += size;
        res
    }

    pub fn read_u8(&mut self) -> u8 {
        assert!(self.rpos < self.data.len());
        let pos = self.rpos;
        self.rpos += 1;
        self.data[pos]
    }

    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    pub fn read_u16(&mut self) -> u16 {
        assert!(self.rpos + 2 <= self.data.len());
        let range = self.rpos..self.rpos+2;
        self.rpos += 2;
        BigEndian::read_u16(&self.data[range])
    }

    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    pub fn read_u32(&mut self) -> u32 {
        assert!(self.rpos + 4 <= self.data.len());
        let range = self.rpos..self.rpos+4;
        self.rpos += 4;
        BigEndian::read_u32(&self.data[range])
    }

    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    pub fn read_u64(&mut self) -> u64 {
        assert!(self.rpos + 8 <= self.data.len());
        let range = self.rpos..self.rpos+8;
        self.rpos += 8;
        BigEndian::read_u64(&self.data[range])
    }

    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }

    pub fn read_f32(&mut self) -> f32 {
        assert!(self.rpos + 4 <= self.data.len());
        let range = self.rpos..self.rpos+4;
        self.rpos += 4;
        BigEndian::read_f32(&self.data[range])
    }

    pub fn read_f64(&mut self) -> f64 {
        assert!(self.rpos + 8 <= self.data.len());
        let range = self.rpos..self.rpos+8;
        self.rpos += 8;
        BigEndian::read_f64(&self.data[range])
    }

    pub fn read_string(&mut self) -> String {
        let size = self.read_u32();
        String::from_utf8(self.read_bytes(size as usize)).unwrap()
    }

    // Other

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for b in &self.data {
            str = str + &format!("0x{:01$x} ", b, 2);
        }
        str.pop();
        str
    }

    pub fn get_rpos(&self) -> usize {
        self.rpos
    }

    pub fn set_rpos(&mut self, rpos : usize) {
        self.rpos = std::cmp::min(rpos, self.data.len());
    }

    pub fn get_wpos(&self) -> usize {
        self.wpos
    }

    pub fn set_wpos(&mut self, wpos : usize) {
        self.wpos = std::cmp::min(wpos, self.data.len());
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.to_vec()
    }

}

