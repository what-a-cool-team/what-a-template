# what-a-template

Rust web app template

## Table of Contents

- [Overview](#overview)
- [Structure](#structure)
- [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Introduction

The "what-a-template"  is a Rust web app template designed to help developers quickly set up a new web application. It
includes a pre-configured project structure, necessary dependencies, and example configurations to streamline the
development process.

## Structure

### Overview
- `api/`: Contains different APIs such as the models and controllers that are used in multiple projects.
- `app/`: Contains the main application logic.
    - `migrations/`:
        - Example files: `20210707120000_create_users_table.sql`: SQL script to create the users table.
    - `main.rs`: The main entry point for the application.
- `domain/`: Contains core business logic and domain models for the application. The domain layer typically abstracts
  and encapsulates the business logic, making it independent of the infrastructure and application layers. For more
  information, visit https://opus.ch/en/category/ddd-en/.
    - `models/`: Domain models used for the project.
    - `repositories/`: Repositories that save/load any persistent information.
    - `services/`: Service logics for the project. e.g. `greeting_service.rs`
- `settings/`: Defines and loads configuration settings.
  - Do not push any personal information from the `settings.toml` to a public worksapce.

### Flowchart
![what-a-flowchart](https://github.com/user-attachments/assets/a1dae913-be68-4c66-8d4a-f915b7e82f8c)

## Prerequisites

- [Rustup (Recommended)](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Docker](https://www.docker.com/)

## Usage

### Running App

#### Using Cargo

```bash
cargo run --package app --bin app -- --config settings.toml
```

#### Using Docker

```bash
docker compose up
```

### Running Tests

```bash
cargo test
```

## Contributing

## License

## Contact
