pub mod blockchain;
use crate::blockchain::{transaction::Transaction, Serialization};
use blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult};
use sha2::{Digest, Sha256};
// use transaction::*;

fn create_hasher() {
    let mut hasher = Sha256::new();
    hasher.update(b"Hello world\n");
    // :x to use the hex format
    // println!("hash result is: {:x}", hasher);
    let result = hasher.finalize();
    println!("this is the hasher: {:?}", result);
    println!("this is the hasher: {:x?}", result);
}

fn create_block(print: bool) {
    let b = Block::new(0, "this is our first block!".to_string().into_bytes());

    if print {
        b.print();
    }
}

fn create_block_chain(address: String, print: bool) -> BlockChain {
    // create the chain of blocks
    // by default, the constructor will create the genesis block
    let block_chain = BlockChain::new(address); // TODO: add address

    if print {
        block_chain.print();
    }

    block_chain
}

fn get_previous_hash(block_chain: &BlockChain, print: bool) -> Vec<u8> {

    // Display the hash of the block.
    // Right now there is only one block.
    let previous_hash = block_chain.last_block().hash();

    // previous_hash is actually the message wrote in the block
    // in a Sha256 using its data for that.
    if print {
        println!("previous_hash: {:?}\n", previous_hash);
    }

    previous_hash
}

fn create_serialized_tx(print: bool) -> Transaction {
    // create transaction
    let tx: Transaction = Transaction::new(
        "sender".as_bytes().to_vec(),
        "recipient".as_bytes().to_vec(),
        100,
    );

    if print {
        // encoded
        let serialized_tx = tx.serialization();

        // decoded
        let deserialized_tx = Transaction::deserialization(&serialized_tx);

        println!("Transaction using modified display trait: {}", tx);
        println!("Transaction using non-modified debug trait: {:#?}", tx);

        println!("serialized transaction: {:?}\n", serialized_tx);
        println!("deserialized transaction: {:#?}\n", deserialized_tx);
    }

    // TODO: implement PartialEq and/or Eq to serialized_tx and deserialized_tx
    tx
}

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!("find given block: {:?}", block);
        }
        BlockSearchResult::FailOfIndex(index) => {
            println!("fail to find block with given index: {}", index);
        }
        BlockSearchResult::FailOfEmptyBlocks => {
            println!("the block chain is empty");
        }
        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!("not block has given previous hash as: {:?}", hash);
        }
        BlockSearchResult::FailOfBlockHash(hash) => {
            println!("not block has given hash as: {:?}", hash);
        }
        BlockSearchResult::FailOfNonce(nonce) => {
            println!("no block has nonce as: {}", nonce);
        }
        BlockSearchResult::FailOfTimestamp(time_stamp) => {
            println!("no block has timestamp as: {}", time_stamp);
        }
        BlockSearchResult::FailOfTransaction(transaction) => {
            println!("no block has transaction as: {:?}", transaction);
        }
    }
}

fn search_blocks(block_chain: &BlockChain, previous_hash: &Vec<u8>, print: bool) {
    // search by index
    let block_search_result_enum = block_chain.search_block(BlockSearch::SearchByIndex(1));

    // search by hash
    let hash_to_find = previous_hash.clone();
    let result = block_chain.search_block(BlockSearch::SearchByBlockHash(hash_to_find));

    if print {
        get_block_search_result(block_search_result_enum);
        get_block_search_result(result);
    }
}

fn main() {
    let my_blockchain_address: &str = "my blockchain address";
    let mut block_chain: BlockChain = BlockChain::new(my_blockchain_address.into());
    // block_chain.print();

    // create transactions
    // let trx_1 = Transaction::new("A".into(), "B".into(), 1);
    let trx_1 = Transaction::new("A".to_string().into(), "B".to_string().into(), 1);

    // let trx_2 = Transaction::new("C".into(), "D".into(), 2);
    // let trx_3 = Transaction::new("X".into(), "Y".into(), 3);

    // add transactions to the pool and mint
    block_chain.add_transaction(&trx_1);
    // block_chain.mining();

    // add more transactions
    // block_chain.add_transaction(trx_2);
    // block_chain.add_transaction(trx_3);

    block_chain.mining();
    block_chain.print();

    println!(
        "value for miner: {}",
        block_chain.calculate_total_amount(my_blockchain_address.to_string())
    );
    println!(
        "value for A: {}",
        block_chain.calculate_total_amount("A".to_string())
    );
    println!(
        "value for B: {}",
        block_chain.calculate_total_amount("B".to_string())
    );
    // println!("value for D: {}", block_chain.calculate_total_amount("D".to_string()));

}

// // _create_hasher();

// let address: &str = "0xFake_address";
// let mut block_chain: BlockChain = create_block_chain(address, false);

// // block 1
// let previous_hash: Vec<u8> = get_previous_hash(&block_chain, false);
// block_chain.create_block(1, &previous_hash);

// // block 2
// let previous_hash: Vec<u8> = get_previous_hash(&block_chain, false);
// block_chain.create_block(2, &previous_hash);

// // serializations/deserialization
// let tx: Transaction = create_serialized_tx(false);

// // add transactions to the (last) block
// block_chain.add_transaction(&tx);

// // so block 3 is going to grab all the current trxs available from the pool
// block_chain.create_block(3, &block_chain.last_block().hash());

// // show the entire blocks in the chain
// // block_chain.print();

// // search block
// search_blocks(&block_chain, &previous_hash, false);

// // block_chain.print();

// // now we can make comparations between blocks
// let block1: Block = Block::new(0, "previous hash".as_bytes().to_vec());
// let block2: Block = Block::new(0, "previous hash".as_bytes().to_vec());
// println!("block1 == block2: {:?}", block1 == block2);

// // we can access to an specific block
// let mut block_chain: BlockChain = create_block_chain(false);
// let block: &Block = &block_chain[0];
// println!("the first block is: {:?}", block);
