
use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

//序列化函数
pub fn my_serialize<T: ?Sized>(value: &T)->Vec<u8>
    where T: Serialize,
{
    let serialized =bincode::serialize(value).unwrap();
    serialized
}
//反序列化,传入字节列表的引用
pub fn my_deserialize<'a,T>(bytes: &'a[u8])->T
    where T:Deserialize<'a>,
{
    let deserialized =bincode::deserialize(bytes).unwrap();
    deserialized
}

//求哈希
pub fn get_hash(value : &[u8])->String{
    let mut hasher=Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

pub fn get_sha256()-> Sha3{
   Sha3::sha3_256()

}
#[cfg(test)]
mod tests{
    use super::*;
    #[derive(Serialize,Deserialize,Debug)]
    #[derive(PartialEq)]
    struct Point{
        x:i32,
        y:i32,
    }
    #[test]
    fn coder_works(){
        let point = Point{x:1,y:1};
        let se =my_serialize(&point);
        let de: Point=my_deserialize(&se);
        assert_eq!(de,point);
    }
}