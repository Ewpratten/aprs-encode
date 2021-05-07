//! Encode APRS data into a string

#![no_std]

// Re-export of arrayvec
pub use arrayvec;

// Modules
pub mod errors;
pub mod aprs;
pub mod ddm;
pub mod stack_str;


// Top-level exports
pub use aprs::header::AprsHeader;


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
