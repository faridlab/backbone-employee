// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

pub mod employee_public_handler;
pub mod guarded_routes;
pub use employee_public_handler::create_employee_public_read_routes;
