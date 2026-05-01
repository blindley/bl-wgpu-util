//! A module for defining and managing dynamic vertex layouts at runtime.
//!
//! This module provides tools to describe vertex structures, build WGPU vertex buffer layouts,
//! and automatically generate corresponding WGSL shader code. It is designed to be flexible,
//! allowing attributes to be added or modified without recompiling the core rendering logic.

mod attribute;
mod vertex;
mod wgsl;

pub use vertex::{
    DynamicVertexDescriptor, DynamicVertexDescriptorBuilder, NamedAttributeDescriptor,
};
