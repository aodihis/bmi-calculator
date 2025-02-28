﻿# BMI Calculator API

## Overview
The **BMI Calculator API** is a service to calculate the Body Mass Index (BMI) and classify it 
based on user-provided height and weight. 

## Project Structure

The project follows a modular structure for maintainability and scalability:

```
├── src
│   ├── errors          # Custom error handling
│   │   ├── mod.rs
│   │   ├── api.rs
│   ├── handlers        # Request handlers
│   │   ├── api         # Request handlers for API
│   │   │   ├── mod.rs
│   │   │   ├── bmi_handler.rs
│   │   ├── mod.rs
│   │   ├── error_handler.rs
│   ├── models          # Data models
│   │   ├── mod.rs
│   │   ├── bmi.rs
│   │   ├── generic.rs
│   ├── routes          # API route definitions
│   │   ├── mod.rs
│   │   ├── bmi_route.rs
│   ├── utils           # Helper functions
│   │   ├── mod.rs
│   │   ├── bmi.rs
│   │── config.rs
│   ├── main.rs         # Main entry point
├── tests               # Integration tests
│   ├── integration_test_bmi.rs
├── Cargo.toml          # Project dependencies
├── Makefile            # Make commands
├── README.md           # Documentation
```


### Endpoints
The API can be accessed in the following endpoints:

#### Calculate BMI and Classification
- **Endpoint:** `/api/bmi`
- **Method:** `POST`
- **Description:** Computes and classify BMI based on height (in meters) and weight (in kilograms).
- **Request Body:**
  ```json
  {
    "height": 1.61,
    "weight": 55
  }
  ```
- **Response:**
  ```json
  {
    "status": "success",
    "data": {
             "bmi": 21.22,
             "classification": "Healthy weight"
          }
  }
  ```
  
- **Error Response (Invalid parameter(s)):**
  `Status Code: 400`
```json
  {
     "status": "error",
     "message": "Invalid parameters"
  }
  ```
  
### BMI Calculation Logic
BMI formula:

[ BMI = weight/height^2 ]

The classification follows these BMI categories:

| BMI Range   | Category      |
|-------------|---------------|
| < 18.5      | Underweight   |
| 18.5 - 24.9 | Normal weight |
| 25.0 - 29.9 | Overweight    |
| 30.0 - 34.9 | Obese I       |
| 35.0 - 39.9 | Obese II      |
| \>= 40.0    | Obese III     |

### HTTP Method and Parameter Handling : 
`POST` with JSON Input is used for the consistency between the data input and output. Additionally, it ensures 
the valid/safe type when handling the float number. For example, decimal number can be represented 
by using comma and point. Thus, by using JSON can reduce this ambiguous.

## Running the API

### Prerequisites
- Ensure you have Rust installed. You can install it using:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Optional
Install [makefile](https://makefiletutorial.com/) in your system .

### Build and Run
1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd bmi_calculator
   ```
2. Copy and rename .env.example to .env. Update it accordingly.
3. Build the project (Optional):
   ```sh
   cargo build
   ```
   or 
    ```sh
   make build
   ```
4. Run the API:
   ```sh
   cargo run
   ```
   or

    ```sh
   make run
   ```
### Run in dev mode (watch) (Optional)
1. Install cargo watch
   ```sh
   cargo install cargo-watch
   ```
   or
    ```sh
   make install-tools
   ```
2. Run cargo watch
   ```sh
   cargo watch -w src/ \
		-w .env \
		-x clippy \
		-x 'test -- --nocapture' \
		-x 'run -- --color=always'
   ```
   or
    ```sh
   make watch
   ```

### Testing the API
Run tests:
```sh
  cargo test -- --nocapture
```
or using make command:
```sh
  make test
```

### Run Lint
Run lint checker:
```sh
  cargo fmt --all -- --check
  cargo clippy -- -D warnings
```
or using make command:
```sh
  make lint
```
