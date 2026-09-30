//! API-only backend for the UTME Lab workspace.
//!
//! This crate is a library plus a thin `server` binary. The binary's only job
//! is to mount the server functions declared in the `api` crate and serve
//! them; it deliberately does not render pages, so it stays API-only even
//! though it links the fullstack machinery.
//!
//! Backend internals (database queries, token minting, hashing) belong here
//! as library functions, and must never be imported by `ui` or `core`.
