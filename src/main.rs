use chrono::Utc;
use log::{error, warn};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
pub struct App {
    pub blocks: Vec<Block>,
}

const DIFFICULTY_PREFIX: &str = "00";

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

impl App {
    fn new() -> Self {
        Self{
            blocks: vec![],
        }
    }

    fn genesis(&mut self) {
        let genesis_block = Block{
            id: 0,

            previous_hash: String::from("genesis"),

            data: String::from("Data"), 

            nonce: 1,

            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),

            timestamp: Utc::now().timestamp(),
        };

        self.blocks.push(genesis_block)
    }

    pub fn try_add_block(&mut self, block: Block){
        let latest_block = self.blocks.last().unwrap();

        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("Could not add last block!")
        }
    }

    fn is_block_valid(&self, block:&Block ,previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id {} has wrong previous hash",block.id);

            false
        } else if !hash_to_binary_representation(
                &hex::decode(&block.hash).expect("can decode from hex"),
            ).starts_with(DIFFICULTY_PREFIX) {
            
            warn!("Block with id {} has invalid difficulty",block.id);
            false
        } else if block.id != previous_block.id +1 {
            warn!("The block id of previos block {} does not match ", block.id);

            false
        } else if hex::encode(calculate_hash(
                block.id,
                block.timestamp,
                &block.previous_hash,
                &block.data,
                block.nonce,
                )) != block.hash {

            warn!("block with id {} has invalid hash", block.id);
        
            false
        } 
        else {
            true
        }
    }

    pub fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i==0 {
                continue;
            }

            let first = chain.get(i - 1).expect("Has to exist");
            let second = chain.get(i).expect("has to exist");
             
            if !self.is_block_valid(second, first) {
              return  false;
            }
        } 

        true
    } 
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id:u64,

    pub hash: String,

    pub previous_hash: String,

    pub timestamp: i64,

    pub data: String,

    pub nonce: u64,
}
