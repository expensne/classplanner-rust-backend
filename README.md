# Classplanner Backend (Rust)

Classplanner is a simple application for managing student and exam data. It allows users to create, read, update, and delete students and exams. The backend is built with Rust and the axum web framework, and it uses MongoDB for data storage.

## Structure

`consts.rs`: This file likely contains constant values used throughout the project.

`custom.rs`: This file may contain custom types or functions used in the project.

`databases/`: This directory contains the database-related code, including a general database.rs and a specific mongo_database.rs for MongoDB operations.

`endpoints/`: This directory contains the API endpoints for the service, including exam.rs and student.rs.

`helper.rs`: This file likely contains helper functions used throughout the project.

`main.rs`: This is the entry point of the application.

`models/`: This directory contains the data models for the service, including exam.rs and student.rs.

The project uses several dependencies, including serde for serialization and deserialization, futures and tokio for asynchronous programming, mongodb for MongoDB operations, dotenv for environment variable management, axum for web service development, and tracing and tracing-subscriber for application-level tracing.

## Justfile

The project is set up with a justfile for task running. It includes tasks for running the application, building the application for local and Linux environments, and publishing the application to a remote server.
