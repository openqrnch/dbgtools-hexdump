use dbgtools_hexdump::*;

fn main() {
  let data = &["hello", "world"];

  hexdump(Config::default(), data, |offs, hex, ascii| {
    println!("{:08x}  {}  {}", offs, hex, ascii);
  });


  let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];

  hexdump_buf(Config::default(), data, |offs, hex, ascii| {
    println!("{:08x}  {}  {}", offs, hex, ascii);
  });
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
