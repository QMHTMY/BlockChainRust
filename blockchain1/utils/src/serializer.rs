use bincode;
use serde::Serialize;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8> 
    where T: Serialize,
{
    let seialized = bincode::serialize(value).unwrap();
    seialized
}

pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}
