use chrono::Utc;
use serde::{Deserialize, Serialize};
use utils::coder;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String, // 交易数据的hash(merkle root hash)
    pub pre_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub data: String, // 交易数据
    pub hash: String,
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&self.header);
        self.hash = coder::get_hash(&header);
    }

    pub fn new_block(data: String, pre_hash: String) -> Self {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions);
        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
            },
            data: data,
            hash: "".to_string(),
        };
        block.set_hash();
        block
    }
}
