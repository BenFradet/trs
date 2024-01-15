use nalgebra::{SMatrix, Matrix4};
use rand::Rng;

use crate::state::buckets::Buckets;

pub struct Grid {
    pub matrix: SMatrix<u32, 4, 4>,
}

impl Grid {
    pub fn new<R: Rng + ?Sized>(r: &mut R, base_values: Box<[u32]>) -> Grid {
        let grid_size = 16;
        let buckets = Buckets::new(r, base_values, grid_size);
        let elements = buckets.draw(r);
        Grid {
            matrix: Matrix4::from_iterator(elements),
        }
    }
}
