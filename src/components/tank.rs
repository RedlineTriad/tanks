pub struct Tank {
    pub speed: f32,
    pub turn_speed: f32,
    pub health: i32,
}

pub enum TankColor {
    Beige,
    Blue,
    Green,
    Red,
}

impl TankColor {
    pub fn name(self) -> &'static str {
        match self {
            TankColor::Beige => "Beige",
            TankColor::Blue => "Blue",
            TankColor::Green => "Green",
            TankColor::Red => "Red",
        }
    }
}