#[cfg(test)]
mod test{

    #[test]
    fn test_read_from_file(){
        let path = "test_matrix.txt";
        let matrix_string = "1.0 2.0 3.0 4.0 5.0\n6.0 7.0 8.0 9.0 10.0\n 11.0 12.0 13.0 14.0 15.0\n 16.0 17.0 18.0 19.0 20.0\n 21.0 22.0 23.0 24.0 25.0";
        let error_msg = "Failed to read from file+";
        let rm_error_msg = "Failed to remove file+";
        std::fs::write(path, matrix_string).expect(error_msg);

        let expected = linear_algebra::Matrix {
            rows: 5, 
            cols: 5,
            data: vec![
                vec![1.0, 2.0, 3.0, 4.0, 5.0],
                vec![6.0, 7.0, 8.0, 9.0, 10.0],
                vec![11.0, 12.0, 13.0, 14.0, 15.0],
                vec![16.0, 17.0, 18.0, 19.0, 20.0],
                vec![21.0, 22.0, 23.0, 24.0, 25.0],
            ],
        };

        let matrix = linear_algebra::Matrix::read_from_file(path);
        assert_eq!(matrix.rows, expected.rows);
        assert_eq!(matrix.cols, expected.cols);
        assert_eq!(matrix.data, expected.data);

        std::fs::remove_file(path).expect(rm_error_msg);
    }

    #[test]
    fn test_read_from_string(){
        let string = "1.0 2.0 3.0 4.0 5.0\n6.0 7.0 8.0 9.0 10.0\n 11.0 12.0 13.0 14.0 15.0\n 16.0 17.0 18.0 19.0 20.0\n 21.0 22.0 23.0 24.0 25.0";

        let expected = linear_algebra::Matrix {
            rows: 5, 
            cols: 5,
            data: vec![
                vec![1.0, 2.0, 3.0, 4.0, 5.0],
                vec![6.0, 7.0, 8.0, 9.0, 10.0],
                vec![11.0, 12.0, 13.0, 14.0, 15.0],
                vec![16.0, 17.0, 18.0, 19.0, 20.0],
                vec![21.0, 22.0, 23.0, 24.0, 25.0],
            ],
        };

        let matrix = linear_algebra::Matrix::read_from_string(string);

        assert_eq!(matrix.cols, expected.cols);
        assert_eq!(matrix.rows, expected.rows);
        assert_eq!(matrix.data, expected.data);
    }
 }