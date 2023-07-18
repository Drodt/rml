extern crate self as rml_contracts;

pub use crate::macros::*;

pub mod well_founded;

pub use well_founded::WellFounded;

#[cfg(rml)]
mod macros {
    pub use rml_proc::spec;

    pub use rml_proc::pure;

    pub use rml_proc::strictly_pure;

    pub use rml_proc::invariant;

    pub use rml_proc::modifies;

    pub use rml_proc::variant;

    pub use rml_proc::logic;

    pub use rml_proc::rml;

    pub use rml_proc::trusted;

    pub use rml_proc::proof_assert;

    pub mod stubs {
        #[rustc_diagnostic_item = "rml_exists"]
        pub fn exists<T, F: Fn(T) -> bool>(_: F) -> bool {
            panic!()
        }

        #[rustc_diagnostic_item = "rml_forall"]
        pub fn forall<T, F: Fn(T) -> bool>(_: F) -> bool {
            panic!()
        }
    }
}

#[cfg(not(rml))]
mod macros {
    pub use rml_proc_dummy::spec;

    pub use rml_proc_dummy::pure;

    pub use rml_proc_dummy::strictly_pure;

    pub use rml_proc_dummy::invariant;

    pub use rml_proc_dummy::modifies;

    pub use rml_proc_dummy::variant;

    pub use rml_proc_dummy::logic;

    pub use rml_proc_dummy::rml;

    pub use rml_proc_dummy::trusted;

    pub use rml_proc_dummy::proof_assert;
}
