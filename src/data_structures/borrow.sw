library;

pub struct BorrowInfo {
    pub borrower: Address,
    pub starting: u64,
    pub expiration: u64,
    pub collateral: u64,
}

impl BorrowInfo {
    pub fn new(
        borrower: Address,
        starting: u64,
        expiration: u64,
        collateral: u64,
    ) -> Self {
        Self {
            borrower,
            starting,
            expiration,
            collateral,
        }
    }
}
