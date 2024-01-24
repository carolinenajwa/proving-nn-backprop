
use core::result::Result;

// Notes:
// Functions to translate:
// 1. dot_product (done) 
// 2. matrix_multiply (done) 
// 3. compute_transpose (done) 
// 4. compute_factorial (done) 
// 5. compute_ipow (done) 
// 6. compute_exp (done) 





pub struct ArithmeticFunctions;

impl ArithmeticFunctions {
 
    // Compute the dot product of two 1-dimensional lists.
    // Args:
    // a (list): A 1-dimensional list of numbers.
    // b (list): A 1-dimensional list of numbers.
    // Returns:
    // float: The dot product of the two lists.
    // Raises:
    // ValueError: If the input lists are not of equal length.
    pub fn dot_product(&self, a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        // Check if both lists are of the same length
        if a.len() != b.len() {
            return Err("Both slices must be of the same length");
        }
        // Define mutable variable to store the dot product
        let mut dot_product = 0.0;

        // Iterate over the slices
        for (i, &item_a) in a.iter().enumerate() {
            let item_b = b[i]; // Get the corresponding item from b
            dot_product += item_a * item_b; // Add the product to the dot product
        }

        Ok(dot_product) // Return the dot product
    }
    
    // Test the dot product function
    pub fn run_dot_product_test() {
        let a1: &[f64] = &[1.0, 2.0, 3.0]; 
        let b1: &[f64] = &[4.0, 5.0, 6.0];

        let af = ArithmeticFunctions; 
        let result = af.dot_product(a1, b1); 
       
       // Print the result
        match result {
            Ok(dot_product) => println!("Test 1: Dot Product: {}", dot_product),
            Err(e) => println!("Error: {}", e),
        }
        println!("Test 1: Dot Expected: 32.00");
    }

    pub fn matrix_multiply(&self, mat1: &[Vec<f64>], mat2: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, &'static str> {
        if mat1.is_empty() || mat2.is_empty() || mat1[0].is_empty() || mat2[0].is_empty() {
            return Err("Input matrices cannot be empty");
        }

        if mat1.iter().any(|row| row.len() != mat1[0].len()) || mat2.iter().any(|row| row.len() != mat2[0].len()) {
            return Err("Rows in each matrix must have uniform length");
        }
    
        let num_cols_mat1 = mat1[0].len();
        let num_rows_mat2 = mat2.len();

        if num_cols_mat1 != num_rows_mat2 {
            return Err("Number of columns in mat1 must be equal to the number of rows in mat2");
        }

        let num_rows_result = mat1.len();
        let num_cols_result = mat2[0].len();
    
        let mut result = vec![vec![0.0; num_cols_result]; num_rows_result];
    
        for i in 0..mat1.len() {
            for j in 0..mat2[0].len() {
                let row = &mat1[i];
                let col: Vec<f64> = mat2.iter().map(|r| r[j]).collect();
                result[i][j] = self.dot_product(row, &col)?;
            }
        }

        Ok(result)

    }

    pub fn compute_transpose(&self, matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if matrix.is_empty() || matrix.iter().any(|row| row.len() != matrix[0].len()) {
            return Vec::new(); // Return an empty matrix if input is empty or rows are not uniform
        }
    
        let num_cols = matrix[0].len();

        let mut transposed_matrix = Vec::with_capacity(num_cols);
        
        for i in 0..num_cols {
            let mut row = Vec::with_capacity(matrix.len());
            for j in 0..matrix.len() {
                row.push(matrix[j][i]);
            }
            transposed_matrix.push(row);
        }
    
        transposed_matrix
    }

    pub fn compute_factorial(&self, n: u64) -> Result<u64, &'static str> {
        if n > 20 {
            Err("Input value is too large for factorial calculation")
        } else {
            Ok((1..=n).product())
        }
    }


    pub fn compute_ipow(&self, x: i32, y: u32) -> Option<i32> {
        let mut res: i32 = 1;
        for _ in 0..y {
            res = res.checked_mul(x)?;
        }
        Some(res)
    }
    

    pub fn compute_exp(&self, z: f64, ftol: f64) -> f64 {
        if z == 0.0 {
            return 1.0;
        }
        if z < 0.0 {
            return 1.0 / self.compute_exp(-z, ftol);
        }
    
        let mut res = 1.0;
        let mut term = 1.0; // First term in the series
        let mut i = 1;
        while term >= ftol {
            term *= z / i as f64;
            res += term;
            i += 1;
        }

        res
    }
    

}



