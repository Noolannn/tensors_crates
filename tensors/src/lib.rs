use std::ops::{Add, Index, IndexMut, Mul};

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
    pub content: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Default + Copy> Default for Mat<M, N, T> {
    fn default() -> Self {
        Self {
            content: [[T::default(); N]; M]
        }
    }
} 

impl<const L: usize, const M: usize, const N: usize, T: Add<Output = T> + Mul<Output = T> + Default + Copy> Mul<Mat<M, N, T>> for Mat<L, M, T> {
    type Output = Mat<L, N, T>;
    fn mul(self, rhs: Mat<M, N, T>) -> Self::Output {
        let mut res = Self::Output::default();
        for i in 0..L {
            for j in 0..N {
                let mut sum = T::default();
                for k in 0..L {
                    sum = sum + self[i][k] * rhs[k][j];
                }
                res[i][j] = sum;
            }
        }
        return res;
    }
}

impl<const M: usize, T: Add<Output = T> + Default + Copy> Mat<M, M, T> {
    pub fn trace(&self) -> T {
        let mut res = T::default();
        for i in 0..M {
            res = res + self.content[i][i];
        }
        return res;
    }
}

impl<const M: usize, const N: usize, T: ToString> Mat<M, N, T> {
    pub fn pretty_print(&self) {
        let mut max = 0;
        for i in 0..M {
            for j in 0..N {
                let len = self[i][j].to_string().chars().count();
                if len > max {
                    max = len;
                }
            }
        }
        
        for i in 0..M {
            let mut line_buffer = String::new();
            for j in 0..N {
                let string = self[i][j].to_string();
                line_buffer.push_str(&string);
                for k in 0..(max - string.chars().count()) {
                    line_buffer.push(' ');
                }
                line_buffer.push(' ');
            }
            println!("{}", line_buffer);
        }
    }
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

impl<const M: usize, const N: usize, T> IndexMut<usize> for Mat<M, N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
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
