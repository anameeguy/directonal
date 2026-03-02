use std::ops::{Index, IndexMut};

#[cfg(feature = "bevy")]
pub const DIRECTIONAL_VEC2S: Directional<bevy::math::Vec2> = Directional {
    right: bevy::math::Vec2::X,
    left: bevy::math::Vec2::NEG_X,
    up: bevy::math::Vec2::Y,
    down: bevy::math::Vec2::NEG_Y,
};
pub const DIRECTIONAL_DIRS: Directional<Direction> = Directional {
    right: Direction::Right,
    left: Direction::Left,
    up: Direction::Up,
    down: Direction::Down,
};
pub const ALL_DIRS: [Direction; 4] = [
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
];

#[cfg_attr(feature = "bevy", derive(bevy::reflect::Reflect))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Directional<T> {
    pub right: T,
    pub left: T,
    pub up: T,
    pub down: T,
}

impl<T> Directional<T> {
    pub fn new<B>(input: B) -> Directional<T>
    where
        Directional<T>: From<B>,
    {
        Directional::from(input)
    }
}

impl<T: Clone> From<T> for Directional<T> {
    fn from(value: T) -> Self {
        Self {
            right: value.clone(),
            left: value.clone(),
            up: value.clone(),
            down: value,
        }
    }
}

impl<T> From<[T; 4]> for Directional<T> {
    fn from([right, left, up, down]: [T; 4]) -> Self {
        Self {
            right,
            left,
            up,
            down,
        }
    }
}

impl<T> From<(T, T, T, T)> for Directional<T> {
    fn from((right, left, up, down): (T, T, T, T)) -> Self {
        Self {
            right,
            left,
            up,
            down,
        }
    }
}

// trait DirectionalGenerator {
//     type Item;

//     fn do_da_ting(self) -> Directional<Self::Item>;
// }

// impl<T: Clone> DirectionalGenerator for T {
//     type Item = T;

//     fn do_da_ting(self) -> Directional<T> {
//         Directional {
//             right: self.clone(),
//             left: self.clone(),
//             up: self.clone(),
//             down: self,
//         }
//     }
// }

// impl<T> DirectionalGenerator for [T; 4] {
//     type Item = T;

//     fn do_da_ting(self) -> Directional<T> {
//         let [right, left, up, down] = self;

//         Directional {
//             right,
//             left,
//             up,
//             down,
//         }
//     }
// }

impl<T> Index<Direction> for Directional<T> {
    type Output = T;

    fn index(&self, dir: Direction) -> &Self::Output {
        match dir {
            Direction::Right => &self.right,
            Direction::Left => &self.left,
            Direction::Up => &self.up,
            Direction::Down => &self.down,
        }
    }
}

impl<T> IndexMut<Direction> for Directional<T> {
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        match dir {
            Direction::Right => &mut self.right,
            Direction::Left => &mut self.left,
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
        }
    }
}

impl<T> IntoIterator for Directional<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [self.right, self.left, self.up, self.down].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Directional<T> {
    type Item = &'a T;
    type IntoIter = std::array::IntoIter<&'a T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.right, &self.left, &self.up, &self.down].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Directional<T> {
    type Item = &'a mut T;
    type IntoIter = std::array::IntoIter<&'a mut T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            &mut self.right,
            &mut self.left,
            &mut self.up,
            &mut self.down,
        ]
        .into_iter()
    }
}

#[cfg_attr(feature = "bevy", derive(bevy::reflect::Reflect))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Right || *self == Direction::Left
    }

    pub fn opposite(&self) -> Direction {
        let mut response = Direction::Up;
        let oppo_dirs = Directional {
            right: Direction::Left,
            left: Direction::Right,
            up: Direction::Down,
            down: Direction::Up,
        };

        for dir in DIRECTIONAL_DIRS {
            if *self == dir {
                response = oppo_dirs[dir];
                break;
            }
        }

        response
    }

    pub fn rotated_90_clockwise(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }

    pub fn rotated_90_anticlockwise(&self) -> Self {
        self.rotated_90_clockwise().opposite()
    }

    #[cfg(feature = "bevy")]
    pub fn get_vec2(&self) -> &bevy::math::Vec2 {
        &DIRECTIONAL_VEC2S[*self]
    }
}
