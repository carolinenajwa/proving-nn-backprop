
// Notes:
// Functions to Test:
// 1. dot_product (in progress)
// 2. matrix_multiply (in progress)
// 3. compute_transpose (in progress)
// 4. compute_factorial (in progress)
// 5. compute_ipow
// 6. compute_exp
// 7. isinstance

mod arithm_functions;

#[cfg(test)]
mod tests {
    use super::*;
    use arithm_functions::ArithmeticFunctions;
    
    #[test]
    fn test_dot_product() {
        let af = ArithmeticFunctions;

        // Test with valid inputs
        let a1: &[f64] = &[1.0, 2.0, 3.0];
        let b1: &[f64] = &[4.0, 5.0, 6.0];
        assert_eq!(af.dot_product(a1, b1), Ok(32.0));

        // Test with inputs of different lengths
        let a2: &[f64] = &[1.0, 2.0];
        let b2: &[f64] = &[4.0, 5.0, 6.0];
        assert_eq!(af.dot_product(a2, b2), Err("Both slices must be of the same length"));

        // Test with empty inputs
        let a3: &[f64] = &[];
        let b3: &[f64] = &[];
        assert_eq!(af.dot_product(a3, b3), Ok(0.0));
    }

    #[test]
    fn test_matrix_multiply() {
        let af = ArithmeticFunctions;

        // Test with valid matrices
        let mat1: &[Vec<f64>] = &[vec![1.0, 2.0], vec![3.0, 4.0]];
        let mat2: &[Vec<f64>] = &[vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = af.matrix_multiply(mat1, mat2).unwrap();
        assert_eq!(result, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);

        // Test with empty matrices
        let mat3: &[Vec<f64>] = &[];
        let mat4: &[Vec<f64>] = &[vec![]];
        assert_eq!(af.matrix_multiply(mat3, mat4), Err("Input matrices cannot be empty"));

        // Test with incompatible matrices
        let mat5: &[Vec<f64>] = &[vec![1.0, 2.0], vec![3.0, 4.0]];
        let mat6: &[Vec<f64>] = &[vec![5.0, 6.0, 7.0], vec![8.0, 9.0, 10.0]];
        assert_eq!(
            af.matrix_multiply(mat5, mat6),
            Err("Number of columns in mat1 must be equal to the number of rows in mat2")
        );
    }

    #[test]
    fn test_compute_transpose() {
        let af = ArithmeticFunctions;

        // Test with a valid matrix
        let matrix: &[Vec<f64>] = &[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = af.compute_transpose(matrix);
        assert_eq!(result, vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);

        // Test with an empty matrix
        let empty_matrix: &[Vec<f64>] = &[];
        let result2 = af.compute_transpose(empty_matrix);
        assert_eq!(result2, Vec::new());

        // Test with a matrix with empty rows
        let matrix_with_empty_rows: &[Vec<f64>] = &[vec![1.0, 2.0, 3.0], vec![]];
        let result3 = af.compute_transpose(matrix_with_empty_rows);
        assert_eq!(result3, Vec::new());
    }

    #[test]
    fn test_compute_factorial() {
        let af = ArithmeticFunctions;

        // Test with a valid input
        assert_eq!(af.compute_factorial(5), 120);

        // Test with n = 0
        assert_eq!(af.compute_factorial(0), 1);

        // Test with n = 1
        assert_eq!(af.compute_factorial(1), 1);

        // Test with n > 20
        assert_eq!(af.compute_factorial(25), 0);
    }

    #[test]
    fn test_compute_ipow() {
        let af = ArithmeticFunctions;

        // Test with valid inputs
        assert_eq!(af.compute_ipow(2, 3), Some(8));

        // Test with potential overflow
        assert_eq!(af.compute_ipow(10, 10), None);
    }

    #[test]
    fn test_compute_exp() {
        let af = ArithmeticFunctions;

        // Test with valid inputs
        assert_eq!(af.compute_exp(2.0, 0.0001), (2.0_f64).exp());

        // Test with large values
        assert_eq!(af.compute_exp(100.0, 0.0001), (100.0_f64).exp());
    }




}
