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

pub mod visibility;

pub mod resource_geometry;

pub mod annual_irradiance;

pub mod fixed_geometry;

pub mod origami;

pub mod electrical;

pub mod thermal;

pub mod thermal_electrical;

pub mod annual_electrical_adapter;
