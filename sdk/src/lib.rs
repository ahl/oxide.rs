// Copyright 2023 Oxide Computer Company

#[allow(clippy::clone_on_copy)]
#[allow(clippy::len_zero)]
#[allow(clippy::uninlined_format_args)]
#[allow(clippy::vec_init_then_push)]
mod generated_sdk;

#[cfg(feature = "clap")]
mod clap_feature;

pub use generated_sdk::*;
