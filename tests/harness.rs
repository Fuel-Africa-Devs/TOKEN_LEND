use fuels::accounts::{provider, wallet};
use fuels::types::{Identity, SizedAsciiString};
use fuels::{prelude::*, types::ContractId};
use std::clone;
use std::ops::Deref;
use std::str::FromStr;
// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/nft_lend-abi.json"
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
    println!(
        "wallet asset balance {:?}",
        wallet.get_asset_balance(&AssetId::zeroed()).await.unwrap()
    );
    println!(
        "coins: {:?}",
        wallet.get_coins(AssetId::zeroed()).await.unwrap()
    );
    println!("wallet balances{:?}", wallet.get_balances().await.unwrap()); //
    let id = Contract::load_from("./out/debug/nft_lend.bin", LoadConfiguration::default())
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

// #[tokio::test]
pub async fn test_can_list_nft() -> (AssetId, u64) {
    let (instance, _id, user_wallet) = get_contract_instance().await;
    let hex_str = "0000000000000000000000000000000000000000000000000000000000000000"; //fuel asset-Id
    let collateral_asset_id = AssetId::from_str(hex_str).unwrap();
    let collateral_amount: u64 = 2000;
    let price_per_block: u64 = 2;
    let deposit_amount: u64 = 100;
    println!("fuel asset id: {}", collateral_asset_id);
    let binding = user_wallet.get_balances().await.unwrap();
    let initial_token_balance = binding
        .get(&collateral_asset_id.to_string())
        .unwrap_or(&deposit_amount);
    // println!("initial token balance: {}", initial_token_balance);
    let params = CallParameters::new(deposit_amount, AssetId::zeroed(), 100000000000); //gas
    let inst = instance
        .with_account(user_wallet.clone())
        .methods()
        .list_nft(
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
    let final_token_balance = *user_wallet
        .get_balances()
        .await
        .unwrap()
        .get(&collateral_asset_id.to_string())
        .unwrap();
    println!("final token balance: {}", final_token_balance);
    println!("{:?}", inst.decode_logs().filter_succeeded());
    assert!(*initial_token_balance - final_token_balance >= deposit_amount);
    (collateral_asset_id, deposit_amount)
}

#[tokio::test]
async fn test_can_borrow() {
    let (asset_id, amount) = test_can_list_nft().await;
    let (instance, _id, user_one) = get_contract_instance().await;
    let mut user_two = WalletUnlocked::new_random(None);
    let deposit_to_user_two = 3000;
    let borrow_duration = 1;
    let collateral = 2000;
    user_two.set_provider(user_one.provider().unwrap().clone());
    user_one
        .transfer(
            user_two.address(),
            deposit_to_user_two,
            AssetId::zeroed(),
            TxPolicies::default(),
        )
        .await
        .unwrap();
    let bal_user_one = user_one
        .get_coins(AssetId::zeroed())
        .await
        .unwrap_or_default();
    let bal = user_two
        .get_coins(AssetId::zeroed())
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 bfore borrowing{:?}", bal_user_one); //
    println!("wallet balances user 2 bfore borrowing{:?}", bal); //

    // borrow logic now <-user2 borrows some tokens ->
    let params = CallParameters::new(collateral, AssetId::zeroed(), 100000000000); //gas
    instance
        .clone()
        .with_account(user_two.clone())
        .methods()
        .borrow_nft(
            asset_id,
            amount,
            borrow_duration,
            user_two.clone().address(),
            collateral,
        )
        .call_params(params)
        .expect("bad params")
        .call()
        .await
        .unwrap();
    let bal = user_two
        .get_coins(AssetId::zeroed())
        .await
        .unwrap_or_default();
    let bal_user_one = user_one
        .get_coins(AssetId::zeroed())
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 after borrowing{:?}", bal_user_one); //
    println!("wallet balances user 2 after borrowing{:?}", bal);
    let provider = user_one.try_provider().unwrap();
    let block_time = 2u32;
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

    // trying to pay back my debt
    let params = CallParameters::new(3, AssetId::zeroed(), 100000000000); //gas

    instance
        .with_account(user_two.clone())
        .methods()
        .return_nft(asset_id, amount)
        .call_params(params)
        .expect("msg")
        .call()
        .await
        .unwrap();
    // assert!(response.is_ok(), "Transaction failed: {:?}", response);
    let bal = user_two
        .get_coins(AssetId::zeroed())
        .await
        .unwrap_or_default();
    let bal_user_one = user_one
        .get_coins(AssetId::zeroed())
        .await
        .unwrap_or_default();
    println!("wallet balances user 1 after repaying{:?}", bal_user_one); //

    println!("wallet balances user 2 after repaying {:?}", bal);
}
