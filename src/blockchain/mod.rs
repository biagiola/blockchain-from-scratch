use sha2::{Digest, Sha256};
use std::time::SystemTime;

pub enum BlockSearch {
    // tag value
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByNonce(i32),
    SearchByTimestamp(u128),
    SearchByTransaction(Vec<u8>),
}

pub enum BlockSearchResult<'a> {
    // 'a indicate the block reference attaching to the tag value
    // has the same life time as the block on the chain
    Success(&'a Block),
    FailOfEmptyBlocks,
    FailOfIndex(usize),
    FailOfPreviousHash(Vec<u8>),
    FailOfBlockHash(Vec<u8>),
    FailOfNonce(i32),
    FailOfTimestamp(u128),
    FailOfTransaction(Vec<u8>),
}

#[derive(Debug)]
pub struct Block {
    pub nonce: i32,
    pub previous_hash: Vec<u8>,
    pub time_stamp: u128,
    pub transactions: Vec<Vec<u8>>,
}

impl Block {
    // Two kind of methos, one kind static method which not reading
    // or writing into field of our struct, like the constructor.
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        Block {
            nonce: nonce,
            previous_hash: previous_hash,
            time_stamp: time_now.as_nanos(),
            transactions: Vec::<Vec<u8>>::new(),
        }
    }

    pub fn print(&self) {
        println!("timestamp: {:}", self.time_stamp);
        println!("nonce: {}", self.nonce);
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions);
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamp.to_be_bytes());

        for tx in self.transactions.iter() {
            bin.extend(tx.clone());
        }

        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, vec![0 as u8; 32]);
        bc
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }

    pub fn print(&self) {
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} chain {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "*".repeat(25));
    }

    pub fn last_block(&self) -> &Block {
        if self.chain.is_empty() {
            panic!("Blockchain is empty");
        }

        if self.chain.len() == 1 {
            return &self.chain[self.chain.len() - 1];
        }

        self.chain.last().unwrap()
    }

    pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        // Check if the chain is empty first
        if self.chain.is_empty() {
            return BlockSearchResult::FailOfEmptyBlocks;
        }

        // Handle SearchByIndex separately since it has different logic
        if let BlockSearch::SearchByIndex(index) = search {
            if index >= self.chain.len() {
                return BlockSearchResult::FailOfIndex(index);
            }
            return BlockSearchResult::Success(&self.chain[index]);
        }

        // For other search types, iterate through the chain
        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(_) => {
                    // This case is already handled above
                    unreachable!()
                }
                BlockSearch::SearchByPreviousHash(ref hash) => {
                    if block.previous_hash == *hash {
                        return BlockSearchResult::Success(block);
                    }
                }
                BlockSearch::SearchByBlockHash(ref hash) => {
                    if block.hash() == *hash {
                        return BlockSearchResult::Success(block);
                    }
                }
                BlockSearch::SearchByNonce(nonce) => {
                    if block.nonce == nonce {
                        return BlockSearchResult::Success(block);
                    }
                }
                BlockSearch::SearchByTimestamp(time_stamp) => {
                    if block.time_stamp == time_stamp {
                        return BlockSearchResult::Success(block);
                    }
                }
                BlockSearch::SearchByTransaction(ref transaction) => {
                    for tx in block.transactions.iter() {
                        if tx == transaction {
                            return BlockSearchResult::Success(block);
                        }
                    }
                }
            }
        }

        // If we reach here, the search failed
        match search {
            BlockSearch::SearchByIndex(_) => unreachable!(), // Already handled above
            BlockSearch::SearchByPreviousHash(hash) => BlockSearchResult::FailOfPreviousHash(hash),
            BlockSearch::SearchByBlockHash(hash) => BlockSearchResult::FailOfBlockHash(hash),
            BlockSearch::SearchByNonce(nonce) => BlockSearchResult::FailOfNonce(nonce),
            BlockSearch::SearchByTimestamp(time_stamp) => BlockSearchResult::FailOfTimestamp(time_stamp),
            BlockSearch::SearchByTransaction(transaction) => BlockSearchResult::FailOfTransaction(transaction),
        }
    }
}
