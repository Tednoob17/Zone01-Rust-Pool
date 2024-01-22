
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
	let  ((a, b), (c, d)) = m.0;
	Matrix((a, c), (b, d))
}
