use core::fmt::{Debug, Display, Formatter, Result};
use core::marker::PhantomData;

/// A wrapper type for use with Matrix2d
///
/// Any type that implements PartialEq with MatrixBool will be able
/// to be displayed by Matrix2d.
pub struct MatrixBool(bool);
impl PartialEq<MatrixBool> for bool {
    fn eq(&self, other: &MatrixBool) -> bool {
        *self == other.0
    }
}
impl PartialEq<MatrixBool> for u8 {
    fn eq(&self, other: &MatrixBool) -> bool {
        *self == other.0 as u8
    }
}

/// A generic unit struct that holds a reference to any 2-dimensional slice-like type
pub struct Matrix2d<'a, T, Matrix, Row>(&'a Matrix, PhantomData<T>, PhantomData<Row>)
where
    Matrix: AsRef<[Row]>,
    Row: AsRef<[T]>;

impl<'a, T, Matrix, Row> Matrix2d<'a, T, Matrix, Row>
where
    Matrix: AsRef<[Row]>,
    Row: AsRef<[T]> + 'a,
{
    pub fn from(matrix: &'a Matrix) -> Self {
        Self(matrix, PhantomData::<T>, PhantomData::<Row>)
    }

    pub fn rows(&self) -> impl Iterator<Item = &'_ Row> {
        self.0.as_ref().iter()
    }

    pub fn row_len(&self) -> usize {
        self.0.as_ref().len()
    }

    pub fn col_len(&self) -> usize {
        if let Some(row) = self.0.as_ref().first() {
            row.as_ref().len()
        } else {
            0
        }
    }
}

impl<'a, T, Matrix, Row> Debug for Matrix2d<'a, T, Matrix, Row>
where
    T: Debug,
    Matrix: AsRef<[Row]>,
    Row: AsRef<[T]> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.rows()
            .fold(&mut f.debug_list(), |b, e| b.entry(&e.as_ref()))
            .finish()
    }
}

impl<'a, T, Matrix, Row> Display for Matrix2d<'a, T, Matrix, Row>
where
    T: PartialEq<MatrixBool>,
    Matrix: AsRef<[Row]>,
    Row: AsRef<[T]>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = String::with_capacity(self.row_len() * self.col_len() + self.row_len());
        for row in self.rows() {
            row.as_ref()
                .iter()
                .map(|x| {
                    if *x == MatrixBool(true) {
                        '\u{2B1B}'
                    } else {
                        '\u{2B1C}'
                    }
                })
                .collect_into(&mut output);
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}
