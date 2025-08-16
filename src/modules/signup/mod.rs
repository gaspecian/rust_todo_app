//! # SignUp Module
//! This module handles user signup functionality, including
//! user registration, validation, and storage.

pub mod interfaces;
pub mod repository;
pub mod routes;
pub mod service;

pub use routes::signup_routes;