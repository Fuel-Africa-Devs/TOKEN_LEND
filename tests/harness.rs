use fuels::accounts::{provider, wallet};
use fuels::types::{Identity, SizedAsciiString};
use fuels::{prelude::*, types::ContractId};
use std::clone;
use std::ops::{Deref, Div};
use std::str::FromStr;
// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/token_lend-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();
    // wallet.get_asset_balance(asset_id)
    // wallet.get_coins(asset_id)
    // println!(
    //     "wallet asset balance {:?} ",
    //     wallet.get_asset_balance(&AssetId::zeroed()).await.unwrap()
    // );
    // println!(
    //     "coins: {:?} ",
    //     wallet.get_asset_balance(&AssetId::zeroed()).await.unwrap()
    // );
    println!("wallet balances{:?}", wallet.get_balances().await.unwrap()); //
    let id = Contract::load_from("./out/debug/token_lend.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.clone(), wallet.clone());

    (instance, id.into(), wallet)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id, user_wallet) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}
//  same as the test in verify contract
// #[tokio::test]
// pub async fn test_can_list_nft() {
//     let (instance, _id, user_wallet) = get_contract_instance().await;
//     let hex_str = "0000000000000000000000000000000000000000000000000000000000000000"; //fuel asset-Id
//     let collateral_asset_id = AssetId::from_str(hex_str).unwrap();
//     let collateral_amount: u64 = 2000;
//     let price_per_block: u64 = 2;
//     let deposit_amount: u64 = 100;
//     println!("fuel asset id: {}", collateral_asset_id);
//     let binding = user_wallet.get_balances().await.unwrap();
//     let initial_token_balance = binding
//         .get(&collateral_asset_id.to_string())
//         .unwrap_or(&deposit_amount);
//     // println!("initial token balance: {}", initial_token_balance);
//     let params = CallParameters::new(deposit_amount, AssetId::zeroed(), 100000000000); //gas
//     let inst = instance
//         .with_account(user_wallet.clone())
//         .methods()
//         .list_token(
//             collateral_asset_id,
//             deposit_amount,
//             price_per_block,
//             collateral_amount,
//             collateral_asset_id,
//         )
//         .call_params(params)
//         .expect("bad callparams")
//         .call()
//         .await
//         .unwrap();
//     println!("logs list nft{:?}", inst.decode_logs().filter_succeeded());

//     let final_token_balance = *user_wallet
//         .get_balances()
//         .await
//         .unwrap()
//         .get(&collateral_asset_id.to_string())
//         .unwrap();
//     println!("final token balance: {}", final_token_balance);
//     println!("{:?}", inst.decode_logs().filter_succeeded());
//     assert!(*initial_token_balance - final_token_balance >= deposit_amount);
// }

#[tokio::test]
async fn test_verify_contract() {
    let (instance, _id, user_one) = get_contract_instance().await;

    let hex_str = "0000000000000000000000000000000000000000000000000000000000000000"; //fuel asset-Id
    let collateral_asset_id = AssetId::from_str(hex_str).unwrap();
    let collateral_amount: u64 = 2000;
    let price_per_block: u64 = 2;
    let deposit_amount: u64 = 100;
    println!("fuel asset id: {}", collateral_asset_id);
    let binding = user_one.get_balances().await.unwrap();
    let initial_token_balance = binding
        .get(&collateral_asset_id.to_string())
        .unwrap_or(&deposit_amount);
    // println!("initial token balance: {}", initial_token_balance);
    let params = CallParameters::new(deposit_amount, AssetId::zeroed(), 100000000000); //gas
    let inst = instance
        .clone()
        .with_account(user_one.clone())
        .methods()
        .list_token(
            collateral_asset_id,
            deposit_amount,
            price_per_block,
            collateral_amount,
            collateral_asset_id,
        )
        .call_params(params)
        .expect("bad callparams")
        .call()
        .await
        .unwrap();
    println!("logs list nft{:?}", inst.decode_logs().filter_succeeded());

    let final_token_balance = *user_one
        .get_balances()
        .await
        .unwrap()
        .get(&collateral_asset_id.to_string())
        .unwrap();
    println!("final token balance: {}", final_token_balance);
    println!("{:?}", inst.decode_logs().filter_succeeded());
    assert!(*initial_token_balance - final_token_balance >= deposit_amount);

    let mut user_two = WalletUnlocked::new_random(None);
    let deposit_to_user_two = 3000;
    let borrow_duration = 1;
    let collateral = 200;
    user_two.set_provider(user_one.provider().unwrap().clone());
    user_one
        .transfer(
            user_two.address(),
            deposit_to_user_two,
            collateral_asset_id,
            TxPolicies::default(),
        )
        .await
        .unwrap();
    let bal_user_one = user_one
        .get_asset_balance(&collateral_asset_id)
        .await
        .unwrap_or_default();
    let bal = user_two
        .get_asset_balance(&collateral_asset_id)
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 bfore borrowing {:?} ", bal_user_one); //
    println!("wallet balances user 2 bfore borrowing {:?} ", bal); //

    // borrow logic now <-user2 borrows some tokens ->
    // let params = CallParameters::new(collateral_amount, collateral_asset_id, 100000000000); //gas
    let provider = user_one.try_provider().unwrap();
    let block_before_borrow = provider.latest_block_height().await.unwrap();
    println!("before {}", block_before_borrow);
    let params = CallParameters::default()
        .with_amount(collateral_amount)
        .with_asset_id(collateral_asset_id);
    let b = instance
        .clone()
        .with_account(user_two.clone())
        .methods()
        .borrow_token(
            collateral_asset_id,
            deposit_amount,
            borrow_duration,
            user_two.clone().address(),
            collateral_amount,
        )
        .call_params(params)
        .expect("bad params")
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1)) // returns 1 token utxo shit --> destoyrs and sed the rest back
        .call()
        .await
        .unwrap();

    println!("logs borrow nft{:?}", b.decode_logs().filter_succeeded());

    let data = instance
        .clone()
        .methods()
        .get_borrowed_info(collateral_asset_id, deposit_amount)
        .call()
        .await
        .unwrap();
    println!("get borrow data{:?} ", data.value);

    let bal = user_two
        .get_asset_balance(&AssetId::zeroed())
        .await
        .unwrap_or_default();
    let bal_user_one = user_one
        .get_asset_balance(&AssetId::zeroed())
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 after borrowing {:?} ", bal_user_one); //
    println!("wallet balances user 2 after borrowing {:?} ", bal);
    let block_time = 20u32; // time between blocks 20 seconds
    let config = NodeConfig {
        block_production: Trigger::Interval {
            block_time: std::time::Duration::from_secs(block_time.into()),
        },
        ..NodeConfig::default()
    }; // block 4
    let origin_block_time = provider.latest_block_time().await.unwrap();
    let blocks_to_produce = 3;
    provider
        .produce_blocks(blocks_to_produce, None)
        .await
        .unwrap();

    let block_when_to_repay = provider.latest_block_height().await.unwrap();

    // let amount_to_pay = (price_per_block * (block_when_to_repay as u64 - block_before_borrow as u64)); //

    let amount_to_pay = deposit_amount
        + (price_per_block * (block_when_to_repay as u64 - block_before_borrow as u64)); //
    println!(
        "amount to pay  {:?} {} {}",
        amount_to_pay, block_when_to_repay, block_before_borrow
    );
    // trying to pay back my debt
    let params = CallParameters::default()
        .with_amount(amount_to_pay)
        .with_asset_id(collateral_asset_id);

    let d = instance
        .clone()
        .with_account(user_two.clone())
        .methods()
        .return_token(collateral_asset_id, deposit_amount)
        .call_params(params)
        .expect("msg")
        .with_variable_output_policy(VariableOutputPolicy::Exactly(2)) //utxo changes made on two addresses.
        .call()
        .await
        .unwrap();
    println!("logs return nfts {:?}", d.decode_logs().filter_succeeded());

    // assert!(response.is_ok(), "Transaction failed: {:?}", response);
    let bal = user_two
        .get_asset_balance(&AssetId::zeroed())
        .await
        .unwrap_or_default();
    let bal_user_one = user_one
        .get_asset_balance(&AssetId::zeroed())
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 after repaying {:?} ", bal_user_one); //

    println!("wallet balances user 2 after repaying {:?} ", bal);

    // reclaim nft
    let claim = instance
        .clone()
        .with_account(user_one.clone())
        .methods()
        .reclaim_token(collateral_asset_id, deposit_amount)
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1)) //utxo changes made on two addresses.
        .call()
        .await
        .unwrap();

    println!(
        "logs return nfts {:?}",
        claim.decode_logs().filter_succeeded()
    );

    let bal = user_two
        .get_asset_balance(&AssetId::zeroed())
        .await
        .unwrap_or_default();
    let bal_user_one = user_one
        .get_asset_balance(&AssetId::zeroed())
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 after claiming {:?} ", bal_user_one); //

    println!("wallet balances user 2 after claiming {:?} ", bal);
}
