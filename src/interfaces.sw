library;
use ::data_structures::{borrow::BorrowInfo, listing::Listing};

abi NftLend {
    #[storage(read, write), payable]
    fn list_nft(
        asset_id: AssetId,
        nft_id: u64,
        price_per_block: u64,
        collateral: u64,
        collateral_asset_id: AssetId,
    );
    #[storage(read, write), payable]
    fn borrow_nft(
        asset_id: AssetId,
        nft_id: u64,
        duration: u64,
        borrower: Address,
        sent_funds: u64,
    ) -> bool;
    #[storage(read, write), payable]
    fn return_nft(asset_id: AssetId, nft_id: u64) -> bool;
    #[storage(read, write)]
    fn reclaim_nft(asset_id: AssetId, nft_id: u64) -> bool;
}

abi NftInfo {
    #[storage(read)]
    fn get_listing_info(asset_id: AssetId, nft_id: u64) -> Listing;
    #[storage(read)]
    fn get_borrowed_info(asset_id: AssetId, nft_id: u64) -> BorrowInfo;
    #[storage(read)]
    fn get_all_listing() -> Vec<Listing>;
    #[storage(read)]
    fn get_all_borrowed() -> Vec<BorrowInfo>;
}
