#[cfg(kodex_bazel)]
pub use code_mode_proto::kodex::code_mode::v1::*;

#[cfg(not(kodex_bazel))]
tonic::include_proto!("kodex.code_mode.v1");

pub const MAX_IDENTIFIER_BYTES: usize = 256;
