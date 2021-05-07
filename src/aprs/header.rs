use crate::errors::PackError;

/// Represents the header of an APRS packet
pub struct AprsHeader<'a> {

    /// Source callsign
    pub src_call: &'a str,

    /// Destination callsign
    pub dest_call: &'a str,

    /// Optional packet path specifier
    pub path: Option<&'a [&'a str]>,
}

impl AprsHeader<'_> {

    /// Pack the header into an `ArrayString`
    pub fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), PackError> {
        // Push in the src and dest calls
        s.try_push_str(self.src_call)?;
        s.try_push('>')?;
        s.try_push_str(self.dest_call)?;

        // Handle optional path data
        if self.path.is_some() {
            // Push in all segments
            for segment in self.path.as_ref().unwrap().iter() {
                s.try_push(',')?;
                s.try_push_str(*segment)?;
            }
        }

        Ok(())
    }
}