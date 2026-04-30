use std::ops::Index;

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
