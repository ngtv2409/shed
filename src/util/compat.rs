#[cfg(target_os = "android")]
/// extend nix libc to be compatible on Android
pub mod libc {
  #[allow(unused_imports)]
  use nix::libc::*;
  pub const _CS_PATH: i32 = 1;
  pub unsafe fn confstr(_name: i32, _buf: *mut u8, _len: usize) -> usize {
    0
  }
}
