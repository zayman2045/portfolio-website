# Rust Full-Stack Portfolio

This web application serves as my development portfolio. The frontend and backend are both written in Rust, demonstrating the language's versatility, safety, and efficiency.

## Overview

The frontend is a client-side rendered single page application built using Yew, a robust Rust framework for creating multi-threaded web applications that compile to WebAssembly. It utilizes a component-based architecture, offering a reactive and modular user interface. This is similar to React and Vue.js, but with the added benefits of Rust's strong type system and exceptional performance. The styling is contained within its own module, using a CSS-like syntax. This syntax is written as a raw string literal, which is then parsed by the Stylist crate to style the functional components.

Yewdux is used for state management across the application. It allows for the persistence of state and can be scoped to local storage, session storage, or to a single window’s current render. In this application, Yewdux stores are used for collecting and submitting form data, displaying JSON response data, and maintaining user sessions across pages.

The backend uses Axum, a Rust web framework designed for highly concurrent and asynchronous web services and API layers. It works in conjunction with the SeaORM crate to programmatically generate SQL queries for a connected Postgres database. The SeaORM's object relational mapper uses "Entity" and "Model" structs to represent tables and rows respectively, providing a type-safe interface for data manipulation.

The frontend, backend, and database are coordinated and deployed using Docker & Docker Compose.

## Getting Started

### To run this project locally:

Make sure you have Docker and Docker Compose installed. If not, you can download and install Docker from [here](https://docs.docker.com/get-docker/) and Docker Compose from [here](https://docs.docker.com/compose/install/).

Once Docker and Docker Compose are installed, you can build and run the application using the following command from the project root directory:

```zsh
docker-compose up --build
``` 

The application will be available at http://localhost:8080.
