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
