//! This module provides utilities for managing uniform data in the renderer.
//!
//! It includes:
//! - [`UniformBindingData`]: A high-level wrapper for wgpu uniform buffers and bind groups.
//! - [`DynamicUniformBuffer`]: Metadata and code generation for dynamically defined uniform structs.

mod binding_data;
mod dynamic_uniform;

pub use binding_data::UniformBindingData;
pub use dynamic_uniform::{DynamicUniformBuffer, DynamicUniformBufferBuilder, UniformType};

