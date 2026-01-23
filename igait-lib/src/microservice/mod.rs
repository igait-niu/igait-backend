//! Microservice types and utilities for the iGait pipeline.
//! 
//! This module provides shared types, traits, and utilities used by all
//! stage microservices in the iGait pipeline.

mod types;
mod storage;

#[cfg(feature = "microservice")]
mod server;

pub use types::*;
pub use storage::*;

#[cfg(feature = "microservice")]
pub use server::*;
