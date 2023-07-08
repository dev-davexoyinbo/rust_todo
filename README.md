# Todo CRUD API

![GitHub](https://img.shields.io/github/license/dev-davexoyinbo/rust_todo)
![GitHub issues](https://img.shields.io/github/issues/dev-davexoyinbo/rust_todo)

This is a simple todo API written in rust.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Tests](#tests)
- [Running the project](#running-the-project)
- [Database model diagram](#database-model-diagram)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

| TASKS | STATUS |
|-------|--------|
| Create database models | :construction: In progress |
| Define endpoints | :grey_exclamation: Not Started |
| Create tests for endpoints | :grey_exclamation: Not Started |
| Implement endpoints | :grey_exclamation: Not Started |


## Installation

```bash
cargo install
```

## Tests
```bash
cargo test
```

## Running the project
```bash
cargo run
```

## Database model diagram
<img width="423" alt="image" src="https://github.com/dev-davexoyinbo/rust_todo/assets/67616046/fc5f3b93-7bdc-4c1a-9bee-2ec2be04d8fe">



## License

This project is licensed under the [MIT License](LICENSE.md).

## Acknowledgments

---
This project was developed using the following
- Rust programming language
- Actix web framework




<br />
<br />
<br />
<br />
<br />





# API Documentation
## Endpoints

### Endpoint 1
- **URL**: `/todos`
- **Method**: `GET`
- **Description**: Get all todo items
- **Parameters**: 
    - page: Defines the current page (default: 1)
    - per_page: Defines the number of todos per page (default: 20)
- **Request Example**:
  ```bash
  GET /todos?page=2&per_page=20
  ```
- **Response Example**:
  ```json
  HTTP/1.1 200 OK
  Content-Type: application/json
  
  {
    "message": "Todos",
    "data": {
      "values": [
        {
            "id": 1,
            "title": "This is the title 1",
            "body": "This is the body of the todo"
            status varchar
            created_at timestamp
            updated_at timestamp
        },
        ...
      ]
    }
  }
  ```

### Endpoint 2
- **URL**: `/endpoint2`
- **Method**: `POST`
- **Description**: Describe the purpose and functionality of this endpoint.
- **Parameters**: Specify the request body parameters, their types, and whether they are required or optional.
- **Request Example**:
  ```
  POST /endpoint2
  Content-Type: application/json

  {
    "param1": "value1",
    "param2": "value2"
  }
  ```
- **Response Example**:
  ```
  HTTP/1.1 201 Created
  Content-Type: application/json
  
  {
    "message": "Resource created successfully",
    "data": {
      "id": "12345",
      "param1": "value1",
      "param2": "value2"
    }
  }
  ```