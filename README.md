# Portfolio Website

This web application serves as my development portfolio. It's written in Rust for both the frontend and backend, demonstrating the language's versatility, safety, and efficiency.

## Overview

The frontend is built using Yew, a robust Rust framework for creating multi-threaded web applications that compile to WebAssembly. It utilizes a component-based architecture, offering a reactive and modular user interface. This is similar to React and Vue.js, but with the added benefits of Rust's strong type system and exceptional performance.

The backend uses Axum, a Rust web framework designed for highly concurrent and asynchronous web services and API layers. It works in conjunction with the SeaORM crate to programmatically generate SQL queries for a connected Postgres database. The frontend, backend, and database are coordinated and deployed using Docker & Docker Compose.

## Getting Started

### To run this project locally: