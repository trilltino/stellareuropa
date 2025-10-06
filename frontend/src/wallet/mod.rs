pub mod freighter;

pub use freighter::{
    connect_wallet,
    is_freighter_available,
    sign_transaction,
    FreighterError,
};
