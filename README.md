# Rust Undefined Behavior: Modifying Vector During Iteration

This repository demonstrates a common error in Rust: modifying a vector while iterating over it using a mutable reference.  This leads to undefined behavior, meaning the program's outcome is unpredictable.  The provided code showcases the issue and a solution using cloning or creating a new vector for modifications.

## Bug Description
The `bug.rs` file contains a program that pushes elements onto a vector, then iterates over it using an iterator.  However, it attempts to modify the vector (pushing another element) while the iteration is in progress. This results in undefined behavior.

## Solution
The `bugSolution.rs` file presents a corrected version. Instead of directly modifying the original vector, it creates a copy or handles modifications in a way that ensures iterator validity.