use core::fmt::Display;

use arrayvec::CapacityError;

#[derive(Debug)]
pub struct PackError {}

impl From<CapacityError<char>> for PackError {
    fn from(_: CapacityError<char>) -> Self {
        Self {}
    }
}

impl <'a> From<CapacityError<&'a str>> for PackError {
    fn from(_: CapacityError<&'a str>) -> Self {
        Self {}
    }
}

impl Display for PackError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to pack data. Out of space")
    }
}