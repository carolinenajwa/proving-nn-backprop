
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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_matrix_multiply() {
        let af = ArithmeticFunctions;
        let mat1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let mat2 = vec![vec![2.0, 0.0], vec![1.0, 2.0]];
        let result = af.matrix_multiply(&mat1, &mat2);
        let expected = vec![vec![4.0, 4.0], vec![10.0, 8.0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_transpose() {
        let af = ArithmeticFunctions;
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = af.compute_transpose(&matrix);
        let expected = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_factorial() {
        let af = ArithmeticFunctions;
        assert_eq!(af.compute_factorial(5), 120);
        assert_eq!(af.compute_factorial(0), 1);
    }


}
