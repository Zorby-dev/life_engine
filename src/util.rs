use std::ops::Add;

use sdl2::rect::Rect;

pub type CellsToRender = Vec<Vector>;

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    pub fn down_left(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    pub fn up_left(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    pub fn down_right(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    pub fn up_right(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    pub fn cross_neighbors(&self) -> Vec<Self> {
        vec![
            self.left(),
            self.right(),
            self.up(),
            self.down()
        ]
    }

    pub fn diagonal_neighbors(&self) -> Vec<Self> {
        vec![
            self.up_left(),
            self.up_right(),
            self.down_left(),
            self.down_right()
        ]
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            self.left(),
            self.right(),
            self.up(),
            self.down(),
            self.up_left(),
            self.up_right(),
            self.down_left(),
            self.down_right()
        ]
    }
}

impl Into<Vector> for &Vector {
    fn into(self) -> Vector {
        self.clone()
    }
}

impl<T> Into<Vector> for (T, T)
where
    T: TryInto<i32>,
{
    fn into(self) -> Vector {
        let x = self.0.try_into().ok().unwrap();
        let y = self.1.try_into().ok().unwrap();

        Vector::new(x, y)
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<&Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: &Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

pub fn pos_to_rect(pos: &Vector, size: &Vector) -> Rect {
    Rect::new(pos.x, pos.y, size.x as u32, size.y as u32)
}
