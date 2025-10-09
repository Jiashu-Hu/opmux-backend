//! Vendor integrations for LLM APIs.
//!
//! This module contains vendor-specific implementations for different LLM providers.
//! Each vendor implements the `LLMVendor` trait defined in `traits.rs`.

pub mod traits;

// Vendor implementations will be added in later tasks
// pub mod openai;
// pub mod anthropic;

pub use traits::LLMVendor;

