#[allow(
    non_camel_case_types,
    unused_imports,
    clippy::cognitive_complexity,
    clippy::large_enum_variant,
    clippy::needless_lifetimes,
    clippy::redundant_closure_call,
    clippy::type_complexity
)]
pub mod parse {
    include!(concat!(env!("OUT_DIR"), "/parse.rs"));
}
