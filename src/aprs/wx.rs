use crate::{errors::PackError, stack_str::PackArrayString};

/// Represents the weather component of an APRS packet
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AprsWx {
    wind_direction_degrees: Option<u8>,
    wind_speed_mph: Option<u8>,
    temperature_f: Option<i8>,
}

impl PackArrayString for AprsWx {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), PackError> {
        // Push wind direction
        if self.wind_direction_degrees.is_some() {
            s.try_push('_')?;
            let direction = self.wind_direction_degrees.as_ref().unwrap();
            s.try_push(('0' as u8 + ((direction / 100) % 10) as u8) as char)?;
            s.try_push(('0' as u8 + ((direction / 10) % 10) as u8) as char)?;
            s.try_push(('0' as u8 + (direction % 10) as u8) as char)?;
        }

        // Push wind speed
        if self.wind_speed_mph.is_some() {
            s.try_push('/')?;
            let speed = self.wind_speed_mph.as_ref().unwrap();
            s.try_push(('0' as u8 + ((speed / 100) % 10) as u8) as char)?;
            s.try_push(('0' as u8 + ((speed / 10) % 10) as u8) as char)?;
            s.try_push(('0' as u8 + (speed % 10) as u8) as char)?;
        }

        // Push temperature
        if self.temperature_f.is_some() {
            s.try_push('t')?;
            let temp = self.temperature_f.as_ref().unwrap();
            s.try_push(('0' as u8 + ((temp / 100) % 10) as u8) as char)?;
            s.try_push(('0' as u8 + ((temp / 10) % 10) as u8) as char)?;
            s.try_push(('0' as u8 + (temp % 10) as u8) as char)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use arrayvec::ArrayString;

    use super::*;

    #[test]
    fn test_wx_packing() {
        // Build a buffer
        let mut buffer = ArrayString::<128>::new();

        // Add the packet
        let data = AprsWx {
            wind_speed_mph: Some(10),
            wind_direction_degrees: Some(35),
            temperature_f: Some(65),
        };

        // Pack
        data.pack_into(&mut buffer).unwrap();

        assert_eq!(*buffer, *"_035/010t065");
    }
}
