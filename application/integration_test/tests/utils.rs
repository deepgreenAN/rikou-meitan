use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub fn use_sync_mutex() -> MutexGuard<'static, ()> {
    MTX.lock().unwrap_or_else(|p_err| p_err.into_inner())
}
