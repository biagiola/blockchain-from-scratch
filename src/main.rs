pub mod blockchain;
use crate::blockchain::{transaction::Transaction, Serialization};
use blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult};
// use sha2::Sha256;
// use transaction::*;

// fn create_hasher() {
//     let mut hasher = Sha256::new();
//     hasher.update(b"Hello world\n");
//     // :x to use the hex format
//     println!("hash result is: {:x}", result);
//     hasher.finalize()
// }

fn _create_block(print: bool) {
    let b = Block::new(0, "this is our first block!".to_string().into_bytes());

    if print {
        b.print();
    }
}

fn create_block_chain(print: bool) -> BlockChain {
    // create the chain of blocks
    // by default, the constructor will create the genesis block
    let block_chain = BlockChain::new();

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

fn verify_serialization(print: bool) -> Transaction {
    // create transaction
    let tx: Transaction = Transaction::new(
        "sender".as_bytes().to_vec(),
        "recipient".as_bytes().to_vec(),
        100,
    );

    // encoded
    let serialized_tx = tx.serialization();

    // decoded
    let deserialized_tx = Transaction::deserialization(&serialized_tx);

    if print {
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

fn main() {
    let mut block_chain: BlockChain = create_block_chain(false);

    // block 1
    let previous_hash: Vec<u8> = get_previous_hash(&block_chain, false);
    block_chain.create_block(1, &previous_hash);

    // block 2
    let previous_hash: Vec<u8> = get_previous_hash(&block_chain, false);
    block_chain.create_block(2, &previous_hash);

    // serializations/deserialization
    let tx: Transaction = verify_serialization(false);

    // add transactions to the (last) block
    // TODO: looks like anything was added actually
    block_chain.add_transaction(&tx);

    // show the entire blocks in the chain
    block_chain.print();

    // search block
    let block_search_result_enum = block_chain.search_block(BlockSearch::SearchByIndex(1));
    get_block_search_result(block_search_result_enum);

    let hash_to_find = previous_hash.clone();
    let result = block_chain.search_block(BlockSearch::SearchByBlockHash(hash_to_find));
    get_block_search_result(result);
}
