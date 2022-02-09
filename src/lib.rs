mod bdes {
    pub trait Chain {
        fn head(&self) -> BdesBlock;
        fn append(&mut self, block: BdesBlock) -> bool;
        fn length(&self) -> i64;
    }

    pub struct BdesBlock {
        pub data: String,
        pub metadata: String
    }

    pub struct BdesChain {
        pub entity: String,
        pub blocks: Vec<BdesBlock>
    }
    

    impl Chain for BdesChain {
        fn head(&self) -> BdesBlock {
            return BdesBlock { data: "".to_string(), metadata: "".to_string()};
        }

        fn append(&mut self, block: BdesBlock) -> bool {
            self.blocks.push(block);
            return true;
        }

        fn length(&self) -> i64 {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bdes::Chain;

    #[test]
    fn create_chain() {
        let mut chain = bdes::BdesChain::new();

        chain.entity = "Document".to_string();

        let data = "hello world".to_string();
        let metadata = "this is some metadata".to_string();

        let block = bdes::BdesBlock { data: data, metadata: metadata };

        chain.append(block);

        let last_block = chain.head();

        println!("Data: {}", last_block.data);

        assert!(chain.length()==1);
    }
}