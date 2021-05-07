use crate::errors::PackError;



pub struct AprsPosition {
    longitude_ddm: f32,
    latitude_ddm: f32
}

impl AprsPosition {

    pub fn new() -> Self {
        Self {
            
        }
    }

    /// Pack the position into an `ArrayString`
    pub fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), PackError> {

        Ok(())
    }
}