fn print(s: &str) {} // does not need explicit lifetime
fn print<'a>(s: &'a str) {} // expands code by compiler

fn debug(v: usize, s: &str) {} // does not need explicit lifetime
fn debug<'a>(v: usize, s: &'a str) {} // expands code by compiler

fn substr(s: &str, until: usize) -> &str {} // does not need explicit lifetime
fn substr<'a>(s: &'a str, until: usize) -> &'a str; // expands code by compiler

fn get_str() -> &str {} // needs explicit lifetimes

fn frob(s: &str, t: &str) -> &str {} // needs explicit lifetimes

fn get_mut(&mut self) -> &mut T; // does not need explicit lifetime
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expands code by compiler

fn new(buf: &mut [u8]) -> BufWriter; // does not need explicit lifetime
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded 
