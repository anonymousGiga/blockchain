use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

// pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>> 
// where
//     T: Serialize, 
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8> 
    where T: Serialize,
{
    let seialized = bincode::serialize(value).unwrap();
    seialized
}

// pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T> 
// where
//     T: Deserialize<'a>, 
pub fn my_deserialize<'a, T>(bytes: &'a[u8]) -> T 
    where T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

//for test
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_serialize, my_deserialize};

    #[test]
    fn coder_works() {
        let point = Point{ x: 1, y: 1 };
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);

        assert_eq!(de, point);
    }
}

