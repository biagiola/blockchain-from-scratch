pub mod blockchain;
use crate::blockchain::{transaction::Transaction, Serialization};
use blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult};
// use transaction::*;

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
