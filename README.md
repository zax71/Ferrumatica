# Ferrumatica

A Rust based calculator api service

## Routes

- POST `/add`
  - Request:
  ```json
  {
    "x": 5,
    "y": 10
  }
  ```
  - Response:
  ```json
  {
    "result": 15
  }
  ```
- POST `/subtract`
  - Request:
  ```json
  {
    "x": 10,
    "y": 5
  }
  ```
  - Response:
  ```json
  {
    "result": 5
  }
  ```
