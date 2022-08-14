use super::utilities;
use core::ops::DerefMut;
use once_cell::sync::Lazy;
use std::error::Error;
use tokio::{
    io::{stderr, stdin, stdout, Stderr, Stdin, Stdout},
    sync::Mutex,
};

static STDIN: Lazy<Mutex<Stdin>> = Lazy::new(|| Mutex::new(stdin()));
static STDOUT: Lazy<Mutex<Stdout>> = Lazy::new(|| Mutex::new(stdout()));
static STDERR: Lazy<Mutex<Stderr>> = Lazy::new(|| Mutex::new(stderr()));

#[ffi::bindgen]
async fn _pen_os_read_stdin() -> Result<ffi::ByteString, Box<dyn Error>> {
    utilities::read(&mut stdin()).await
}

#[ffi::bindgen]
async fn _pen_os_read_limit_stdin(limit: ffi::Number) -> Result<ffi::ByteString, Box<dyn Error>> {
    utilities::read_limit(STDIN.lock().await.deref_mut(), f64::from(limit) as usize).await
}

#[ffi::bindgen]
async fn _pen_os_write_stdout(bytes: ffi::ByteString) -> Result<ffi::Number, Box<dyn Error>> {
    utilities::write(STDOUT.lock().await.deref_mut(), bytes).await
}

#[ffi::bindgen]
async fn _pen_os_write_stderr(bytes: ffi::ByteString) -> Result<ffi::Number, Box<dyn Error>> {
    utilities::write(STDERR.lock().await.deref_mut(), bytes).await
}
