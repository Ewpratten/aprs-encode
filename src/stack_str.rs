use crate::errors::PackError;

/// Trait for structs that can pack themselves into the end of a ArrayString
pub trait PackArrayString {

    /// Try to append self to the end of a ArrayString
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), PackError>;
}
