//! A hex dumper which calls a closure to allow the application to choose how
//! to output the dump.
//!
//! # Example: Dumping a struct
//! ```
//! use dbgtools_hexdump::{Config, hexdump};
//!
//! struct MyStruct {
//!   eight: u8,
//!   sixteen: u16,
//!   thirtytwo: u32
//! }
//!
//! let data = MyStruct { eight: 8, sixteen: 16, thirtytwo: 32 };
//! hexdump(Config::default(), &data, |offs, hex, ascii| {
//!   println!("{:08x} {} {}", offs, hex, ascii);
//! });
//! ```
//!
//! # Example: Dumping a struct with addresses
//! Sometimes it may be useful to include the real addresses in dumped buffers.
//! This can be accomplished by adding a base offset to the configuration
//! context.
//! ```
//! use dbgtools_hexdump::{Config, hexdump};
//!
//! struct MyStruct {
//!   eight: u8,
//!   sixteen: u16,
//!   thirtytwo: u32
//! }
//!
//! let data = MyStruct { eight: 8, sixteen: 16, thirtytwo: 32 };
//! hexdump(Config {
//!    offs: &data as *const _ as usize,
//!    ..Default::default()
//!   }, &data, |offs, hex, ascii| {
//!   println!("{:08x} {} {}", offs, hex, ascii);
//! });
//! ```

#![deny(missing_docs)]
#![deny(missing_crate_level_docs)]
#![deny(missing_doc_code_examples)]

use std::borrow::Borrow;

/// Return a `Sized` object as a byte slice.  😬
///
/// Warning: Reading uninitialized memory is UB.
pub fn asbuf<T: Sized>(buf: &T) -> &[u8] {
  // SAFETY: No.  :(
  unsafe {
    std::slice::from_raw_parts(
      buf as *const T as *const u8,
      std::mem::size_of::<T>()
    )
  }
}

/// Hex dumper configuration context.
pub struct Config {
  /// Number of columns in hex dump.  Defaults to 16.
  pub cols: usize,

  /// A base offset.  Defaults to 0.  If it's useful to display the addresses
  /// of a dumped buffer, this can be set to the initial address of the
  /// buffer.
  pub offs: usize
}

impl Default for Config {
  fn default() -> Self {
    Config { cols: 16, offs: 0 }
  }
}

/// Generate a hex dump of a `Sized` object and call a closure to process each
/// hex dump line.
///
/// ```
/// use dbgtools_hexdump::{Config, hexdump};
///
/// struct MyStruct {
///   eight: u8,
///   sixteen: u16,
///   thirtytwo: u32
/// }
///
/// let data = MyStruct { eight: 8, sixteen: 16, thirtytwo: 32 };
///
/// hexdump(Config::default(), &data, |offs, hex, ascii| {
///   println!("{:08x} {} {}", offs, hex, ascii);
/// });
/// ```
pub fn hexdump<C, T, F>(cfg: C, buf: &T, f: F)
where
  C: Borrow<Config>,
  T: Sized,
  F: Fn(usize, &str, &str)
{
  let buf = asbuf(buf);

  hexdump_buf(cfg, buf, f)
}


/// Generate a hex dump of a byte buffer (`&[u8]`) and call a closure to
/// process each hex dump line.
///
/// ```
/// use dbgtools_hexdump::{Config, hexdump_buf};
///
/// let data: &[u8] = &[1, 2, 3, 4];
///
/// hexdump_buf(Config::default(), &data, |offs, hex, ascii| {
///   println!("{:08x} {} {}", offs, hex, ascii);
/// });
/// ```
pub fn hexdump_buf<C, F>(cfg: C, buf: &[u8], f: F)
where
  C: Borrow<Config>,
  F: Fn(usize, &str, &str)
{
  let cfg = cfg.borrow();

  if cfg.cols == 0 {
    // derpy caller
    return;
  }

  let mut offset = cfg.offs;

  let mut ascii = String::new();

  for block in buf.chunks(cfg.cols) {
    let this_offs = offset;

    ascii.clear();

    let mut vals = Vec::new();
    for byte in block {
      vals.push(format!("{:02x}", byte));

      if *byte < 0x20 || *byte > 0x7e {
        ascii.push('.');
      } else {
        ascii.push(char::from(*byte));
      }

      offset += 1;
    }

    let rem = cfg.cols - vals.len();
    if rem > 0 {
      let rest_it = std::iter::repeat("  ".to_string()).take(rem);
      vals.extend(rest_it);

      let rest_ascii = String::from(" ").repeat(rem);
      ascii.push_str(&rest_ascii);
    }

    let hex_str = vals.join(" ");

    f(this_offs, &hex_str, &ascii);
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
