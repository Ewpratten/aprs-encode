use crate::{AprsHeader, stack_str::PackArrayString};

use self::position::AprsPosition;

pub mod header;
pub mod position;

pub struct AprsPacket<'a> {
    header: AprsHeader<'a>,
    position: Option<AprsPosition>,
    message: Option<&'a str>
}

impl AprsPacket<'_> {

    

}

impl PackArrayString for AprsPacket<'_> {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), crate::errors::PackError> {
        
        // Pack the header
        self.header.pack_into(s)?;

        // Pack the position
        if self.position.is_some(){
            self.position.as_ref().unwrap().pack_into(s)?;
        }

        // Pack WX data 
        // TODO:

        // Pack the message
        if self.message.is_some() {
            s.try_push_str(self.message.as_ref().unwrap())?;
        }


        Ok(())
    }
}
