mod baides {
    pub trait Chain {
        fn head(&self) -> Option<&BdesBlock>;
        fn append(&mut self, data: String, metadata: String) -> bool;
        fn length(&self) -> usize;
    }

    pub struct BdesBlock {
        pub data: String,
        pub metadata: String,
        pub idx: u64,
        pub hash: String,
    }

    pub struct BdesChain {
        pub entity: String,
        pub blocks: Vec<BdesBlock>
    }

    pub fn construct_chain(entity: &str) -> BdesChain {
        let mut vec : Vec<BdesBlock> = Vec::new();
        let mut chain = BdesChain { entity: entity.to_string(), blocks: vec};
        return chain;
    }

    impl Chain for BdesChain {
        fn head(&self) -> Option<&BdesBlock> {
            if self.length()>0 {
                return Some(&self.blocks[self.length()-1]);
            }
            None   
        }

        fn append(&mut self, data: String, metadata: String) -> bool {
            let mut idx: u64 = 0;
            let mut hashable_string: String;
            let current_head = self.head();
            match current_head {
                Some(block) => { 
                    idx = block.idx + 1; 
                    hashable_string = block.hash.clone();
                }, 
                None => {
                    idx = 0;
                    hashable_string = "".to_string();
                },
            }
            // let hash = hmac_sha256::hash("abc")
            let hash = hashable_string;

            let block = BdesBlock { data: data, metadata: metadata, idx: idx, hash: hash };
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
        let metadata = "this is some metadata".to_string();

        chain.append(data, metadata);

        let last_block = chain.head();

        match last_block {
            Some(block) => {
                println!("Data: {}", block.data);
                assert!(block.data.eq("hello world"));
            },
            None => println!("No DATA!")
        }
        
        assert!(chain.length()==1);
    }
}