pub static EMULATOR_NODE_ID: &'static str = "risk_assessment_emulator";
pub static NODE_ID: &'static str = "risk_assessment_runner";

pub mod emulators;
pub use crate::emulators::*;


pub mod tickers;
pub use crate::tickers::*;