
use core::result::Result;
use rand::Rng;
use crate::arithm_functions::ArithmeticFunctions;

// UNTESTED (01/23/2023) 

pub trait AbstractLayer {
    fn forward_propagation(&self, input: Vec<Vec<f64>>) -> Vec<Vec<f64>>;
    fn backward_propagation(&self, output_error: Vec<Vec<f64>>, learning_rate: f64) -> Vec<Vec<f64>>;
}

pub struct FCLayer {
    input: Vec<Vec<f64>>,
    output: Vec<Vec<f64>>,
    weights: Vec<Vec<f64>>,
    bias: Vec<Vec<f64>>,
    arithmetic: ArithmeticFunctions,
}

impl FCLayer {
    // Initialize weights and bias
    pub fn new(input_size: usize, output_size: usize, arithmetic: ArithmeticFunctions) -> FCLayer {
        let mut rng = rand::thread_rng(); // Initialize random number generator
        let weights = (0..input_size) // Create a vector of size input_size
            .map(|_| (0..output_size) // Create a vector of size output_size
                 .map(|_| rng.gen::<f64>() - 0.5) // Generate a random number between -0.5 and 0.5
                 .collect()) // Collect the output of the inner map into a vector
            .collect(); // Initialize weights with random values between -0.5 and 0.5
        let bias = vec![(0..output_size)
                        .map(|_| rng.gen::<f64>() - 0.5)
                        .collect()]; // Initialize bias with random values between -0.5 and 0.5
        
        FCLayer { weights, bias }
    }
}

impl Layer for FCLayer {
    fn forward_propagation(&mut self, input_data: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.input = input_data;
        let product = self.arithmetic.matrix_multiply(&self.input, &self.weights)
            .expect("Matrix multiplication failed"); // Compute the dot product of input and weights

        self.output = product.iter().zip(self.bias.iter())
            .map(|(row, bias_row)| {
                row.iter().zip(bias_row.iter()).map(|(r, b)| r + b).collect()
            })
            .collect(); // Add bias to the dot product
        
        self.output.clone() // Return the output of the layer
    }


    fn backward_propagation(&self, output_error: Vec<Vec<f64>>, learning_rate: f64) -> Vec<Vec<f64>> {
        // Compute the transpose of weights and input data
        let transposed_weights = self.arithmetic.compute_transpose(&self.weights);
        let transposed_input = self.arithmetic.compute_transpose(&self.input);

        // Compute the input error and weights error
        let input_error = self.arithmetic.matrix_multiply(&output_error, &transposed_weights)
            .expect("Matrix multiplication failed in backward_propagation for input_error");
        let weights_error = self.arithmetic.matrix_multiply(&transposed_input, &output_error)
            .expect("Matrix multiplication failed in backward_propagation for weights_error");

        // Update each weight in the weight matrix using gradient descent
        for (w, e) in self.weights.iter_mut().zip(weights_error.iter()) {
            for (weight, error) in w.iter_mut().zip(e.iter()) {
                *weight -= learning_rate * error;
            }
        }
        // Update each bias in the bias matrix using gradient descent
        for (b, e) in self.bias.iter_mut().zip(output_error.iter()) {
            for (bias, error) in b.iter_mut().zip(e.iter()) {
                *bias -= learning_rate * error;
            }
        }

        input_error // Return the input error for the previous layer
    }
}

// Take and return 2D arrays for activation functions
type ActivationFn = Box<dyn Fn(Vec<Vec<f64>>) -> Vec<Vec<f64>>>;
type ActivationPrimeFn = Box<dyn Fn(Vec<Vec<f64>>) -> Vec<Vec<f64>>>;


pub struct ActivationLayer {
    input: Vec<Vec<f64>>,
    output: Vec<Vec<f64>>,
    activation: ActivationFn,
    activation_prime: ActivationPrimeFn,
} 

impl ActivationLayer {
    pub fn new(activation: ActivationFn, activation_prime: ActivationPrimeFn) -> ActivationLayer {
        ActivationLayer {
            activation,
            activation_prime,
            input: Vec::new(), // Initialize input and output with empty vectors
            output: Vec::new(), // Initialize input and output with empty vectors
        }
    }
}

impl Layer for ActivationLayer {
    // Forward propagation for activation layer
    fn forward_propagation(&mut self, input_data: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.input = input_data;
        self.output = (self.activation)(self.input.clone());
        self.output.clone()
    }

    // Backward propagation for activation layer 
    fn backward_propagation(&mut self, output_error: Vec<Vec<f64>>, _learning_rate: f64) -> Vec<Vec<f64>> {
        (self.activation_prime)(self.input.clone()).iter().zip(output_error.iter())
            .map(|(prime_row, error_row)| {
                prime_row.iter().zip(error_row.iter()).map(|(p, e)| p * e).collect()
            })
            .collect()
    }
}
 







