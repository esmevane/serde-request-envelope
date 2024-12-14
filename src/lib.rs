#![deny(missing_docs, missing_debug_implementations, rust_2024_compatibility)]
//! # Serde Request Envelope
//!
//! This crate provides the `Request` struct, which is a newtype wrapper that takes
//! any given serde friendly type and turns it into a request envelope that includes
//! the type name of the given type. This lets you do tagged structures without having
//! to manually curate enums.
//!
//! For example:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_request_envelope::Request;
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct MyStruct {
//!    field: String,
//! }
//!
//! # fn main() {
//! let my_struct = MyStruct {
//!    field: "Hello, World!".to_string(),
//! };
//!
//! let request = Request::new(my_struct);
//!
//! let serialized = serde_json::to_string(&request).unwrap();
//! // serialized is now: {"type":"MyStruct","data":{"field":"Hello, World!"}}
//!
//! let deserialized: Request<MyStruct> = serde_json::from_str(&serialized).unwrap();
//! assert_eq!(deserialized.0.field, "Hello, World!");
//! # }
//! ```
//!

mod request;
mod support;

pub use request::Request;
