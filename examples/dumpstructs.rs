use dbgtools_hexdump::*;

struct MyStruct {
  eight: u8,
  sixteen: u16,
  thirtytwo: u32
}

#[repr(C, packed(1))]
struct MyStructC {
  eight: u8,
  sixteen: u16,
  thirtytwo: u32
}


fn main() {
  let data = MyStruct {
    eight: 8,
    sixteen: 16,
    thirtytwo: 32
  };
  println!("MyStruct {{ eight: u8, sixteen: u16, thirtytwo: u32 }}");
  println!("data = MyStruct {{ eight: 8, sixteen: 16, thirtytwo: 32 }}");
  hexdump(Config::default(), &data, |offs, hex, ascii| {
    println!("{:08x}  {}  |{}|", offs, hex, ascii);
  });

  println!("\nSame, but show the struct's addresses");
  hexdump(
    Config {
      offs: &data as *const _ as usize,
      ..Default::default()
    },
    &data,
    |offs, hex, ascii| {
      println!("{:016x}  {}  |{}|", offs, hex, ascii);
    }
  );


  println!("\nSame, but repr C and packed(1)");

  let data = MyStructC {
    eight: 8,
    sixteen: 16,
    thirtytwo: 32
  };
  println!("#[repr(C, packed(1))]");
  println!("MyStructC {{ eight: u8, sixteen: u16, thirtytwo: u32 }}");
  println!("data = MyStructC {{ eight: 8, sixteen: 16, thirtytwo: 32 }}");
  hexdump(Config::default(), &data, |offs, hex, ascii| {
    println!("{:08x}  {}  |{}|", offs, hex, ascii);
  });
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
