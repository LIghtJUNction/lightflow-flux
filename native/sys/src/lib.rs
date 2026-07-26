#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(clippy::missing_safety_doc, clippy::ptr_offset_with_cast)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
#[path = "../build_support.rs"]
mod build_support;
