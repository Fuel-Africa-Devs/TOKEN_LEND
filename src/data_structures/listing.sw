library;

pub struct Listing {
    pub lender: Address,
    pub asset_id: AssetId,
    pub borrow_amount: u64,
    pub price_per_block: u64, // Rental fee per block
    pub collateral_amount: u64, // Required collateral
    pub collateral_asset_id: AssetId,
    pub active: bool,
}

impl Listing {
    pub fn new(
        lender: Address,
        asset_id: AssetId,
        borrow_amount: u64,
        price_per_block: u64,
        collateral_amount: u64,
        collateral_asset_id: AssetId,
        active: bool,
    ) -> Self {
        Self {
            lender,
            asset_id,
            borrow_amount,
            price_per_block,
            collateral_amount,
            collateral_asset_id,
            active,
        }
    }

    pub fn default() -> Self {
        Self {
            lender: Address::zero(),
            asset_id: AssetId::default(),
            borrow_amount: 0,
            price_per_block: 0,
            collateral_amount: 0,
            collateral_asset_id: AssetId::default(),
            active: false,
        }
    }
}
