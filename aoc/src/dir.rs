#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

#[allow(dead_code)]
impl Dir {
    pub fn from_letter_str(input: &str) -> Self {
        match input {
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            "U" => Self::Up,
            _ => unreachable!(),
        }
    }

    pub fn from_int_str(input: &str) -> Self {
        match input {
            "0" => Self::Right,
            "1" => Self::Down,
            "2" => Self::Left,
            "3" => Self::Up,
            _ => unreachable!(),
        }
    }

    pub fn is_vertical(&self) -> bool {
        *self == Self::Up || *self == Self::Down
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Self::Right || *self == Self::Left
    }

    pub fn reverse(&self) -> Self {
        match *self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
        }
    }

    pub fn reflect_forward_slash(&self) -> Self {
        match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Up,
            Self::Down => Self::Left,
            Self::Left => Self::Down,
        }
    }

    pub fn reflect_back_slash(&self) -> Self {
        match *self {
            Self::Up => Self::Left,
            Self::Right => Self::Down,
            Self::Down => Self::Right,
            Self::Left => Self::Up,
        }
    }
}
