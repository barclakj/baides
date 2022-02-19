mod baides {
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::fs::File;
    

    pub trait Chain {
        fn head(&self) -> Option<&BdesBlock>;
        fn append(&mut self, key: String, mime_type: String, data: String) -> bool;
        fn length(&self) -> usize;
    }

    pub trait Block {
        fn to_string(&self) -> String;
    }

    pub struct BdesBlock {
        pub idx: u64,
        pub key: String,
        pub data: String,
        pub mime_type: String,
        pub ts: u128,
        pub size: u64,
        pub hash: String,
    }

    pub struct BdesChain {
        pub entity: String,
        pub blocks: Vec<BdesBlock>,
        pub append_file: std::fs::File,
    }

    pub fn construct_chain(entity: &str) -> BdesChain {
        let mut vec : Vec<BdesBlock> = Vec::new();
        let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");

        let mut chain = BdesChain { entity: entity.to_string(), blocks: vec, append_file: file};
        return chain;
    }

    impl Block for BdesBlock {
        fn to_string(&self) -> String {
            let mut _data: String = "^B".to_string();
            _data.push_str(&self.idx.to_string());
            _data.push_str(&self.key.to_string());
            _data.push_str(&self.mime_type.to_string());
            _data.push_str(&self.size.to_string());
            _data.push_str(&self.data.to_string());
            _data.push_str(&self.ts.to_string());
            _data.push_str(&self.hash.to_string());
            _data.push_str(&"$B".to_string());

            return _data; // as_bytes();
        }
    } 

    impl Chain for BdesChain {
        fn head(&self) -> Option<&BdesBlock> {
            if self.length()>0 {
                return Some(&self.blocks[self.length()-1]);
            }
            None   
        }

        fn append(&mut self, key: String, mime_type: String, data: String) -> bool {
            let mut idx: u64 = 0;
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

            let mut _data: String = data.clone();
            let mut _data_hash: String = base64::encode(hmac_sha256::Hash::hash(_data.as_bytes()));
        
            let size: u64 = data.len().try_into().unwrap();
            let current_head = self.head();
            match current_head {
                Some(block) => { 
                    idx = block.idx + 1; 
                    _data_hash.push_str(&block.hash.clone());
                }, 
                None => {
                    idx = 0;
                },
            }
            let hash = base64::encode(hmac_sha256::Hash::hash(_data_hash.as_bytes()));

            let block = BdesBlock { key: key, data: data, idx: idx, hash: hash, ts: ts, mime_type: mime_type, size: size };
            
            self.append_file.write_all(block.to_string().as_bytes()).expect("write failed");
            
            self.blocks.push(block);
            return true;
        }

        fn length(&self) -> usize {
            return self.blocks.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baides::Chain;

    #[test]
    fn create_chain() {
        let mut chain = baides::construct_chain("Document");

        let data = "hello world".to_string();
        let mime_type = "application/octetstream".to_string();

        chain.append("uuid".to_string(), mime_type, data);

        let last_block = chain.head();

        match last_block {
            Some(block) => {
                println!("Data: {}", block.data);
                assert!(block.data.eq("hello world"));
            },
            None => println!("No DATA!")
        }
        
        assert!(chain.length()==1);

        let h = hmac_sha256::Hash::hash(b"ooo");

        let st = base64::encode(h);

        println!("Hashed string: {}", st);
    }
}