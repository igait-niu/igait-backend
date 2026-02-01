//! Microservice types and utilities for the iGait pipeline.
//! 
//! This module provides shared types, traits, and utilities used by all
//! stage microservices in the iGait pipeline.

mod types;
mod storage;
mod queue;

#[cfg(feature = "microservice")]
mod server;

#[cfg(feature = "microservice")]
mod worker;

#[cfg(feature = "email")]
mod email;

pub use types::*;
pub use storage::*;
pub use queue::*;

#[cfg(feature = "microservice")]
pub use server::*;

#[cfg(feature = "microservice")]
pub use worker::*;

#[cfg(feature = "email")]
pub use email::*;
