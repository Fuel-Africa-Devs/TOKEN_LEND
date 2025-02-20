contract;
pub mod data_structures;
pub mod interfaces;
pub mod events;
pub mod error;
use std::{
    asset::{
        burn,
        mint,
        mint_to,
        transfer,
    },
    auth::msg_sender,
    block::height,
    call_frames::msg_asset_id,
    context::msg_amount,
    hash::Hash,
    string::String,
};
use ::data_structures::{borrow::BorrowInfo, listing::Listing};
use ::interfaces::{NftInfo, NftLend};
use ::events::{Borrowed, Listed, Reclaimed, Repayed};
use ::error::{Borrow, Repay, List};
storage {
    listings: StorageMap<(AssetId, u64), Listing> = StorageMap {},
    borrowed: StorageMap<(AssetId, u64), BorrowInfo> = StorageMap {},
    listing_count: u64 = 0,
    borrowed_count: u64 = 0,
    count_to_listing: StorageMap<u64, (AssetId, u64)> = StorageMap {},
    count_to_borrow: StorageMap<u64, (AssetId, u64)> = StorageMap {},
}
// deposit nft use transfer , depositing src20 use payable
impl NftLend for Contract {
    #[storage(read, write), payable]
    fn list_nft(
        asset_id: AssetId,
        nft_id: u64, //or sub_id
        price_per_block: u64,
        collateral: u64,
        collateral_asset_id: AssetId,
    ) {
        let lender = msg_sender().unwrap();
        let contract_id: ContractId = ContractId::this(); // get contract id <--sheeeh->
        let identity_from_contract_id = Identity::ContractId(contract_id);
        require(
                    nft_id == msg_amount(),
                    List::AssetAmountMismatch,
                );
        match lender {
            Identity::Address(lender_address) => {
                let new_listing = Listing::new(
                    lender_address,
                    asset_id,
                    nft_id,
                    price_per_block,
                    collateral,
                    collateral_asset_id,
                    true,
                );
                storage.listings.insert((asset_id, nft_id), new_listing);
                storage
                    .listing_count
                    .write(storage.listing_count.read() + 1);
                storage
                    .count_to_listing
                    .insert(storage.listing_count.read(), (asset_id, nft_id));
                // tranfer nft to this contract
                // transfer(identity_from_contract_id, asset_id, nft_id);
                log(Listed {
                    lender: lender_address,
                    asset_id: asset_id,
                    nft_id,
                    price_per_block,
                    collateral_amount: collateral,
                    collateral_asset_id,
                    active: true,
                })
            },
            Identity::ContractId(_) => (),
        }
    }

    #[storage(read, write), payable]
    fn borrow_nft(
        asset_id: AssetId,
        nft_id: u64,
        duration: u64,
        borrower: Address,
        sent_funds: u64,
    ) -> bool {
        let listing_to_borrow = storage.listings.get((asset_id, nft_id)).try_read();
        match listing_to_borrow {
            Some(listing) => {
                // let total_cost_in_time = listing.price_per_block * duration; // cost duration
                // let contract_id: ContractId = ContractId::this(); // get contract id <--sheeeh->
                // let identity_from_contract_id = Identity::ContractId(contract_id);
                let mut listing = listing;
                require(
                    listing
                        .collateral_asset_id == msg_asset_id(),
                    Borrow::CollateralMisMatch,
                );
                require(listing.asset_id == asset_id, Borrow::AssetIdMismatch);
                require(
                    sent_funds == msg_amount() && sent_funds == listing
                        .collateral_amount,
                    Borrow::InaccurateCollateral,
                );
                let total_duration = height().as_u64() + duration;
                let starting = height().as_u64();
                let borrower_info = BorrowInfo::new(borrower, starting, total_duration, sent_funds);
                listing.active = false;
                storage.borrowed.insert((asset_id, nft_id), borrower_info);
                storage.listings.insert((asset_id, nft_id), listing);
                storage
                    .borrowed_count
                    .write(storage.borrowed_count.read() + 1);
                storage
                    .count_to_borrow
                    .insert(storage.listing_count.read(), (asset_id, nft_id));

                let new_owner_identity = Identity::Address(borrower);
                transfer(new_owner_identity, listing.asset_id, listing.nft_id);
                log(Borrowed {
                    borrower,
                    asset_id,
                    nft_id,
                    starting,
                    expiration: total_duration,
                    collateral: sent_funds,
                });
                true
            },
            None => false,
        }
    }

    #[storage(read, write), payable]
    fn return_nft(asset_id: AssetId, nft_id: u64) -> bool {
        let borrowed = storage.borrowed.get((asset_id, nft_id)).try_read();
        let mut listings_borrowed = storage.listings.get((asset_id, nft_id)).try_read().unwrap();
        match borrowed {
            Some(borrowed_data) => {
                let mut borrowed_data = borrowed_data;
                require(
                    borrowed_data
                        .expiration <= height()
                        .as_u64(),
                    Repay::BorrowedTImePassed,
                );
                let duration = {
                    if (height().as_u64() > borrowed_data.expiration) {
                        borrowed_data.expiration
                    } else {
                        height().as_u64() - borrowed_data.starting
                    }
                };
                let total_cost_in_time = listings_borrowed.price_per_block * duration; // cost duration
                let borrower = Identity::Address(borrowed_data.borrower);
                require(total_cost_in_time <= msg_amount(), Repay::IncorrectInterest);
                require(
                    listings_borrowed
                        .asset_id == asset_id,
                    Repay::AssetIdMismatch,
                );
                listings_borrowed.active = true;
                borrowed_data.collateral = 0;
                transfer(
                    borrower,
                    listings_borrowed
                        .collateral_asset_id,
                    listings_borrowed
                        .collateral_amount,
                ); // transfer collateral to borrower
                transfer(
                    Identity::Address(listings_borrowed.lender),
                    listings_borrowed
                        .collateral_asset_id,
                    total_cost_in_time,
                ); // transfer interest to lender
                storage.borrowed.insert((asset_id, nft_id), borrowed_data);
                storage
                    .listings
                    .insert((asset_id, nft_id), listings_borrowed);
                // // storage.count_to_borrow.write(storage.listing_count.read(),borrower_info);
                // // storage.count_to_listing.write(storage.listing_count.read(),(asset_id, nft_id) );
                log(Repayed {
                    lender: listings_borrowed.lender,
                    borrower: borrowed_data.borrower,
                    amount_repayed: total_cost_in_time, // Required collateral
                    collateral: borrowed_data.collateral,
                    collateral_asset_id: listings_borrowed.collateral_asset_id,
                });
                true
            },
            None => false,
        }
    }

    #[storage(read, write)]
    fn reclaim_nft(asset_id: AssetId, nft_id: u64) -> bool {
        let listing = storage.listings.get((asset_id, nft_id)).try_read();
        match listing {
            Some(listing) => {
                assert(listing.active == true); // else nft on loan
                transfer(
                    Identity::Address(listing.lender),
                    listing
                        .asset_id,
                    listing
                        .nft_id,
                );
                let default_listing = Listing::default();
                storage.listings.insert((asset_id, nft_id), default_listing);
                log(Reclaimed {
                    lender: listing.lender,
                    asset_id,
                    nft_id,
                });
                true
            },
            None => false,
        }
    }
}

impl NftInfo for Contract {
    #[storage(read)]
    fn get_listing_info(asset_id: AssetId, nft_id: u64) -> Listing {
        storage.listings.get((asset_id, nft_id)).try_read().unwrap()
    }

    #[storage(read)]
    fn get_borrowed_info(asset_id: AssetId, nft_id: u64) -> BorrowInfo {
        storage.borrowed.get((asset_id, nft_id)).try_read().unwrap()
    }

    #[storage(read)]
    fn get_all_listing() -> Vec<Listing> {
        let mut listing: Vec<Listing> = Vec::new();
        let mut counter = 0;
        while counter < storage.listing_count.read() {
            let (asset_id, nft_id) = storage.count_to_listing.get(counter).try_read().unwrap();
            listing.push(storage.listings.get((asset_id, nft_id)).try_read().unwrap());
            counter += 1;
        }
        listing
    }

    #[storage(read)]
    fn get_all_borrowed() -> Vec<BorrowInfo> {
        let mut borrowed_data: Vec<BorrowInfo> = Vec::new();
        let mut counter = 0;
        while counter < storage.borrowed_count.read() {
            let (asset_id, nft_id) = storage.count_to_borrow.get(counter).try_read().unwrap();
            borrowed_data.push(storage.borrowed.get((asset_id, nft_id)).try_read().unwrap());
            counter += 1;
        }
        borrowed_data
    }
}
