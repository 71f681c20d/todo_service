# todo_service
Rust API backend for the to-do app

Description:
The Todo List API allows users to manage a list of tasks. Each task has a title, description, and a completion status.

API Endpoints:
1. GET /tasks:
   - Retrieves a list of all tasks.
   - Response: JSON array of tasks.
2. POST /tasks:
   - Creates a new task.
   - Request: JSON object containing title, description, and completion status.
   - Response: JSON object representing the created task.
3. GET /tasks/{id}:
   - Retrieves a specific task by its ID.
   - Response: JSON object representing the task.
4. PUT /tasks/{id}:
   - Updates a specific task by its ID.
   - Request: JSON object containing updated title, description, and/or completion status.
   - Response: JSON object representing the updated task.
5. DELETE /tasks/{id}:
   - Deletes a specific task by its ID.
   - Response: Empty response with a 204 status code.

#### Logic
1. Structuring Data: Create Rust structs to represent your API resources such as tasks. Use the `impl` block to define methods on these structs for operations like creating, updating, or deleting tasks. This will allow you to work with owned and borrowed references within the context of these operations.
2. Memory ownership and borrowing

#### Libraries/Dependencies:
You can use the following Rust libraries to build the project:
- Actix-Web: A lightweight, high-performance web framework for building REST APIs.
- Diesel: A query builder and ORM (Object Relational Mapping) for interacting with the Postgres database.
- dotenv: For loading environment variables from a .env file.
- serde: For serialization and deserialization of JSON data.

#### Testing:
You can write unit tests and integration tests using the built-in Rust testing framework. You can test each API endpoint for both success and error scenarios.

#### Documentation:
Consider generating API documentation using tools like Swagger or Rust-specific libraries like `actix-web-doc` to automatically generate API documentation from your code annotations.



