#![cfg_attr(not(creusot), feature(rustc_attrs))]

extern crate self as rml_contracts;

pub use crate::macros::*;

#[cfg(not(rml))]
mod macros {
    pub use rml_proc::spec;
}

#[cfg(rml)]
mod macros {
    pub use rml_proc_dummy::spec;
}
