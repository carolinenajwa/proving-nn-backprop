
use core::result::Result;

// Notes:
// Functions to translate:
// 1. dot_product (done) 
// 2. matrix_multiply (done) 
// 3. compute_transpose (done) 
// 4. compute_factorial (done) 
// 5. compute_ipow
// 6. compute_exp
// 7. isinstance



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

    pub fn matrix_multiply(&self, mat1: &[Vec<f64>], mat2: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut result = Vec::new();

        for i in 0..mat1.len() {
            let mut row_result = Vec::new();
            for j in 0..mat2[0].len() {
                let row = &mat1[i];
                let col: Vec<f64> = mat2.iter().map(|r| r[j]).collect();
                row_result.push(self.dot_product(row, &col).unwrap()); // Assuming dot_product is implemented
            }
            result.push(row_result);
        }

        result
    }

    pub fn compute_transpose(&self, matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }

        let mut transposed_matrix = Vec::new();

        for i in 0..matrix[0].len() {
            let mut row = Vec::new();
            for j in 0..matrix.len() {
                row.push(matrix[j][i]);
            }
            transposed_matrix.push(row);
        }

        transposed_matrix
    }

    pub fn compute_factorial(&self, n: u64) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n * self.compute_factorial(n - 1),
        }
    }



}



