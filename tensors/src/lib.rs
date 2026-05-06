use std::ops::{Add, Index, Mul};

use tensors_macros::tensor;

tensor!(1);
tensor!(2);
tensor!(3);
tensor!(4);

impl<const M: usize, T: Default + Copy + Add + Mul> From<Tensor1<M, T>> for [T; M] {
    fn from(value: Tensor1<M, T>) -> Self {
        value.content
    }
}

impl<const M: usize, T: Default + Copy + Add + Mul> From<[T; M]> for Tensor1<M, T> {
    fn from(value: [T; M]) -> Self {
        Self {
            content: value
        }
    }
}

/// M rows, N columns
pub struct Mat<const M: usize, const N: usize, T> {
    content: [[T; N]; M],
}

impl<const M: usize, const N: usize, T> From<[[T; N]; M]> for Mat<M, N, T> {
    fn from(value: [[T; N]; M]) -> Self {
        Self {
            content: value
        }
    }
}

impl<const M: usize, const N: usize, T> Index<usize> for Mat<M, N, T> {
    type Output = [T; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl<const M: usize, const N: usize, T: Default + Copy + Add + Mul> From<Tensor2<M, N, T>> for Mat<M, N, T> {
    fn from(value: Tensor2<M, N, T>) -> Self {
        Self {
            content: value.content
        }
    }
}

impl<const D0: usize, const D1: usize, T: Default + Copy + Add + Mul> From<Mat<D0, D1, T>> for Tensor2<D0, D1, T> {
    fn from(value: Mat<D0, D1, T>) -> Self {
        Self {
            content: value.content
        }
    }
}

#[macro_export]
macro_rules! mat {
    ($($($x:expr),*);*) => {
        Mat::<_, _, _>::from([$([$($x),*]),*])
    };
}

#[macro_export]
macro_rules! tensor_bis {
    ($($x:expr),*; $t:ty) => {
        use std::marker::PhantomData;
        struct Tensor<const D: usize, T> {
            _phantom: PhantomData<T>,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
