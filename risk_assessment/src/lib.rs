pub static EMULATOR_NODE_ID: &'static str = "risk_assessment_emulator";
pub static NODE_ID: &'static str = "risk_assessment_runner";
pub static TEST_TICKER_RATE: u64 = 1000; // milliseconds
pub static NUMBER_OF_TEST_CASES: u64 = 20;

pub mod emulators;
pub use crate::emulators::*;

pub mod tickers;
pub use crate::tickers::*;

pub mod models;
pub use crate::models::*;
