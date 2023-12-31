use nalgebra::{SMatrix, Matrix4};

pub struct State {
    matrix: SMatrix<u32, 4, 4>,
    max: u32,
}

impl State {
    fn new(m: Matrix4<u32>) -> State {
        State {
            matrix: m,
            max: m.max(),
        }
    }

    fn shift_right(mut self: State) -> State {
        self.matrix = self.matrix
            .remove_column(3)
            .insert_column(0, 0);
        self
    }

    fn shift_left(mut self: State) -> State {
        self.matrix = self.matrix
            .remove_column(0)
            .insert_column(3, 0);
        self
    }

    fn shift_up(mut self: State) -> State {
        self.matrix = self.matrix
            .remove_row(0)
            .insert_row(3, 0);
        self
    }

    fn shift_down(mut self: State) -> State {
        self.matrix = self.matrix
            .remove_row(3)
            .insert_row(0, 0);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_right_fills_left_with_zeroes() -> () {
        let s = State::new(Matrix4::repeat(1));
        let res = s.shift_right();
        for i in 0..=3 {
            for j in 0..=3 {
                if j == 0 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_left_fills_right_with_zeroes() -> () {
        let s = State::new(Matrix4::repeat(1));
        let res = s.shift_left();
        for i in 0..=3 {
            for j in 0..=3 {
                if j == 3 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_up_fills_bottom_with_zeroes() -> () {
        let s = State::new(Matrix4::repeat(1));
        let res = s.shift_up();
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 3 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_down_fills_up_with_zeroes() -> () {
        let s = State::new(Matrix4::repeat(1));
        let res = s.shift_down();
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }
}