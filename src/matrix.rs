use std::ops;

/// A structure for using matrix
#[derive(Clone)]
pub struct Matrix{
    rows : usize,
    columns : usize,
    content : Vec<f64>
}

impl Matrix {
    fn get_index(&self, row : usize, column : usize) -> usize {
        row * self.columns + column
    }

    fn set_at_index(&mut self, index : usize, value : f64) -> () {
	    self.content[index] = value;
    }

    pub fn new(rows : usize, columns : usize) -> Matrix {
        Matrix {
            rows : rows,
            columns : columns,
            content : vec![0.0; rows * columns]
        }
    }

    pub fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn get_at(&self, row : usize, column : usize) -> f64 {
        self[self.get_index(row, column)]
    }

    pub fn set_at(&mut self, row : usize, column : usize, value : f64) -> () {
	    self.set_at_index(self.get_index(row, column), value);
    }

    pub fn determinant(&self) -> f64 {
        if self.rows != self.columns {
            panic!("Determinant exist only for (N x N) matrix");
        }

        if self.rows == 2 {
            // ad - bc
            (self.content[0] * self.content[3]) - (self.content[1] * self.content[2])
        } else{
            let mut det = 0.0;
            let sign : i8 = -1;
            let mut submatrix = Matrix::new(self.rows -1, self.columns -1);

            for x in 0 .. self.rows {
                let mut subi = 0;
                for i in 1 .. self.rows {
                    let mut subj = 0;
                    for j in 0 .. self.rows {
                        if j != x {
                            submatrix.set_at(subi, subj, self.get_at(i, j));
                            subj += 1;
                        }
                    }
                    subi += 1;
                }
                det += det + sign.pow(x as u32) as f64 * self.get_at(0, x) * submatrix.determinant();
            }
            det
        }
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        let mut output = String::from(format!("[{} x {}] : {}", self.rows, self.columns, self.size()));
        
        for (i, value) in self.content.iter().enumerate() {
            if i % &self.columns == 0 {
                output.push('\n');
            }
            let fmt_value = format!("{: <6}", value);
            output.push_str(fmt_value.as_str());
            
        }
        output
    }
}

impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Matrix) -> Matrix {
        if self.rows != _rhs.rows && self.columns != _rhs.columns {
            panic!("Matrixes must have the same rows and columns.");
        } else {
            let mut result = self.clone();

            for i in 0 .. self.size() {
			    result[i] = _rhs[i] + self.content[i];
		    }

            result
        }
    }
}

impl ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, _rhs: &Matrix) -> Matrix {
        if self.rows != _rhs.rows && self.columns != _rhs.columns {
            panic!("Matrixes must have the same rows and columns.");
        } else {
            let mut result = self.clone();

            for i in 0 .. self.size() {
			    result[i] = _rhs[i] + self.content[i];
		    }

            result
        }
    }
}

impl ops::Add<Matrix> for &mut Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Matrix) -> Matrix {
        if self.rows != _rhs.rows && self.columns != _rhs.columns {
            panic!("Matrixes must have the same rows and columns.");
        } else {
            let mut result = self.clone();

            for i in 0 .. self.size() {
			    result[i] = _rhs[i] + self.content[i];
		    }

            result
        }
    }
}

impl ops::AddAssign<Matrix> for Matrix {
    fn add_assign(&mut self, _rhs: Matrix) {
        if self.rows != _rhs.rows && self.columns != _rhs.columns {
            panic!("Matrixes must have the same rows and columns.");
        } else {
            for i in 0 .. self.size() {
			    self[i] = _rhs[i] + self[i];
		    }
        }
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        if self.columns != _rhs.rows {
            panic!("Second matrix must have the same columns as the first matrix rows.");
        } else {
            let mut result = self.clone();

            for i in 0 .. self.rows {
                for j in 0 .. _rhs.columns {
                    let mut value = 0.0;
                    for k in 0 .. self.columns {
                        value += self.get_at(i, k) * _rhs.get_at(k, j);
                    }

                    result.set_at(i, j, value);
                }
		    }

            result
        }

    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: &Matrix) -> Matrix {
        if self.columns != _rhs.rows {
            panic!("Second matrix must have the same columns as the first matrix rows.");
        } else {
            let mut result = self.clone();

            for i in 0 .. self.rows {
                for j in 0 .. _rhs.columns {
                    let mut value = 0.0;
                    for k in 0 .. self.columns {
                        value += self.get_at(i, k) * _rhs.get_at(k, j);
                    }

                    result.set_at(i, j, value);
                }
		    }

            result
        }

    }
}

impl ops::MulAssign<Matrix> for Matrix {
    fn mul_assign(&mut self, _rhs: Matrix) {
        if self.columns != _rhs.rows {
            panic!("Second matrix must have the same columns as the first matrix rows.");
        } else {
            let mut result = self.clone();

            for i in 0 .. self.rows {
                for j in 0 .. _rhs.columns {
                    let mut value = 0.0;
                    for k in 0 .. self.columns {
                        value += self.get_at(i, k) * _rhs.get_at(k, j);
                    }

                    result.set_at(i, j, value);
                }
		    }

            for i in 0 .. result.size() {
                self[i] = result[i];
            }
        }
    }
}

impl ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: f64) -> Matrix {
        let mut result = self.clone();

        for i in 0 .. self.size() {
            result[i] = self[i] * _rhs;
        }

        result
    }
}

impl ops::MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, _rhs: f64) {
        for i in 0 .. self.size() {
            self.content[i] *= _rhs;
        }
    }
}

impl ops::Index<usize> for Matrix {
    type Output = f64;

    fn index(&self, position : usize) -> &Self::Output {
        &self.content[position]
    }
}

impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, position : usize) -> &mut f64 {
        &mut self.content[position]
    }
}