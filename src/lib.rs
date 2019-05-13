#[allow(
    unused_imports,
    non_camel_case_types,
    clippy::redundant_closure_call,
    clippy::type_complexity,
    clippy::cognitive_complexity,
    clippy::large_enum_variant
)]
pub mod parse {
    include!(concat!(env!("OUT_DIR"), "/parse.rs"));
}
