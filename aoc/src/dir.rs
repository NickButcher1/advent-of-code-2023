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
            "R" => Dir::Right,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "U" => Dir::Up,
            _ => unreachable!(),
        }
    }

    pub fn from_int_str(input: &str) -> Self {
        match input {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
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
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }

    pub fn reflect_forward_slash(&self) -> Self {
        match *self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Down,
        }
    }

    pub fn reflect_back_slash(&self) -> Self {
        match *self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Up,
        }
    }
}
