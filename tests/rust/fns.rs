#[repr(C)]
pub struct Fns {
  noArgs: fn(),
  anonymousArg: fn(i32),
  returnsNumber: fn() -> i32,
  namedArgs: fn(first: i32, snd: i16) -> i8,
}

#[no_mangle]
pub extern "C" fn root(_fns: Fns) {}
