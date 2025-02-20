library;
pub enum List {
    AssetAmountMismatch:(),
}
pub enum Borrow {
    CollateralMisMatch: (),
    AssetIdMismatch: (),
    InaccurateCollateral: (),
}

pub enum Repay {
    BorrowedTImePassed: (),
    IncorrectInterest: (),
    AssetIdMismatch: (),
}
