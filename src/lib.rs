//! Core analytical models for the mushroom-head solar research project.
//!
//! The current crate intentionally starts with equations that can be checked
//! analytically. Higher-fidelity irradiance, ray tracing and optimisation are
//! added only after their data provenance and numerical validation are defined.

pub mod geometry;
pub mod solar;

pub mod candidates;

pub mod mesh;

pub mod weather;

pub mod irradiance;

pub mod spa;
