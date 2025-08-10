//! # Health Check Module
//!
//! This module provides health check functionality for the application,
//! including endpoints to verify the application's status and readiness.

pub mod interfaces;
pub mod routes;
pub mod service;

pub use routes::health_routes;
