pub use de::Deserializer;

#[allow(clippy::module_inception)]
pub mod de;
pub(crate) mod parser;
pub(crate) mod composer;
pub(crate) mod constructor;
