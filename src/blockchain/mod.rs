use std::{panic, time::SystemTime};
use std::ops::AddAssign;
use std::time::Instant;
use std::cmp::PartialEq;
use std::ops::Index;
use sha2::{Digest, Sha256};
use transaction::*;

pub mod transaction;

pub trait Serialization<T> {
    fn serialization(&self) -> Vec<u8>;
    fn deserialization(bytes: &Vec<u8>) -> T;
}

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

impl AddAssign<i32> for Block {
    fn add_assign(&mut self, rhs: i32) {
        self.nonce += rhs;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        let self_hash: Vec<u8> = self.hash();
        let other_hash: Vec<u8> = self.hash();
        self_hash == other_hash
    }
}

impl Block {
    // Two kind of methods, one kind static method which not reading
    // or writing into field of our struct, like the constructor.
    // TODO: consider if we need to not make public
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

        // raw transaction
        // println!("transactions: {:?}", self.transactions);

        // encoded transaction
        for (idx, tx) in self.transactions.iter().enumerate() {
            // TODO: verify, to_vec suppose to allow us not lose ownership
            let transaction: Transaction = Transaction::deserialization(&tx.to_vec());

            // we made transaction implement our custom default trait
            println!("the {}'th transaction is: {}", idx, transaction);
        }
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
    blockchain_address: String, // TODO: what represent this address exactly?
}

impl Index<usize> for BlockChain {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        let res: Option<&Block> = self.chain.get(index);
        match res {
            Some(block) => {
                return block;
                // btw, block is a struct, a complex type, if that was a i32 for example, we dont have
                // to deal with reference, in this case our reference is block, coming from the let res variable
            }
            None => {
                panic!("index out of range for the chain") // TODO: consider to change
            }
        }
    }
}

impl BlockChain {
    const DIFFICULTY: usize = 3;
    const MINING_SENDER: &str = "THE BLOCKCHAIN"; // TODO: this must to be an address
    const MINING_REWARD: u64 = 1; // TODO: right now we're not considering floats actually

    pub fn new(address: String) -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
            blockchain_address: address,
        };

        // create genesis block
        let b: Block = Block::new(0, vec![0 as u8, 32]);

        // add the block to the chain
        bc.chain.push(b);

        // mine the block to the chain
        bc.mining();

        bc
    }

    pub fn mining(&mut self) -> bool {
        // when a block is mined, a transaction need to be created to record the value
        // that the blockchain send to the miner
        let tx: Transaction = Transaction::new(
            BlockChain::MINING_SENDER.clone().into(), // sender
            self.blockchain_address.clone().into(),   // reciever
            BlockChain::MINING_REWARD,                // value
        );

        self.add_transaction(&tx);
        self.create_block(&self.last_block().hash());
        true
    }

    pub fn create_block(&mut self, previous_hash: &Vec<u8>) {
        // TODO: consider to use reference and add the lifetime annotation
        // to the new contructor.
        let nonce: i32 = 0;

        let mut b = Block::new(nonce, previous_hash.clone());

        // add the pending transactions to the block
        for tx in self.transaction_pool.iter() {
            b.transactions.push(tx.clone());
        }

        // all the trxs attached to the block needs to be cleared from the pool
        self.transaction_pool.clear();

        // resolve proof of work computation
        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut b);
        let elapsed = now.elapsed();
        println!("compuse time: {:?}", elapsed);
        println!("proof of current block: {:?}", proof_hash);

        self.chain.push(b);
    }

    fn do_proof_of_work(block: &mut Block) -> String {
        const DIFFICULTY: usize = BlockChain::DIFFICULTY;

        loop {
            // create and transform hash to hex
            let hash: Vec<u8> = block.hash();
            let hash_str: String = hex::encode(&hash);

            // check if the hash starts with the required number of zeros
            if hash_str[0..DIFFICULTY] == "0".repeat(DIFFICULTY) {
                return hash_str;
            }

            // increment nonce
            *block += 1;
        }
    }

    pub fn print(&self) {
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} chain {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "=".repeat(25));
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

    pub fn add_transaction(&mut self, tx: &impl Serialization<Transaction>) {
        // detects duplicate
        for tx_in_pool in self.transaction_pool.iter() {
            if *tx_in_pool == tx.serialization() {
                return;
            }
        }

        self.transaction_pool.push(tx.serialization());
    }

    pub fn calculate_total_amount(&self, address: String) -> i64 {
        let mut total_amount: i64 = 0;
        for i in 0..self.chain.len() {
            let block = &self[i];

            for t in block.transactions.iter() {
                let tx: Transaction = Transaction::deserialization(&t.clone());
                let value = tx.value;

                // into() is a trait used for converting one type into another,
                // String implement many type of into trait, such as into<str>, into<i32>
                // into<u64> ..., into<Vec<u8>>
                // So, we need to tell the compiler which trait we should use that is
                // into<Vec<u8>>

                // increase amount
                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.recipient_address {
                    total_amount += value as i64;
                }

                // decrease amount
                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.sender_address {
                    total_amount -= value as i64;
                }
            }
        }
        total_amount
    }
}
