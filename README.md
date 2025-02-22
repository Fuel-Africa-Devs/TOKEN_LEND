# Token Lend

## Overview
The Token Lending Platform is a decentralized protocol built using the Sway language. It enables users to list, borrow, and return tokens as collateralized loans. The platform ensures secure transactions between lenders and borrowers, with smart contract-based enforcement of repayment terms.

## Features
- **List Tokens**: Users can list their tokens for lending, specifying borrow amount, interest rate (price per block), and required collateral.
- **Borrow Tokens**: Borrowers can secure tokens for a fixed duration by providing collateral and agreeing to pay interest.
- **Return Borrowed Tokens**: Borrowers can return the asset and retrieve their collateral after paying the required interest.
- **Reclaim Collateral**: Lenders can reclaim the collateral if the borrower fails to return the asset within the agreed duration.
- **Query Listings & Borrowed Assets**: The platform allows querying of active listings and borrowed assets.

## Library Usage
```rust
use ::data_structures::{borrow::BorrowInfo, listing::Listing};
```

## Smart Contract Interface

### TokenLend
```rust
abi TokenLend {
    #[storage(read, write), payable]
    fn list_token(
        asset_id: AssetId,
        borrow_amount: u64,
        price_per_block: u64,
        collateral: u64,
        collateral_asset_id: AssetId,
    );
    
    #[storage(read, write), payable]
    fn borrow_token(
        asset_id: AssetId,
        borrow_amount: u64,
        duration: u64,
        borrower: Address,
        sent_funds: u64,
    ) -> bool;
    
    #[storage(read, write), payable]
    fn return_token(asset_id: AssetId, borrow_amount: u64) -> bool;
    
    #[storage(read, write)]
    fn reclaim_token(asset_id: AssetId, borrow_amount: u64) -> bool;
}
```

### TokenInfo
```rust
abi TokenInfo {
    #[storage(read)]
    fn get_listing_info(asset_id: AssetId, borrow_amount: u64) -> Listing;
    
    #[storage(read)]
    fn get_borrowed_info(asset_id: AssetId, borrow_amount: u64) -> BorrowInfo;
    
    #[storage(read)]
    fn get_all_listing() -> Vec<Listing>;
    
    #[storage(read)]
    fn get_all_borrowed() -> Vec<BorrowInfo>;
}
```

## How It Works
1. **Listing an Asset**: A lender lists a token along with borrowing terms.
2. **Borrowing the Asset**: A borrower locks collateral and pays interest to borrow the asset.
3. **Returning the Asset**: The borrower repays the interest and retrieves their collateral.
4. **Reclaiming the Token**: If the borrower fails to return the asset, the lender claims the collateral.

## Deployment
- **Contract Address**: [0xdaf426915b8ea7bcebd2905bd6fa8255e9216a80becb126a9b365f4eaa5d7cc7](`https://app-testnet.fuel.network/contract/0xdaf426915b8ea7bcebd2905bd6fa8255e9216a80becb126a9b365f4eaa5d7cc7`)
- **Network: testnet**: `testnet`

## Benefits
- **Decentralized**: Operates on a trustless smart contract system.
- **Transparent & Secure**: Ensures fair lending and borrowing conditions.
- **Automated Transactions**: Smart contracts manage lending, repayments, and collateral transfers.

## Getting Started
1. Clone the repository.
2. Deploy the smart contract on Fuel.
3. Interact with the contract via blockchain transactions.

## License
This project is open-source and licensed under the MIT License.

<!-- ---
This README serves as a comprehensive guide for users and developers integrating with the Token Lending Platform on Sway. 🚀 -->

