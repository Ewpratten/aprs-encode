use crate::{ddm::{DdmLatitude, DdmLongitude}, errors::PackError, stack_str::PackArrayString};

/// Represents the global position of an APRS packet
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AprsPosition {
    longitude: DdmLongitude,
    latitude: DdmLatitude,
}

impl AprsPosition {
    pub fn new(latitude: DdmLatitude, longitude: DdmLongitude) -> Self {
        Self {
            longitude,
            latitude,
        }
    }


}

impl PackArrayString for AprsPosition {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), PackError> {
        
        // pack data
        self.latitude.pack_into(s)?;
        s.try_push('/')?;
        self.longitude.pack_into(s)?;
        
        Ok(())
    }
}