use chrono::Utc;
use crypto::digest::Digest;
use utils::coder;
use serde::Deserialize;
use serde::Serialize;

use super::*;

const TARGET_HEXS: usize = 4;
#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
pub struct BlockHeader{
    pub time: i64,//当前时间戳，也就是区块创建的时间
    pub tx_hash: String,//交易数据默克尔root hash
    pub pre_hash:String,// 前一个块的哈希，即父哈希
}
//1.取一些公开的数据（比如，如果是 email 的话，它可以是接收者的邮件地址；在比特币中，它是区块头）
// 2.给这个公开数据添加一个计数器。计数器默认从 0 开始
// 3.将 data(数据) 和 counter(计数器) 组合到一起，获得一个哈希
// 4.检查哈希是否符合一定的条件：
// 如果符合条件，结束
// 如果不符合，增加计数器，重复步骤 3-4
#[derive(Debug)]
pub struct Block{
    pub header: BlockHeader,
    pub hash:String,
    pub data:String,//交易数据 区块存储的实际有效信息，也就是交易
    pub nonce: i32,//计数器

}

impl Block{
    fn set_hash(&mut self){

        let header=coder::my_serialize(&(self.header));
        self.hash=coder::get_hash(&header[..]);
    }
    
    pub fn new_block(data:String,pre_hash:String)->Result<Block>{
        let transactions =coder::my_serialize(&data);
        let tx_hash =coder::get_hash(&transactions[..]);
       let time = Utc::now().timestamp();
        let mut block =Block{
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
            },
            hash: "".to_string(),
            data,
            nonce: 0,
        };
        block.set_hash();
        block.run_proof_of_work()?;
        Ok(block)
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.header.pre_hash.clone(),
            self.data.clone(),
            self.header.time,
            TARGET_HEXS,
            self.nonce,
        );
        let bytes = coder::my_serialize(&content);
        Ok(bytes)
    }

    /// Validate validates block's PoW
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = coder::get_sha256();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = Vec::new();
        vec1.resize(TARGET_HEXS, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)
    }

    fn run_proof_of_work(&mut self) -> Result<()> {
        println!("Mining the block containing \"{}\"\n", self.data);
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = coder::get_sha256();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }
}