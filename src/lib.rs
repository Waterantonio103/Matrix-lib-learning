use std::{error::Error, fmt::{self, Debug, Display, Formatter}, ops::{Add, Mul, Neg}};

#[derive(Debug, Clone, Copy)]
pub enum MatrixErr {
    InvalidDimensions,
    DifferentDimensions(Operation, (Dimensions, Dimensions)),
}

impl Display for MatrixErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions => {
                write!(f, "invalid matrix dimensions")?;
            },
            Self::DifferentDimensions(operation, (self_dimensions, other_dimensions)) => {
                write!(f, "cannot '{operation}' matrices with dimensions '{self_dimensions}' and '{other_dimensions}'")?;
            },
        }
        Ok(())
    }
}

impl Error for MatrixErr {}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => {
                write!(f, "Add")?;
            },
            Operation::Subtract => {
                write!(f, "Subtract")?;
            },
            Operation::Multiply => {
                write!(f, "Multiply")?;
            },
            Operation::Divide => {
                write!(f, "Divide")?;
            },
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Matrix<T> {
    dimensions: Dimensions,
    data: Vec<T>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    rows: usize,
    columns: usize,
}

#[derive(Debug)]
pub struct Row<'a, T> {
    data: Vec<&'a T>,
}

struct RowIter<'a, T> {
    data: &'a [T],
    start_index: usize,
    end_index: usize,
}

pub struct RowsIter<'a, T> {
    matrix: &'a Matrix<T>,
    current_row: usize,
}

#[derive(Debug)]
pub struct Column<'a, T> {
    data: Vec<&'a T>,
}

struct ColumnIter<'a, T> {
    data: &'a[T],
    column_index: usize,
    step: usize,
}

pub struct ColumnsIter<'a, T> {
    matrix: &'a Matrix<T>,
    current_column: usize,
}

impl<T> Matrix<T> {
    pub fn new(dimensions: Dimensions, data: Vec<T>) -> Self {
        if !Self::check_dimensions(&dimensions.rows, &dimensions.columns, &data) {
            panic!("invalid dimensions, expected {} found {}", data.len(), dimensions.rows * dimensions.columns);
        }
        Matrix {
            dimensions,
            data,
        }
    }

    pub fn try_new(dimensions: Dimensions, data: Vec<T>) -> Result<Self, MatrixErr> {
        if !Self::check_dimensions(&dimensions.rows, &dimensions.columns, &data) {
            return Err(MatrixErr::InvalidDimensions);
        }
        Ok(
            Matrix {
                dimensions,
                data,
            }
        )
    }
    
    fn check_dimensions(rows: &usize, columns: &usize, data: &[T]) -> bool {
        if (rows * columns) != data.len() {
            return false;
        }
        true
    }

    fn equal_dimensions(m: &Dimensions, s: &Dimensions) -> bool {
        if m.rows != s.rows || m.columns != s.columns {
            return false;
        }
        true
    }

    fn row_equal_col(m: &Dimensions, s: &Dimensions) -> bool {
        if m.columns != s.rows {
            return false;
        }
        true
    }

    pub fn row<'a>(&'a self, index: usize) -> Option<Row<'a, T>> {
        if index >= self.dimensions.rows {
            return None;
        }
        let start = index * self.dimensions.columns;
        let end = start + self.dimensions.columns;
        let data = RowIter::new(
            self.data.as_slice(), 
            start, 
            end,
        ).collect();

        Some(Row { data })
    }

    pub fn rows<'a>(&'a self) -> RowsIter<'a, T> {
        RowsIter { 
            matrix: &self, 
            current_row: 0,
        }
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_mut(self.dimensions.columns)
    }

    pub fn column<'a>(&'a self, index: usize) -> Option<Column<'a, T>> {
        if index >= self.dimensions.columns {
            return None;
        }

        let data = ColumnIter::new(
            self.data.as_slice(), 
            index, 
            self.dimensions.columns,
        ).collect();

        Some(Column { data })
    }

    pub fn columns<'a>(&'a self) -> ColumnsIter<'a, T> {
        ColumnsIter::new(
            &self, 
            0
        )
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }

    pub fn negative(&self) -> Self where T: Copy + Neg , Vec<T>: FromIterator<<T as Neg>::Output> {
        Matrix::new(self.dimensions, self.iter().map(|&el| std::ops::Neg::neg(el)).collect())
    }
    
    pub fn add(&self, other: &Matrix<T>) -> Matrix<T> where T: Copy + Add , Vec<T>: FromIterator<<T as Add>::Output> {
        if !Self::equal_dimensions(&self.dimensions, &other.dimensions) {
            panic!("cannot add matrices of dimensions : {} and {}", self.dimensions, other.dimensions);
        }

        let data = self.rows().zip(other.rows())
            .flat_map(|(row1, row2)| {
                row1.add(&row2).collect::<Vec<_>>()
            }).collect::<Vec<T>>();

        Matrix::new(self.dimensions, data)
    }

    pub fn try_add(&self, other: &Matrix<T>) -> Result<Matrix<T>, MatrixErr> where T: Copy + Add , Vec<T>: FromIterator<<T as Add>::Output> {
        if !Self::equal_dimensions(&self.dimensions, &other.dimensions) {
            return Err(MatrixErr::DifferentDimensions(Operation::Add, (Dimensions::new(self.dimensions.rows, self.dimensions.columns), Dimensions::new(other.dimensions.rows, other.dimensions.columns))));
        }

        let data = self.rows().zip(other.rows())
            .flat_map(|(row1, row2)| {
                row1.add(&row2).collect::<Vec<_>>()
            }).collect::<Vec<T>>();

        Ok(Matrix::new(self.dimensions, data))
    }

    pub fn multiply(&self, other: &Matrix<T>) -> Matrix<T> where T: std::iter::Sum<<T as Mul>::Output> + Copy + Mul , Vec<T>: FromIterator<<T as Mul>::Output> {
        if !Self::row_equal_col(&self.dimensions, &other.dimensions) {
            panic!("cannot multiply matrices with dimensions {} and {}", self.dimensions, other.dimensions);
        }

        let data = self.rows().flat_map(|row| {
            other.columns().map(move |col| row.multiply_by_col(&col).sum::<T>())
        }).collect::<Vec<T>>();

        Matrix::new(Dimensions::new(self.dimensions.rows, other.dimensions.columns), data)
    }

    pub fn try_multiply(&self, other: &Matrix<T>) -> Result<Matrix<T>, MatrixErr> where T: std::iter::Sum<<T as Mul>::Output> + Copy + Mul , Vec<T>: FromIterator<<T as Mul>::Output> {
        if !Self::row_equal_col(&self.dimensions, &other.dimensions) {
            return Err(MatrixErr::DifferentDimensions(Operation::Multiply, (self.dimensions, other.dimensions)));
        }

        let data = self.rows().flat_map(|row| {
            other.columns().map(move |col| row.multiply_by_col(&col).sum::<T>())
        }).collect::<Vec<T>>();

        Ok(Matrix::new(Dimensions::new(self.dimensions.rows, other.dimensions.columns), data))
    }

    pub fn determinant(self) where T: Copy + Add {
        let a = self.row(0).unwrap().get(0).unwrap();

        let added = self.row(0).unwrap().add_num(*a);
    }
}

impl<T: Display + Ord> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.data.len() == 0 { return Ok(()); }
        let padding = self.data.iter()
            .max()
            .unwrap()
            .to_string()
            .chars().count() + 2;
        write!(f, "Matrix {{\n")?;
        for row in 0..self.dimensions.rows {
            let start = row * self.dimensions.columns;
            let end = start + self.dimensions.columns;
            let data = RowIter::new(
                self.data.as_slice(), 
                start, 
                end,
            ).collect::<Vec<&T>>();
            data.iter()
                .map(|&element| -> fmt::Result{
                    write!(f, "{:^padding$}", element)?;
                    Ok(())
                })
                .for_each(drop);
            write!(f, "\n")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl Dimensions {
    pub const fn new(rows: usize, columns: usize) -> Self {
        Self { rows, columns }
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.rows, self.columns)
    }
}

impl<'a, T: Display> Display for Row<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Row [")?;
        let mut iter = self.data.iter();
        if let Some(first_element) = iter.next() {
            write!(f, "{}", *first_element)?;
        }
        iter.map(|&element| -> fmt::Result {
            write!(f, ", {}", element)?;
            Ok(())
        }).for_each(drop);

        write!(f, "]")?;
        Ok(())
    }
}

impl<'a, T> Row<'a, T> {
    pub fn new(data: Vec<&'a T>) -> Self {
        Row { data }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a T> {
        self.data.iter().copied()
    }

    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index).copied()
    }

    pub fn add(&self, other: &Self) -> impl Iterator<Item = T::Output> where T: Copy + Add {
        self.iter().zip(other.iter()).map(|(&el1, &el2)| {
            el1 + el2
        })
    }

    pub fn add_num(&self, num: T) -> impl Iterator<Item = T::Output> + use<T> where T: Copy + Add + use<T> {
        self.iter().map(move |&el| {
            el + num
        })
    }

    pub fn multiply_by_col(&self, column: &Column<T>) -> impl Iterator<Item = T::Output> where T: Copy + Mul {
        self.iter().zip(column.iter())
            .map(|(&f, &s)| {
                f * s
            })
    }
}

impl<'a, T> RowIter<'a, T> {
    fn new(data: &'a [T], start_index: usize, end_index: usize) -> Self {
        RowIter { 
            data, 
            start_index,
            end_index,
        }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_index >= self.end_index {
            return None;
        }

        let item = &self.data[self.start_index];
        self.start_index += 1;

        Some(item)
    }
}

impl<'a, T> RowsIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, current_row: usize) -> Self {
        RowsIter { 
            matrix, 
            current_row,
        }
    }

    pub fn iter(&self) -> Self {
        RowsIter::new(&self.matrix, self.current_row)
    }
}

impl<'a, T> Iterator for RowsIter<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.matrix.dimensions.rows {
            return None;
        }
        let current_row = self.matrix.row(self.current_row).unwrap();
        self.current_row += 1;

        Some(current_row)
    }
}

impl<'a, T> Column<'a, T> {
    pub fn new(data: Vec<&'a T>) -> Self {
        Column { data }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a T> {
        self.data.iter().copied()
    }
}

impl<'a, T: Display> Display for Column<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Column [")?;
        let mut iter = self.data.iter();
        if let Some(first_element) = iter.next() {
            write!(f, "{}", *first_element)?;
        }
        iter.map(|&element| -> fmt::Result {
            write!(f, ", {}", element)?;
            Ok(())
        }).for_each(drop);
        
        write!(f, "]")?;
        Ok(())
    }
}

impl<'a, T> ColumnIter<'a, T> {
    fn new(data: &'a[T], column_index: usize, step: usize) -> Self {
        ColumnIter { 
            data, 
            column_index,
            step, 
        }
    }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index >= self.data.len() {
            return None;
        }

        let item = &self.data[self.column_index];
        self.column_index += self.step;

        Some(item)
    }
}

impl<'a, T> ColumnsIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, current_column: usize) -> Self {
        ColumnsIter { 
            matrix, 
            current_column,
        }
    }
}

impl<'a, T> Iterator for ColumnsIter<'a, T> {
    type Item = Column<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column >= self.matrix.dimensions.columns {
            return None;
        }
        let current_column = self.matrix.column(self.current_column).unwrap();
        self.current_column += 1;

        Some(current_column)
    }
}

