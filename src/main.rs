mod matrix;

use std::io;
use std::io::Write;

macro_rules! print_flush {
    ($($arg:tt)*) => {
        print!($($arg)*);
        io::stdout().flush().unwrap();
    };
}

fn get_input() -> f64 {
    let mut op = String::new();
    match io::stdin().read_line(&mut op) {
        Ok(_) => {
            op.trim().parse::<f64>().unwrap()
        }
        Err(_) => {
            -1.0
        }
    }
}

fn init_matrix(mat : &mut matrix::Matrix) -> () {
    for i in 0 .. mat.size() {
        mat[i] = get_input();
    }
}

fn mult() -> () {
	println!("Matrix Multiplication");
	println!("____________________________________________________\n");

    let number : usize;
    let rows : usize;
    let mut columns : usize;


	print_flush!("Enter the number to multiply: ");
    number = get_input() as usize;

	println!("Enter first matrix size (N x K): ");
	rows = get_input() as usize;
    columns = get_input() as usize;


	let mut result = matrix::Matrix::new(rows, columns);
    println!("Enter {} values:", result.size());
	init_matrix(&mut result);

    for i in 1 .. number {
        print_flush!("Enter matrix #{} columns: ", i + 1);
        columns = get_input() as usize;
        let mut temp = matrix::Matrix::new(result.columns(), columns);
        println!("Enter {} values for matrix #{}: ", temp.size(), i + 1);
        init_matrix(&mut temp);
        result = result * temp;
    }

    println!("The result is: {}", result.to_string());
}

fn determinant() -> () {
    println!("Matrix Determinant Calculator");
    println!("____________________________________________________\n");

	let rows : usize;

	print_flush!("Enter matrix size (N x N): ");
    rows = get_input() as usize;

	let mut result = matrix::Matrix::new(rows, rows);
	println!("Enter {} values: ", result.size());

	init_matrix(&mut result);
	
	println!("The determinant of the matrix\n{}\nis: {}", result.to_string(), result.determinant());
}

fn sum() ->() {
    println!("Matrix Summerize");
    println!("____________________________________________________\n");

	let number : usize;
	let rows : usize;
    let cols : usize;

	print_flush!("Enter the number to sum: ");
	number = get_input() as usize;

	println!("Enter first matrix size (N x K):");
	rows = get_input() as usize;
    cols = get_input() as usize;

	let mut result = matrix::Matrix::new(rows, cols);
	println!("Enter {} values:", result.size());
	init_matrix(&mut result);

    for i in 1 .. number {
        let mut temp = matrix::Matrix::new(rows, cols);
        println!("Enter {} values for matrix #{}:", temp.size(), i + 1);
        init_matrix(&mut temp);

        result += temp;
    }

	println!("The result is:\n{}", result.to_string());
}

fn scalar() -> () {
    println!("Matrix Scalar Multiplication");
	println!("____________________________________________________\n");

	let rows : usize;
    let cols : usize;
	let value : f64;

	println!("Enter first matrix size (N x K):");
	rows = get_input() as usize;
    cols = get_input() as usize;

	let mut result = matrix::Matrix::new(rows, cols);
	println!("Enter {} values:", result.size());
	init_matrix(&mut result);

	print_flush!("Enter a scalar value to multiply with: ");
	value = get_input();

	println!("The reslut of the matrix:\n{}\n times {} is:\n{}" ,result.to_string(), value , (result * value).to_string());
}

fn main() {
    let mut op = -1;

    println!("Welcome to rust Matrix V1 - by David Delarosa");
	println!("____________________________________________________\n");
	
    while op != 0 {

        println!("Select from the following:");
        println!("1 - Multiply matrix");
        println!("2 - Calculete matrix determinant");
        println!("3 - Sum matrix together");
        println!("4 - Multiply matrix in scalar");
        println!("0 - Exit");

        print_flush!("Selection: ");
        op = get_input() as i16;
        match op {
            1 => mult(),
            2 => determinant(),
            3 => sum(),
            4 => scalar(),
            0 => println!("See you later."),
            _ => {}
        }
    }

}
