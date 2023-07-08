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

### Get all todo items
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
    "from": 1,
    "last_page": 3,
    "per_page": 10,
    "to": 10,
    "total": 26,
    "data": [
        {
            "id": 1,
            "title": "This is the title 1",
            "body": "This is the body of the todo",
            "status": "pending", // pending, completed,
            "created_at": "2023-07-08 10:30 AM UTC",
            "updated_at": "2023-07-08 10:30 AM UTC",
        },
        ...
      ]
    ]
  }
  ```

### Create todo item
- **URL**: `/todos`
- **Method**: `POST`
- **Description**: Creates the todo item
- **Request Example**:
  ```json
  POST /todos
  Content-Type: application/json

  {
    "title": "value1",
    "body": "value2",
  }
  ```
- **Response Example**:
  ```json
  HTTP/1.1 201 Created
  Content-Type: application/json
  
  {
    "message": "Resource created successfully",
    "data": {
      "id": 3,
    }
  }
  ```

### Get single todo item
- **URL**: `/todos/:id`
- **Method**: `GET`
- **Description**: Get all todo items
- **Request Example**:
  ```bash
  GET /todos/:id
  ```
- **Response Example**:
```json
  HTTP/1.1 200 OK
  Content-Type: application/json
  
    {
        "message": "Fetched todo item",
        "data": {
            "id": 1,
            "title": "This is the title 1",
            "body": "This is the body of the todo",
            "status": "pending", // pending, completed,
            "created_at": "2023-07-08 10:30 AM UTC",
            "updated_at": "2023-07-08 10:30 AM UTC",
        },
    }
```

### Delete single todo item
- **URL**: `/todos/:id`
- **Method**: `DELETE`
- **Description**: Delete all todo items
- **Request Example**:
  ```bash
  DELETE /todos/:id
  ```
- **Response Example**:
```json
  HTTP/1.1 200 OK
  Content-Type: application/json
  
    {
        "message": "Deleted successfully",
        "data": {
            "id": 1,
        },
    }
```


### Update single todo item
- **URL**: `/todos/:id`
- **Method**: `PUT`
- **Description**: Get all todo items
- **Request Example**:
  ```json
  PUT /todos/:id
  Content-Type: application/json

    {
        "title": "This is the title 1",
        "body": "This is the body of the todo",
        "status": "pending", // pending, completed,
        "created_at": "2023-07-08 10:30 AM UTC",
        "updated_at": "2023-07-08 10:30 AM UTC",
    }
  ```
- **Response Example**:
```json
  HTTP/1.1 200 OK
  Content-Type: application/json
  
    {
        "message": "Fetched todo item",
        "data": {
            "id": 1,
            "title": "This is the title 1",
            "body": "This is the body of the todo",
            "status": "pending", // pending, completed,
            "created_at": "2023-07-08 10:30 AM UTC",
            "updated_at": "2023-07-08 10:30 AM UTC",
        },
    }
```