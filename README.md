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

## Overview

The "what-a-template"  is a Rust web app template designed to help developers quickly set up a new web application. It
includes a pre-configured project structure, necessary dependencies, and example configurations to streamline the
development process.

## Structure

- `api/`:
- `app/`: Contains basic app functionalities.
    - `migrations/`:
        - Example files: `20210707120000_create_users_table.sql`: SQL script to create the users table.
    - `main.rs`: The main entry point for the application.
- `domain/`: Contains core business logic and domain models for the application. The domain layer typically abstracts
  and encapsulates the business logic, making it independent of the infrastructure and application layers. For more
  information, visit https://opus.ch/en/category/ddd-en/.
    - `models/`:
    - `repositories/`:
    - `services/`:
- `settings/`: Defines and loads configuration settings.

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
