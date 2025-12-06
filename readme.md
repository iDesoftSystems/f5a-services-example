# F5a Services

This repository includes the sample code developed during the [Web Service Development](https://www.youtube.com/playlist?list=PLSLcKcqBWfjJut_tXCtnMS8l54P2YoEoM) course, covering the following topics.

- Module 1: Introduction to Web APIs and the Rust Ecosystem
- Module 2: Axum - Creating Your First Web Server
- Module 3: Axum - Handling HTTP Requests and Extracting Data
- Module 4: Sea-ORM
- Module 5: Developing a Complete CRUD RESTful API
- Module 6: Enforcing Architecture
- Module 7: Observability
- Module 8: API Integration Testing
- Module 9: API Documentation with OpenAPI (Swagger)
- Module 10: CI Pipeline with GitHub Actions

## Getting started from scratch

1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Install [Docker](https://www.docker.com/)
3. Install [just](https://github.com/casey/just)

   `cargo install just`

4. Install [sea-orm-cli](https://github.com/SeaQL/sea-orm)

   `cargo install sea-orm-cli`

5. Compose docker services

    `just docker-compose`

6. Create an .envfile

    `cp .env-example .env`

7. Run migrations

    `just migrate-fresh`

8. Start the services

    `just start`
