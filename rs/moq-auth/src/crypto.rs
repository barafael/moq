#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod graviola;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub(super) use graviola::*;

#[cfg(any(test, not(any(target_arch = "x86_64", target_arch = "aarch64"))))]
mod aws;
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub(super) use aws::*;
