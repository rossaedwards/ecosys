//! ═══════════════════════════════════════════════════════════════════
//! 🔐 AuraFS Access Control List Module
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//!
//! Provides fine-grained permission management with role-based access control,
//! resource-level permissions, and quantum-safe authentication support.
//!
//! ## Features
//! - Role-Based Access Control (RBAC)
//! - Per-resource ACL entries
//! - Quantum-safe signature verification
//! - Zero-knowledge proof support
//! - Configuration persistence
//! ═══════════════════════════════════════════════════════════════════

pub mod manager;

pub use manager::{AclManager, AclError, AclConfig, Role, User, ResourceAcl};