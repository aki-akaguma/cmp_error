use std::error::Error as StdError;
use thiserror::Error;

//pub type IntOk = u128;
//pub type IntOk = u64;
pub type IntOk = u32;
pub type IntErr = u32;

fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
#[inline(never)]
fn recursion_call<T>(n: u32, h: IntErr, f: impl FnOnce(IntErr) -> T) -> T {
    let h =  fxhash::hash64(&(h as u64 + n as u64)) as IntErr;
    if n == 0 {
        f(h)
    } else {
        black_box(recursion_call(n - 1, h, f))
    }
}

/*
use std::hint::black_box;
#[inline(never)]
fn recursion_call<T>(n: u32, f: impl FnOnce() -> T) -> T {
    let f = black_box(f);
    black_box(
    if n == 0 {
        f()
    } else {
        recursion_call(n - 1, f)
    }
    )
}
*/

#[inline(never)]
pub fn do_null_void(count: u32) -> Result<IntOk, ()> {
    black_box(recursion_call(count, 0, |h| Ok(h as IntOk)))
}

#[inline(never)]
pub fn do_plainerror(count: u32) -> Result<IntOk, MyError> {
    black_box(recursion_call(count, 0, |h| Err(MyError::new(h))))
}

#[inline(never)]
pub fn do_thiserror(count: u32) -> Result<IntOk, ThisError> {
    black_box(recursion_call(count, 0, |h| Err(ThisError::Mine(h))))
}

#[inline(never)]
pub fn do_std_error(count: u32) -> Result<IntOk, Box<dyn StdError>> {
    black_box(recursion_call(count, 0, |h| {
        let e: Box<dyn StdError> = Box::new(MyError::new(h));
        Err(e)
    }))
}

#[inline(never)]
pub fn do_anyhow(count: u32) -> Result<IntOk, anyhow::Error> {
    black_box(recursion_call(count, 0, |h| Err(MyError::new(h).into())))
}

#[inline(never)]
pub fn do_failure(count: u32) -> Result<IntOk, failure::Error> {
    black_box(recursion_call(count, 0, |h| Err(MyError::new(h).into())))
}

//----------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum ThisError {
    #[error("My Error: {0}")]
    Mine(IntErr),
}

#[derive(Debug)]
pub struct MyError(IntErr);
impl MyError {
    fn new(n: IntErr) -> Self {
        Self(n)
    }
}
impl StdError for MyError {}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "My Error: {}", self.0)
    }
}
