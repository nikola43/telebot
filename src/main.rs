use hex_literal::hex;
use std::{thread, time};
use teloxide::prelude::*;
use web3::types::U256;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let rpc_url = "https://data-seed-prebsc-1-s3.binance.org:8545	";
    let web3httpclient = web3::transports::Http::new(rpc_url).unwrap();
    let web3s = web3::Web3::new(web3httpclient);
    let bot = Bot::from_env().auto_send();
    let chat_id = ChatId(-1001711908500);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(hex!("935aFD78C2BF85EBD5D8473E50d7f9C815328aFb").into());

    let mut initial_balance: U256 = U256::from(0);
    let seconds = time::Duration::from_secs(1);
    while true {
        thread::sleep(seconds);
        let balance = web3s.eth().balance(accounts[0], None).await?;
        println!("Balance of {:?}: {}", accounts[0], balance);

        if balance > initial_balance {
            bot.send_message(chat_id, balance.to_string())
                .await
                .unwrap();
            initial_balance = balance;
        }
    }
    Ok(())
}
