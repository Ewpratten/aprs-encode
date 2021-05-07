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

#[cfg(test)]
mod tests {

    use arrayvec::ArrayString;

    use super::*;

    #[test]
    fn test_position_packing() {

        // Build a buffer
        let mut buffer = ArrayString::<128>::new();

        // Add the packet
        let data = AprsPosition {
            latitude: 42.981312.into(),
            longitude: (-81.257472).into()
        };

        // Pack
        data.pack_into(&mut buffer).unwrap();

        assert_eq!(*buffer, *"4258.87N/08115.44W");

    }
}