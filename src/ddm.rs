use crate::stack_str::PackArrayString;

/// Description of an angle in Degrees + Minutes
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct DegreeMinutes {
    pub degrees: i64,
    pub minutes: f32,
}

impl From<f32> for DegreeMinutes {
    // Create DegreeMinutes from a number of degrees
    fn from(dd: f32) -> Self {
        let degrees = (dd
            - match dd % 1.0 >= 0.0 {
                true => dd % 1.0,
                false => 1.0 + dd % 1.0,
            }) as i64;
        return DegreeMinutes {
            degrees,
            minutes: (dd - degrees as f32) * 60.0,
        };
    }
}

impl PackArrayString for DegreeMinutes {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), crate::errors::PackError> {
        // Handle Degrees
        s.try_push(('0' as u8 + (self.degrees / 10) as u8) as char)?;
        s.try_push(('0' as u8 + (self.degrees % 10) as u8) as char)?;

        // Handle Minutes
        s.try_push(('0' as u8 + (self.minutes / 10.0) as u8) as char)?;
        s.try_push(('0' as u8 + (self.minutes % 10.0) as u8) as char)?;

        // Push decimal
        s.try_push('.')?;

        // Handle Minute decimals
        s.try_push(('0' as u8 + ((self.minutes * 10.0) % 10.0) as u8) as char)?;
        s.try_push(('0' as u8 + ((self.minutes * 100.0) % 10.0) as u8) as char)?;

        Ok(())
    }
}

/// Enum representation of the 4 cardinal Directions
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl PackArrayString for CardinalDirection {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), crate::errors::PackError> {
        s.try_push(match self {
            Self::North => 'N',
            Self::East => 'E',
            Self::South => 'S',
            Self::West => 'W',
        })?;
        Ok(())
    }
}

/// Representation of an angle in Degree/Decimal/Minutes
pub trait DdmAngle {
    fn get_direction(&self) -> CardinalDirection;
    fn get_degree_minutes(&self) -> DegreeMinutes;
}

/// Latitude in DDM
pub struct DdmLatitude {
    ddm: DegreeMinutes,
    direction: CardinalDirection,
}

impl From<f32> for DdmLatitude {
    fn from(latitude: f32) -> Self {
        Self {
            ddm: match latitude < 0.0 {
                true => latitude * -1.0,
                false => latitude,
            }
            .into(),
            direction: match latitude < 0.0 {
                true => CardinalDirection::South,
                false => CardinalDirection::North,
            },
        }
    }
}

impl DdmAngle for DdmLatitude {
    fn get_direction(&self) -> CardinalDirection {
        self.direction
    }

    fn get_degree_minutes(&self) -> DegreeMinutes {
        self.ddm
    }
}

impl PackArrayString for DdmLatitude {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), crate::errors::PackError> {
        // Push DDM
        self.ddm.pack_into(s)?;

        // Push Direction
        self.direction.pack_into(s)?;

        Ok(())
    }
}


/// Longitude in DDM
pub struct DdmLongitude {
    ddm: DegreeMinutes,
    direction: CardinalDirection,
}

impl From<f32> for DdmLongitude {
    fn from(longitude: f32) -> Self {
        Self {
            ddm: match longitude < 0.0 {
                true => longitude * -1.0,
                false => longitude,
            }
            .into(),
            direction: match longitude < 0.0 {
                true => CardinalDirection::West,
                false => CardinalDirection::East,
            },
        }
    }
}

impl DdmAngle for DdmLongitude {
    fn get_direction(&self) -> CardinalDirection {
        self.direction
    }

    fn get_degree_minutes(&self) -> DegreeMinutes {
        self.ddm
    }
}

impl PackArrayString for DdmLongitude {
    fn pack_into<const SIZE: usize>(
        &self,
        s: &mut arrayvec::ArrayString<SIZE>,
    ) -> Result<(), crate::errors::PackError> {
        // Push DDM
        s.try_push(('0' as u8 + (self.ddm.degrees / 100) as u8) as char)?;
        self.ddm.pack_into(s)?;

        // Push Direction
        self.direction.pack_into(s)?;

        Ok(())
    }
}
