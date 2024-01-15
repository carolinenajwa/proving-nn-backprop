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

