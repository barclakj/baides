mod bdes {
    pub trait Chain {
        fn head(&self) -> &BdesBlock;
        fn append(&mut self, block: BdesBlock) -> bool;
        fn length(&self) -> usize;
    }

    pub struct BdesBlock {
        pub data: String,
        pub metadata: String
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
        fn head(&self) -> &BdesBlock {
            return &self.blocks[self.length()-1];
        }

        fn append(&mut self, block: BdesBlock) -> bool {
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
    use crate::bdes::Chain;

    #[test]
    fn create_chain() {
        let mut chain = bdes::construct_chain("Document");

        let data = "hello world".to_string();
        let metadata = "this is some metadata".to_string();

        let block = bdes::BdesBlock { data: data, metadata: metadata };

        chain.append(block);

        let last_block = chain.head();

        println!("Data: {}", last_block.data);

        assert!(last_block.data.eq("hello world"));

        assert!(chain.length()==1);
    }
}