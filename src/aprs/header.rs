use crate::{errors::PackError, stack_str::PackArrayString};

/// Represents the header of an APRS packet
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AprsHeader<'a> {

    /// Source callsign
    pub src_call: &'a str,

    /// Destination callsign
    pub dest_call: &'a str,

    /// Optional packet path specifier
    pub path: Option<&'a [&'a str]>,
}

impl PackArrayString for AprsHeader<'_> {

    /// Pack the header into an `ArrayString`
    fn pack_into<const SIZE: usize>(
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


#[cfg(test)]
mod tests {

    use arrayvec::ArrayString;

    use super::*;

    #[test]
    fn test_header_packing() {

        // Build a buffer
        let mut buffer = ArrayString::<128>::new();

        // Add the packet
        let header = AprsHeader{
            src_call: "va3zza",
            dest_call: "n0call",
            path: Some(&["WIDE1-1", "APRSIS"])
        };

        // Pack
        header.pack_into(&mut buffer).unwrap();

        assert_eq!(*buffer, *"va3zza>n0call,WIDE1-1,APRSIS");

    }
}
