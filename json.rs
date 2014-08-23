/*
Quick json parsing example in Rust
 */

extern crate serialize;
use serialize::json;

// Automatically generate `Decodable` and `Encodable` trait implementations
#[deriving(Decodable, Encodable, Show)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    let object = TestStruct {
        data_int: 1,
        data_str: "toto".to_string(),
        data_vector: vec![2,3,4,5],
    };

    println!("Object: {}", object);

    // Serialize using `json::encode`
    let encoded = json::encode(&object);

    // Deserialize using `json::decode`
    let decoded: TestStruct = json::decode(encoded.as_slice()).unwrap();

    println!("Decoded: {}", object);
}
