pub struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn try_decrement(count: &mut i32) -> bool {
        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => Self::try_decrement(&mut self.big),
            2 => Self::try_decrement(&mut self.medium),
            3 => Self::try_decrement(&mut self.small),
            _ => panic!("wtf"),
        }
    }
}
