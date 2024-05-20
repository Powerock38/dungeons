use bevy::math::IVec2;

#[macro_export]
macro_rules! extract_ok {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}

#[macro_export]
macro_rules! extract_some {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

pub struct Map2D<const N: usize, T: Copy> {
    data: [[T; N]; N],
}

impl<const N: usize, T: Copy> Map2D<N, T> {
    pub fn new(fill_with: T) -> Self {
        Self {
            data: [[fill_with; N]; N],
        }
    }

    pub fn get(&self, index: IVec2) -> Option<T> {
        if index.x < 0 || index.y < 0 {
            return None;
        }

        let x = index.x as usize;
        let y = index.y as usize;

        if x >= N || y >= N {
            return None;
        }

        Some(self.data[x][y])
    }

    pub fn set(&mut self, index: IVec2, value: T) {
        if index.x < 0 || index.y < 0 {
            return;
        }

        let x = index.x as usize;
        let y = index.y as usize;

        if x < N && y < N {
            self.data[x][y] = value;
        }
    }
}
