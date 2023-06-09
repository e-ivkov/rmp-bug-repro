use serde::{Deserialize, Serialize};

// BUG: Attributes ignored on serialization but used during deserialization
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
enum A {
    B,
    C,
}

fn main() {
    let a = A::B;
    let a_encoded = rmp_serde::to_vec(&a).unwrap();
    let mut cursor = std::io::Cursor::new(a_encoded.clone());
    dbg!(rmpv::decode::read_value(&mut cursor).unwrap());
    let a_decoded: A = rmp_serde::from_slice(&a_encoded).unwrap();
    dbg!(&a_decoded);
    assert_eq!(a_decoded, a);
}
