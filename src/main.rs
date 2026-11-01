use matrix_tests::{Dimensions, Matrix};

fn main() {
    let matrix = Matrix::new(
        Dimensions::new(9, 9),
        vec![
            2, -1,  4,  0,  3,  5, -2,  1,  6,
            7,  2, -3,  4,  1, -5,  0,  3,  2,
            -4,  6,  1,  2,  5,  0,  7, -1,  3,
            3,  0, -2,  8,  4,  1, -3,  5,  2,
            1,  5,  6, -1,  0,  2,  4,  3, -2,
            0, -3,  2,  5,  7,  1,  6, -4,  8,
            4,  1,  0, -2,  3,  6,  5,  2, -1,
            -2,  7,  3,  1, -4,  5,  2,  6,  0,
            5,  2, -1,  3,  6, -2,  1,  4,  7,
        ],
    );

    let other_matrix = Matrix::new(
        Dimensions::new(9, 9),
        vec![
            1,  3, -2,  4,  0,  5,  2, -1,  6,
            2, -4,  6,  1,  3,  0,  5,  7, -2,
            5,  1,  0, -3,  4,  2, -1,  6,  3,
            -1,  2,  7,  5, -2,  3,  0,  4,  1,
            6,  0,  3,  2,  1, -4,  7, -2,  5,
            3,  5, -1,  0,  6,  2,  4,  1, -3,
            0, -2,  4,  6,  5,  1,  3,  2,  7,
            4,  6,  2, -1,  0,  7, -3,  5,  1,
            -3,  4,  1,  3,  2,  6,  0, -2,  8,
        ],
    );

    println!("{}", other_matrix.column(6).unwrap());


    let added = matrix.add(&other_matrix.negative());
    // let multiplied = matrix.multiply(&other_matrix);

    println!("added = \n{added}");
    // println!("\nmutliplied = \n{multiplied}");
}