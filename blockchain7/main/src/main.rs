use core::mine::Mine;
use core::account::Account;
use core::transaction::Transaction;

fn main() {
    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    println!("-------------------------Mine Info----------------------------");
    let mut mine = Mine::new();

    let mut txs: Vec<Transaction> = Vec::new(); 
    let res = user1.transfer_to(&mut user2, 9, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    let res = user1.transfer_to(&mut user2, 5, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    mine.mining(&mut txs);

    let mut txs: Vec<Transaction> = Vec::new(); 
    let res = user2.transfer_to(&mut user3, 6, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    let res = user2.transfer_to(&mut user3, 3, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    mine.mining(&mut txs);

    println!("-------------------------Miner Info------------------------------");
    mine.miner.miner_info();

    println!("-------------------------Account Info----------------------------");
    let users = vec![&user1, &user2, &user3];
    for u in users {
        u.account_info();
    }

    println!("-------------------------Block Info------------------------------");
    mine.blockchain.block_info();
}
