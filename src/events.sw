library;

pub struct Listed {
    pub lender: Address,
    pub asset_id: AssetId,
    pub borrow_amount: u64,
    pub price_per_block: u64, // Rental fee per block
    pub collateral_amount: u64, // Required collateral
    pub collateral_asset_id: AssetId,
    pub active: bool,
}

pub struct Borrowed {
    pub borrower: Address,
    pub asset_id: AssetId,
    pub borrow_amount: u64,
    pub starting: u64,
    pub expiration: u64,
    pub collateral: u64,
}

pub struct Repayed {
    pub lender: Address,
    pub borrower: Address,
    pub amount_repayed: u64, // Required collateral
    pub collateral: u64,
    pub collateral_asset_id: AssetId,
}

pub struct Reclaimed {
    pub lender: Address,
    pub asset_id: AssetId,
    pub borrow_amount: u64,
}
