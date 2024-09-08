use std::thread;
use std::time::Duration;
use core::blockchain;
pub type Result<T> = std::result::Result<T, failure::Error>;
fn main()-> Result<()> {

    let mut bc =blockchain::BlockChain::new_blockchain();

    println!("start mining....");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("a -> b:5 btc".to_string())?;
    println!("produce a block....");

    println!("start mining....");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("c -> d: 3 btc".to_string())?;
    println!("produce a block....");

    for b in bc.blocks{
        println!("+++++++++++++++++");
        println!("{:#?}",b);
        println!("");
    }
    Ok(())

}
