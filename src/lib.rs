use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Matrix{
    pub rows: usize, 
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let data = vec![vec![0.0; cols]; rows];
        Matrix { rows, cols, data }
    }

    pub fn read_from_file(file_path: &str) -> Matrix {

        let file = File::open(file_path).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut matrix = Matrix::new(0, 0);
        for (i, line) in reader.lines().enumerate() {
            let row: Vec<f64> = line
                .unwrap()
                .trim()
                .split(' ')
                .map(|x| x.parse::<f64>().unwrap())
                .collect();
            if i == 0 {
                matrix.rows = row.len();
            }
            matrix.cols += 1;
            matrix.data.push(row);
        }
        matrix
    }

    pub fn from_string(string: &str) -> Matrix {
        let mut matrix = Matrix::new(0, 0);
        for i in string.lines() {
            let row: Vec<f64> = i.trim().split(' ').map(|x| x.parse::<f64>().unwrap()).collect();
            if matrix.cols == 0 {
                matrix.rows = row.len();
            }
            matrix.cols += 1;
            matrix.data.push(row);
        }
        matrix
    }
}



