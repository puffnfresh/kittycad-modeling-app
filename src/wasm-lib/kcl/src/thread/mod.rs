//! An implementation of threads that works in wasm with promises and other platforms with tokio.

#[cfg(not(target_arch = "wasm32"))]
pub mod local;
#[cfg(not(target_arch = "wasm32"))]
pub use local::JoinHandle;

#[cfg(target_arch = "wasm32")]
#[cfg(not(test))]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
#[cfg(not(test))]
pub use wasm::JoinHandle;

pub trait Thread {
    /// Abort a thread.
    fn abort(&self);

    /// Check if a thread is finished.
    fn is_finished(&self) -> bool;
}