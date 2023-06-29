use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn my_serialize<T: ?Sized>(t: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(t).unwrap()
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn it_works() {
        let result = Point{ x: 2, y: 23 };
        let bytes = my_serialize(&result);
        let point: Point = my_deserialize(&bytes);
        println!("{:?}", point);
        println!("{:?}", bytes);
        assert_eq!(point.x, 2);
        assert_eq!(point.y, 23);
        assert_eq!(result, point);
    }
}
