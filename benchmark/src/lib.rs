#![feature(test)]

extern crate test;

use std::vec;

use criterion;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use simd_json;
use simd_json::base::ValueAsContainer;
use simdjson_rust;
use simdjson_rust::{ondemand, prelude::*, Result as SimdResult};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    f64: f64,
    i32: i32,
    str: String,
    arr_str: Vec<String>,
}

fn parse_sedre_json(mystr: String) {
    let my_struct: MyStruct = serde_json::from_str(&mystr).unwrap();
    // visit all data
    my_struct.f64;
    my_struct.i32;
    my_struct.str;
    my_struct.arr_str.iter().for_each(|x| {
        x;
    });
}

fn parse_simdjson_rust(mystr: String) -> SimdResult<()> {
    let mut parser = ondemand::Parser::default();
    let mut doc = parser.iterate(&mystr)?;
    let mut obj = doc.get_object()?;
    obj.at_pointer("/i32")?;
    obj.at_pointer("/f64")?;
    obj.at_pointer("/str")?;
    obj.at_pointer("/arr_str")?.get_array()?.iter().map(|x| x)?;
    Ok(())
}

fn parse_simd_json_to_owned(my_bytes: &mut [u8]) -> SimdResult<()> {
    let data = simd_json::to_owned_value(my_bytes).unwrap();
    data.as_object().unwrap().get("i32").unwrap();
    data.as_object().unwrap().get("f64").unwrap();
    data.as_object().unwrap().get("str").unwrap();
    data.as_object().unwrap().get("arr_str").unwrap();

    Ok(())
}

fn parse_simd_json_to_borrowed(my_bytes: &mut [u8]) -> SimdResult<()> {
    let data = simd_json::to_borrowed_value(my_bytes).unwrap();
    data.as_object().unwrap().get("i32").unwrap();
    data.as_object().unwrap().get("f64").unwrap();
    data.as_object().unwrap().get("str").unwrap();
    data.as_object().unwrap().get("arr_str").unwrap();

    Ok(())
}

const SAMPLE: &str = r#"
{
    "f64": 2.0,
    "i32": 1,
    "str": "it's a string",
    "arr_str": [
               "str1",
               "str2"
            ]
}

"#;
// discussion: https://www.reddit.com/r/learnrust/comments/bjyrgf/feedback_on_blogpost/
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn benchmark_sedre_json(b: &mut Bencher) {
        {
            b.iter(move || parse_sedre_json(SAMPLE.to_string()));
        }
    }

    #[bench]
    fn benchmark_simdjson_rust(b: &mut Bencher) {
        {
            b.iter(move || parse_simdjson_rust(SAMPLE.to_string()));
        }
    }

    #[bench]
    fn benchmark_simd_json_owned(b: &mut Bencher) {
        {
            let mut vec = Vec::from(SAMPLE);
            b.iter(move || parse_simd_json_to_owned(&mut vec));
        }
    }

    #[bench]
    fn benchmark_simd_json_borrowed(b: &mut Bencher) {
        {
            let mut vec = Vec::from(SAMPLE);
            b.iter(move || parse_simd_json_to_borrowed(&mut vec));
        }
    }
}
