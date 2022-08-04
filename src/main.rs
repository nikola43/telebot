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
        let mut bags: Vec<String> = vec!["".into()];
        for _ in 0..50 {
            let borrowed_string = "💰";
            bags.push(borrowed_string.to_string())
        }

        let mut invested: String = "    👉 Invested: ".into();
        let amount = round(wei_to_eth(balance)).to_string();
        invested.push_str(amount.as_str());
        invested.push_str("BNB of 50 BNB\n\n");

        let temp_text: Vec<String> = vec![
            bags.join(""),
            " \n".into(),
            " \n".into(),
            "      🔥🔥🔥New investment🚀🚀🚀\n\n".into(),
            invested,
            "       Fill this form after investment\n https://forms.gle/tj8Hd6TpGa3R8Bf59\n\n".into(),
            "Send your investment to this wallet:\n
            0x9aA7CE0aAb87C1Ef52E0e081CF9A51aad93d3bF0\n\n
            "
            .into(),
        ];

        if balance > initial_balance {
            bot.send_message(chat_id, temp_text.join("")).await.unwrap();
            initial_balance = balance;
        }
    }
    Ok(())
}

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

pub fn eth_to_wei(eth_val: f64) -> U256 {
    let result = eth_val * 1_000_000_000_000_000_000.0;
    let result = result as u128;
    U256::from(result)
}

fn round(x: f64) -> f64 {
    let r = (x * 100.0).round() / 100.0;
    return r;
}
