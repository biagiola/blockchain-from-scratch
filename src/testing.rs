use crate::blockchain::{transaction::Transaction, Serialization};
use crate::blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult};
use sha2::{Digest, Sha256};

// we use the hasher when we want to mining the block
pub fn create_hasher() {
    let mut hasher = Sha256::new();
    hasher.update(b"Hello world\n");

    // println!("hash result is: {:x}", hasher);
    let result = hasher.finalize();
    println!("this is the hasher in decimal: {:?}", result);
    println!("this is the hasher in hexadecimal: {:x?}", result);
}

pub fn create_block(print: bool) {
    let b = Block::new(0, "this is our first block!".to_string().into_bytes());

    if print {
        b.print();
    }
}

pub fn create_block_chain(address: String, print: bool) -> BlockChain {
    // create the chain of blocks
    // by default, the constructor will create the genesis block
    let block_chain = BlockChain::new(address); // TODO: add address

    if print {
        block_chain.print();
    }

    block_chain
}

pub fn get_previous_hash(block_chain: &BlockChain, print: bool) -> Vec<u8> {

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

pub fn create_serialized_tx(print: bool) -> Transaction {
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

pub fn get_block_search_result(result: BlockSearchResult) {
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

pub fn search_blocks(block_chain: &BlockChain, previous_hash: &Vec<u8>, print: bool) {
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
