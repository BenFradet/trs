use nalgebra::{SMatrix, Scalar};

pub trait MatrixAny {
    type Item;

    fn any_row<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(&[Self::Item]) -> bool;

    fn any_col<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(&[Self::Item]) -> bool;
}

impl<T: Clone + Scalar, const C: usize, const R: usize> MatrixAny for SMatrix<T, C, R> {
    type Item = T;

    fn any_col<F>(&self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(&[Self::Item]) -> bool,
    {
        self.column_iter().any(|c| f(c.as_slice()))
    }

    fn any_row<F>(&self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(&[Self::Item]) -> bool,
    {
        // row views are not contiguous, hence the clone_owned
        self.row_iter().any(|r| f(r.clone_owned().as_slice()))
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Matrix4;

    use super::*;

    #[test]
    fn any_col_satisfies_noop_predicate() -> () {
        let m = Matrix4::repeat(1);
        assert!(!m.any_col(|_| false));
        assert!(m.any_col(|_| true));
    }

    #[test]
    fn any_col_satisfies_predicate() -> () {
        let m = Matrix4::repeat(1);
        assert!(!m.any_col(|c| c.contains(&0)));
        assert!(m.any_col(|c| c.contains(&1)));
    }

    #[test]
    fn any_row_satisfies_noop_predicate() -> () {
        let m = Matrix4::repeat(1);
        assert!(!m.any_row(|_| false));
        assert!(m.any_row(|_| true));
    }

    #[test]
    fn any_row_satisfies_predicate() -> () {
        let m = Matrix4::repeat(1);
        assert!(!m.any_row(|c| c.contains(&0)));
        assert!(m.any_row(|c| c.contains(&1)));
    }
}
