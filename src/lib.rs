#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate flate2;
extern crate serde_json;
extern crate serde;

// use std::path::Path;
// use std::fs::File;
// use std::io;
use std::io::{Read,Write};
use std::error::Error;

use serde_json::to_string_pretty;
use flate2::read::GzDecoder;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestJson {
    pub my_num : u64,
    pub my_str : String
}

impl TestJson {
    pub fn to_json_bytes(&self) -> Vec<u8> {
        let s : String = to_string_pretty(&self).unwrap();
        s.into_bytes()
    }
    
    pub fn to_json_gz(&self) -> Vec<u8> {
        let mut buf : Vec<u8> = Vec::new();
        {
            let mut encoder = flate2::write::GzEncoder::new(&mut buf, flate2::Compression::Default);
            let _ = serde_json::ser::to_writer_pretty(&mut encoder, &self);
            let _ = encoder.finish();
        }
        buf
    }
    
    pub fn from_gz_bytes<T: Read>(buf : T) -> Result<TestJson, Box<Error>> {
        let mut gzbuf = try!(GzDecoder::new(buf));
        let tj : TestJson = try!(serde_json::de::from_reader(&mut gzbuf));
        Ok(tj)
    }
}

#[test]
fn test_eq() {
    let tj1 = TestJson {
        my_num : 0,
        my_str : String::new()
    };
    
    let tj2 = TestJson::default();
    
    assert_eq!(tj1, tj2);
}

#[test]
fn test_json() {
    let tj = TestJson::default();
    let s : String = to_string_pretty(&tj).unwrap();
    
    assert_eq!(s, "{\n  \"my_num\": 0,\n  \"my_str\": \"\"\n}");
}

#[test]
fn test_gz_serde() {
    let tj = TestJson::default();
    let buf = tj.to_json_gz();
    
    assert_eq!(buf, vec![31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 171, 230, 82, 80, 80, 202, 173, 140, 207, 43, 205, 85, 178, 82, 48, 208, 129, 114, 139, 75, 138, 128, 92, 37, 37, 174, 90, 0, 147, 5, 104, 9, 33, 0, 0, 0]);
    println!("Finished encoding!");
    
    let tj2 = TestJson::from_gz_bytes(&*buf).unwrap();
    assert_eq!(tj, tj2);
}

#[test]
fn test_gz_serde_to_str() {
    let tj = TestJson::default();
    let buf = tj.to_json_gz();
    
    assert_eq!(buf, vec![31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 171, 230, 82, 80, 80, 202, 173, 140, 207, 43, 205, 85, 178, 82, 48, 208, 129, 114, 139, 75, 138, 128, 92, 37, 37, 174, 90, 0, 147, 5, 104, 9, 33, 0, 0, 0]);
    println!("Finished encoding!");
    
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    let mut s : String = String::new();
    gzbuf.read_to_string(&mut s).unwrap();
}

#[test]
fn test_gz() {
    let text = "{\n  \"my_num\": 0,\n  \"my_str\": \"\"\n}";
    let mut buf : Vec<u8> = vec![];
    {
        let mut encoder = flate2::write::GzEncoder::new(&mut buf, flate2::Compression::Default);
        write!(encoder, "{}", text);
        encoder.finish().unwrap();
    }
    println!("Finished encoding!");
    
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    let mut s : String = String::new();
    gzbuf.read_to_string(&mut s).unwrap();
    
    println!("Finished decoding!");
    assert_eq!(text, s);
}

pub fn basic_gzip_str() -> Vec<u8> {
    vec![31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 171, 230, 82, 80, 80, 202, 173, 140, 207, 43, 205, 85, 178, 82, 48, 208, 129, 114, 139, 75, 138, 128, 92, 37, 37, 174, 90, 0, 147, 5, 104, 9, 33, 0, 0, 0]
}

#[test]
fn test_gz_basic() {
    let tj = TestJson::default();
    let buf = basic_gzip_str();
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    println!("Now decoding...");
    let tj2 : TestJson = serde_json::de::from_reader(&mut gzbuf).unwrap();
    assert_eq!(tj, tj2);
}

#[test]
fn test_gz_basic2() {
    let tj = TestJson::default();
    let buf = basic_gzip_str();
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    println!("Now decoding...");
    let tj2 : TestJson = serde_json::de::from_iter(gzbuf.bytes()).unwrap();
    assert_eq!(tj, tj2);
}


#[test]
fn test_gz_basic2b() {
    let tj = TestJson::default();
    let buf = basic_gzip_str();
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    println!("Now decoding...");
    let byteiter = gzbuf.bytes().map(|b| {
        match b {
            Ok(n) => {
                println!("Byte: {} -> {:?}", n, std::char::from_u32(n as u32));
                Ok(n)
            }
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    });
    let tj2 : TestJson = serde_json::de::from_iter(byteiter).unwrap();
    assert_eq!(tj, tj2);
}

#[test]
fn test_gz_basic3() {
    let tj = TestJson::default();
    let buf = basic_gzip_str();
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    println!("Now decoding gzip...");
    let bytevec : Vec<u8> = gzbuf.bytes().map(|b| b.unwrap()).collect();
    println!("Now decoding...");
    let it = bytevec.into_iter().map(|b| Ok(b));
    
    let tj2 : TestJson = serde_json::de::from_iter(it).unwrap();
    assert_eq!(tj, tj2);
}

#[test]
fn test_gz_basic_to_str() {
    let buf = basic_gzip_str();
    let mut gzbuf = GzDecoder::new(&*buf).unwrap();
    let mut s : String = String::new();
    gzbuf.read_to_string(&mut s).unwrap();
}
