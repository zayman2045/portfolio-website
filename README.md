# Rust Full-Stack Portfolio

This web application serves as my development portfolio. The frontend and backend are both written in Rust, demonstrating the language's versatility, safety, and efficiency.

## Overview

### Frontend

The frontend is a client-side rendered single page application built using `yew`, a robust Rust framework for creating multi-threaded web applications that compile to WebAssembly. It utilizes a component-based architecture, offering a reactive and modular user interface. This is similar to React and Vue.js, but with the added benefits of Rust's strong type system and exceptional performance.

The styling is contained within its own module, using a CSS-like syntax written as a raw string literal. It is then parsed by the `stylist` crate to style the functional components. Responsive web design is achieved using CSS media queries and flexbox, ensuring a tailored viewing experience across desktop and mobile devices.

The `yewdux` crate is used for state management across the application. It allows for the persistence of state and can be scoped to local storage, session storage, or to a single window’s current render. In this application, Yewdux stores are used for collecting and submitting form data, displaying JSON response data, and maintaining user sessions across pages.

### Backend

The backend uses `axum`, a Rust web framework designed for highly concurrent and asynchronous web services and API layers. Header and body data from requests are type-checked for compliance and deserialized into structs using extractors. Route handlers return `Result<Response, StatusCode>` types; in successful instances, valid HTTP responses are generated, while errors from database manipulation are mapped to appropriate Axum status codes

It integrates a custom middleware utilizing JWTs (JSON Web Tokens) with the HS256 algorithm to guard routes. Tokens are generated and validated using the `jsonwebtoken` crate, with a 24-hour expiration policy that automatically logs the user out upon expiration. Passwords are securely hashed using the `bcrypt` crate before being stored in the database.

Axum works in conjunction with the `seaorm` crate to programmatically generate SQL queries for a connected Postgres database. SeaORM's object-relational mapper generates "Entity" and "Model" structs to represent tables and rows respectively, providing a type-safe interface for data manipulation.

### Deployment

During the development phase, the `trunk serve` and `cargo-watch` commands are used to serve the application. This setup facilitates hot reloading and configures a local proxy system when necessary for an efficient development workflow. The frontend, backend, and database are coordinated and deployed using Docker & Docker Compose.

In production, the compiled frontend WebAssembly and associated static files are served with Nginx, ensuring high performance and reliability. Both the frontend and backend leverage a two-stage Docker image build process, resulting in significantly smaller final image sizes. The entire application stack is then deployed on AWS with the Postgres database being managed through RDS, and front and backend running as highly available and scalable EC2 instances managed by Elastic Beanstalk. This cloud-based deployment strategy ensures seamless distribution and management of the services.

## Explore the Project

### Visit the Live Application:

Experience the live version of my project by following the link below. This will take you directly to the deployed application, allowing you to interact with its features in real-time.

[Link to the live application](http://xaviergriffith.com/)

### To run this project locally:

1. Make sure you have Docker and Docker Compose installed. If not, you can download and install Docker from [here](https://docs.docker.com/get-docker/) and Docker Compose from [here](https://docs.docker.com/compose/install/).

2. You can build and run the application using the following command from the project root directory:

    ```zsh
    docker-compose up --build
    ``` 

The application will be available at http://localhost:8080.
