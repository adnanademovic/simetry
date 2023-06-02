use crate::iracing::{BitField, VarData, VarType};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CarPositions {
    /// Check is off.
    #[default]
    Off = 0,
    /// No cars around us.
    Clear = 1,
    /// There is a car to our left.
    CarLeft = 2,
    /// There is a car to our right.
    CarRight = 3,
    /// There are cars on each side.
    CarLeftRight = 4,
    /// There are two cars to our left.
    CarsLeft = 5,
    /// There are two cars to our right.
    CarsRight = 6,
}

impl CarPositions {
    pub fn car_left(self) -> bool {
        match self {
            CarPositions::CarLeft | CarPositions::CarLeftRight | CarPositions::CarsLeft => true,
            CarPositions::Off
            | CarPositions::Clear
            | CarPositions::CarRight
            | CarPositions::CarsRight => false,
        }
    }

    pub fn car_right(self) -> bool {
        match self {
            CarPositions::CarRight | CarPositions::CarLeftRight | CarPositions::CarsRight => true,
            CarPositions::Off
            | CarPositions::Clear
            | CarPositions::CarLeft
            | CarPositions::CarsLeft => false,
        }
    }
}

impl VarData for CarPositions {
    fn parse(var_type: VarType, data: &[u8]) -> Option<Self> {
        let bit_field = BitField::parse(var_type, data)?;
        Some(match bit_field.0 {
            0 => CarPositions::Off,
            1 => CarPositions::Clear,
            2 => CarPositions::CarLeft,
            3 => CarPositions::CarRight,
            4 => CarPositions::CarLeftRight,
            5 => CarPositions::CarsLeft,
            6 => CarPositions::CarsRight,
            _ => return None,
        })
    }
}
