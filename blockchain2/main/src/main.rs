use core::blockchain::BlockChain;

fn main() {
    println!("-------------------------Mine Info----------------------------");
    let mut bc = BlockChain::new();

    let tx = "0xabcd -> 0xabce: 5 btc".to_string();
    bc.add_block(tx);

    let tx = "0xabcd -> 0xabcf: 2.5 btc".to_string();
    bc.add_block(tx);

    println!("-------------------------Block Info------------------------------");
    bc.block_info();
}
