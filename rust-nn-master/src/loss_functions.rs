use core::result::Result;
use crate::arithm_functions::ArithmeticFunctions;

struct LossFunctions;

// UNTESTED (01/23/2023) 

impl LossFunctions {

    /// Computes the sine of `x` using `n_terms` terms of the Taylor series expansion.
    ///
    /// # Arguments
    ///
    /// * `x` - The input value in radians.
    /// * `n_terms` - The number of terms to use in the Taylor series expansion.
    ///
    /// # Returns
    ///
    /// The sine of `x` computed using the Taylor series expansion.
    pub fn compute_sin(x: f64, n_terms: usize) -> f64 {
        let mut sine = 0.0;
        for n in 0..n_terms {
            let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
            sine += sign * x.powi(2 * n as i32 + 1) / ArithmeticFunctions::compute_factorial(2 * n as u64) as f64;
        }
        sine
    }

    /// Computes the cosine of `x` using `n_terms` terms of the Taylor series expansion.
    ///
    /// # Arguments
    ///
    /// * `x` - The input value in radians.
    /// * `n_terms` - The number of terms to use in the Taylor series expansion.
    ///
    /// # Returns
    ///
    /// The cosine of `x` computed using the Taylor series expansion.
    pub fn compute_cos(x: f64, n_terms: usize) -> f64 {
        let mut cosine = 0.0;
        for n in 0..n_terms {
            let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
            cosine += sign * x.powi(2 * n as i32) / ArithmeticFunctions::compute_factorial(2 * n as u64) as f64;
        }
        cosine
    }

    /// Computes the hyperbolic tangent (tanh) of `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The input value.
    ///
    /// # Returns
    ///
    /// The hyperbolic tangent (tanh) of `x`.
    pub fn compute_tanh(x: f64) -> f64 {
        let e_x = ArithmeticFunctions::compute_exp(x);
        let e_neg_x = ArithmeticFunctions::compute_exp(-x);
        (e_x - e_neg_x) / (e_x + e_neg_x)
    }

    /// Computes the derivative of the hyperbolic tangent (tanh) function at `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The input value.
    ///
    /// # Returns
    ///
    /// The derivative of tanh at `x`.
    pub fn compute_tanh_prime(x: f64) -> f64 {
        let tanh_x = Self::compute_tanh(x);
        1.0 - tanh_x.powi(2)
    }

    /// NOTE: Unused functions
    // pub fn compute_sigmoid(x: f64) -> f64 {
    //     1.0 / (1.0 + ArithmeticFunctions::compute_exp(-x))
    // }
    
    // pub fn compute_sigmoid_prime(x: f64) -> f64 {
    //     let sigmoid_x = compute_sigmoid(x);
    //     sigmoid_x * (1.0 - sigmoid_x)
    // }

    /// Computes the Mean Squared Error (MSE) between two sequences of values.
    ///
    /// # Arguments
    ///
    /// * `y_true` - The true values.
    /// * `y_pred` - The predicted values.
    ///
    /// # Returns
    ///
    /// The Mean Squared Error between `y_true` and `y_pred`.
    pub fn mse(y_true: &[f64], y_pred: &[f64]) -> f64 {
        y_true.iter().zip(y_pred.iter())
            .map(|(yt, yp)| (yt - yp).powi(2))
            .sum::<f64>() / y_true.len() as f64
    }

    /// Computes the derivative of Mean Squared Error (MSE) with respect to `y_pred`.
    ///
    /// # Arguments
    ///
    /// * `y_true` - The true values.
    /// * `y_pred` - The predicted values.
    ///
    /// # Returns
    ///
    /// A vector containing the derivatives of MSE with respect to `y_pred`.
    pub fn mse_prime(y_true: &[f64], y_pred: &[f64]) -> Vec<f64> {
        y_true.iter().zip(y_pred.iter())
            .map(|(yt, yp)| 2.0 * (yp - yt) / y_true.len() as f64)
            .collect()
    }


}
