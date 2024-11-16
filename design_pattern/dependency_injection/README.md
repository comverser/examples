# Dependency Injection in Rust

This repository contains source code from a YouTube lecture that explains dependency injection in Rust. The lecture can be found [here](https://youtu.be/TdNAqobMMWg).

## Overview

The code demonstrates how to implement dependency injection in a Rust web application using the `axum` framework. The example includes:

- A `ProductRepo` trait that defines the interface for a product repository.
- An `InMemoryProductRepo` struct that implements the `ProductRepo` trait.
- An `AppState` struct that holds the injected dependency.
- Handler functions that use the injected `ProductRepo` to handle HTTP requests.

## Project Structure

- `main.rs`: The main entry point of the application. It sets up the routes and injects the `InMemoryProductRepo` into the application state.
- `product_repo.rs`: Contains the `ProductRepo` trait and the `InMemoryProductRepo` implementation.
- `model.rs`: Defines the `Product` struct.

## Running the Application

To run the application, use the following command:

```sh
cargo run
