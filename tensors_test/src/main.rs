use std::{any::Any, default, ops::{Add, Index, IndexMut, Mul}};

use tensors_macros::*;
use tensors::*;

// tensor!(1);
// tensor!(2);

// impl<const D0: usize, const D1: usize, T: Default + Copy + Add + Mul + ToString> Tensor2<D0, D1, T> {
//     fn matrix_display(&self) {
//         let mut cell_size = 0;
//         for i in 0..D0 {
//             for j in 0..D1 {
//                 let current_len = self.content[i][j].to_string().len();
//                 if current_len > cell_size {
//                     cell_size = current_len;
//                 }
//             }
//         }
//         for i in 0..D0 {
//             let mut line = String::new();
//             for j in 0..D1 {
//                 let elem = self.content[i][j].to_string();
//                 line.push_str(&format!("{:>width$} ", elem, width = cell_size));
//             }
//             println!("{}", line);
//         }
//     }
// }

// tensor!(3);

// impl<const D0: usize, const D1: usize, const D2: usize, T: Default + Copy + Add + Mul> Index<usize> for Tensor3<D0, D1, D2, T> {
//     type Output = [[T; D2]; D1];
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.content[index]
//     }
// }

// impl<const D0: usize, const D1: usize, const D2: usize, T: Default + Copy + Add + Mul> IndexMut<usize> for Tensor3<D0, D1, D2, T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.content[index]
//     }
// }

fn main() {

    println!("Hello, world!");
    let epsilon = mat!(0, 1; -1, 0);
    dbg!(epsilon[0][1]);
    let mut salut: Tensor2<2, 2, i32> = Tensor2::<2, 2, i32>::default();

    let mut g: Tensor2<4, 4, i32> = Tensor2::default();
    let mut res: Tensor2<4, 4, i32> = Tensor2::default();

    let mut gammas: Tensor3<4, 4, 4, f32> = Tensor3::default();

    let gamma0 = mat!(
        0.0, 0.0, 1.0, 0.0;
        0.0, 0.0, 0.0, 1.0;
        1.0, 0.0, 0.0, 0.0;
        0.0, 1.0, 0.0, 0.0
    );

    gammas[0] = gamma0.content;

    let mut matrices: Tensor3<3, 2, 2, i32> = Tensor3::default();
    let m0 = mat!(
        1, 0;
        0, 1
    );
    let m1 = mat!(
        2, 0;
        0, 2
    );
    let m2 = mat!(
        3, 0;
        0, 3
    );
    matrices[0] = m0.content;
    matrices[1] = m1.content;
    matrices[2] = m2.content;
    
    let gamma0: Tensor2<4, 4, i32> = mat!(
        0, 0, 1, 0;
        0, 0, 0, 1;
        1, 0, 0, 0;
        0, 1, 0, 0
    ).into();

    g[[0, 0]] = 1;
    g[[1, 1]] = -1;
    g[[2, 2]] = -1;
    g[[3, 3]] = -1;

    // dbg!(gamma0.content);
    einstein!(res[[a, b]] = gamma0[[a, c]] * gamma0[[c, b]]);
    // dbg!(res.content);

    // res.matrix_display();
    // test_macro!(g[[1, b]] * (salut[[a, b]] * fhg[[1, a]] + gt[[b, 0]]));
}