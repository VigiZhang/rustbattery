extern crate bincode;

use bincode::{serialize, deserialize};

fn main() {
    let target: Option<String> = Some("hello world".to_string());

    let encoded: Vec<u8> = serialize(&target).unwrap();
    let decoded: Option<String> = deserialize(&encoded[..]).unwrap();
    assert_eq!(target, decoded);

    println!("{:?}", encoded);
}
