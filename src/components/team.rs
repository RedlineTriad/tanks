use super::TankColor;

#[derive(Clone, Copy)]
pub struct Team {
    pub team: i64,
}

impl Team {
    pub fn color(&self) -> TankColor {
        match self.team.rem_euclid(4) {
            0 => TankColor::Beige,
            1 => TankColor::Blue,
            2 => TankColor::Green,
            3 => TankColor::Red,
            _ => unreachable!(),
        }
    }
}