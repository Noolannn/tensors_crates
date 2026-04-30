use tensors_macros::*;
use tensors::*;

fn main() {
    println!("Hello, world!");
    tensor!(2);
    let epsilon = mat!(0, 1; -1, 0);
    dbg!(epsilon[0][1]);
    einstein!(epsilon);
    let salut: Tensor<2, 2, i32>;
    test_macro!(<const D0: usize>);
}
