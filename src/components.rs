pub mod layouts {}

pub mod partials {}

pub mod atoms {
    mod label;
    pub use label::hello;
}

pub mod sections {
    mod counter;
    pub use counter::Counter;
}
