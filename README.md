# Computational Attestations of Polynomial Integrity Towards Verifiable Machine-Learning #

We wish to understand the costs of deploying traditional backpropagation in such a manner as to cryptographically prove the successful completion of the training algorithm.

## Project Overview
This project aims to provide a clear and educational implementation of the backpropagation algorithm in Python and upon completion, will be written into Rust. We wish to measure the concrete costs of this algorithm translated into a computational integrity statement (ZKP). There are other attempts at using zero-knowledge proofs for various types of machine-learning, but we wish to understand how training algorithms in particular can be proven.

## Backpropagation Status ## 
This project is currently in the design and development phase. We are actively working on the following aspects:
- [ ] Implement a simple, single-layer backpropagation mechanism first in python, then in rust
- [ ] translate backpropagation into an intermediate polynomial representation

## Proof Mechanism and Concrete Asymptotics
Strictly speaking, we desire a prover and verifier with $O(n)$ and $log(n)$ runtime respectively. Choice of commitment scheme will reflect these design constraints.
The following will be implemented:
- [ ] A polynomial type with all of the usual operations
- [ ] A merkle tree, ideally leveraging a parallel construction
- [ ] Number-theoretic transform (NTT) or Fast-fourier transform (FFT)
- [ ] FRI protocol as the core of the argument scheme

# Neural Network Training Process: One Neuron Per Layer

We describe a multi-layer perceptron with parameters:
- the initial state $i$
- the updated state $i +1$
- weight $w$
- bias $b$
- sigmoid activation $\sigma = \frac{1}{1 + e^{-x}}$
- loss as mean-squared-error (MSE)
- learning rate $\eta$

Then we describe the network across states in a fairly straightforward manner:

Output = $\sigma(w_{i+1} \cdot \sigma(w_i \cdot \text{input} + b_i) + b_{i+1})$

Loss = $\frac{(\text{target - output})^2}{2}$

$w = w - \eta \cdot \frac{\partial \text{loss}}{\partial w} $

$b = b - \eta \cdot \frac{\partial \text{loss}}{\partial b} $


## Forward Pass
The forward pass involves computing the output of the network given an input. The process is as follows:

$z_i = w_i \cdot \text{input} + b_i$

$a_i = \sigma(z_i)$

$z_{i+1} = w_{i+1} \cdot a_i + b_{i+1}$

$\text{output} = \sigma(z_{i+1})$

## Loss Calculation
The loss function used is the mean squared error, calculated as:

$\text{loss} = \frac{(\text{target} - \text{output})^2}{2}$

## Backward Pass (Gradient Calculation)
During the backward pass, we calculate the gradient of the loss function with respect to each weight and bias:


$\frac{\partial \text{loss}}{\partial w_{i+1}} = (\text{output} - \text{target}) \cdot \sigma'(z_{i+i}) \cdot a_i$

$\frac{\partial \text{loss}}{\partial b_{i+1}} = (\text{output} - \text{target}) \cdot \sigma'(z_{i+1})$

$\frac{\partial \text{loss}}{\partial w_{i}} = (\text{output} - \text{target}) \cdot \sigma'(z_{i+1}) \cdot w_{i+1} \cdot \sigma'(z_i) \cdot \text{input}$

$\frac{\partial \text{loss}}{\partial b_{i}} = (\text{output} - \text{target}) \cdot \sigma'(z_{i+1}) \cdot w_{i+1} \cdot \sigma'(z_{i})$


## Weight Update
The weights and biases are updated using gradient descent, as follows:

$w_i = w_i - \eta \cdot \frac{\partial \text{loss}}{\partial w_i}$

$b_i = b_i - \eta \cdot \frac{\partial \text{loss}}{\partial b_i}$

$w_{i+1} = w_{i+1} - \eta \cdot \frac{\partial \text{loss}}{\partial w_{i+1}}$

$b_{i+1} = b_{i+1} - \eta \cdot \frac{\partial \text{loss}}{\partial b_{i+1}}$
