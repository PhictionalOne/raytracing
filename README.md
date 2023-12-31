# Raytracing in One Weekend _with Rust_

This Rust project is a implementation of the "Raytracing in One Weekend" tutorial by Peter Shirley. The tutorial serves as a foundational resource for understanding the principles of ray tracing. You can find the tutorial [here](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## Overview

The project aims to implement a basic ray tracer in Rust, following the principles outlined in the tutorial. It serves as an educational endeavor to grasp the fundamentals of ray tracing and gain proficiency in Rust programming.

## Features

- Basic ray tracing principles as per "Raytracing in One Weekend."
- Utilizes Rust programming language for implementation.

## Learning Objectives

This project serves as a practical exercise to achieve the following learning objectives:

1. **Rust Proficiency:** Strengthen Rust programming skills through the implementation of a ray tracer.
2. **Unit Tests:** Develop and apply unit tests to ensure the correctness of individual components.
3. **Integration Tests:** Implement integration tests to verify the functionality of the complete ray tracing system.
4. **GitHub Actions:** Utilize GitHub Actions for continuous integration to automate testing processes.
5. **Rust Documentation:** Comment modules, structs and functions using Rust Docstrings for automatic API documentation.

## Usage

To generate an image using the ray tracer, run the example application:

```bash
cd examples/balls
cargo run > image.ppm
```

This will produce the output image based on the implemented ray tracing algorithm.

## Tests

Unit and integration tests are available to ensure the correctness and reliability of the ray tracer. Run the tests using:

```bash
cargo test
```

## Documentation

### Generating Documentation

To generate documentation for this project, you can use `cargo doc`. Make sure you have Rust and Cargo installed. Run the following command in the project directory:

```bash
cargo doc --open
```

The `--open` flag opens the documentation in your default web browser after the generation is complete.

### Viewing Documentation

After running the cargo doc command, you can navigate to the `target/doc` directory and open the `index.html` file in your browser. Alternatively, the `--open` flag will automatically open the documentation for you.

## Continuous Integration

GitHub Actions is set up to automate the testing process on every push. Check the workflow status in the Actions tab on GitHub.

## Contribution
Contributions are welcome! Feel free to open issues, propose features, or submit pull requests to enhance the project.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
