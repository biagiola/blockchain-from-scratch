pub mod blockchain;
use blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult};
use sha2::Sha256;

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
    // let b = Block::new(0, "this is our first block!".to_string().into_bytes());
    // b.print();
    // println!("the first block is: {:#?}", b)

    // let mut hasher = Sha256::new();
    // hasher.update(b"Hello world\n");
    // let result = hasher.finalize();
    // // :x to use the hex format
    // println!("hash result is: {:x}", result);

    let mut block_chain = BlockChain::new();
    println!("Block chain: {:?}", block_chain);

    // compute the previus hash
    let previous_hash = block_chain.last_block().hash();
    let hash_to_find = previous_hash.clone();

    block_chain.create_block(1, previous_hash);

    // shadow the previous new hash, again
    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(2, previous_hash);
    block_chain.print();

    let result = block_chain.search_block(BlockSearch::SearchByIndex(1));
    get_block_search_result(result);
    let result = block_chain.search_block(BlockSearch::SearchByIndex(5));
    get_block_search_result(result);
    let result = block_chain.search_block(BlockSearch::SearchByBlockHash(hash_to_find));
    get_block_search_result(result);
}
