use std::sync::{Arc, Mutex, MutexGuard};

/// Wrapper de Mutex<T> para acceso controlado a través de closures.
pub struct Mtx<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Mtx<T> {
    /// Crea un nuevo Mtx<T>
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    /// Acceso exclusivo: ejecuta un closure con una referencia inmutable.
    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.inner.lock().expect("Mutex poisoned");
        f(&*guard)
    }

    /// Acceso exclusivo: ejecuta un closure con una referencia mutable.
    pub fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        f(&mut *guard)
    }

    /// Obtiene directamente el guard del mutex.
    pub fn guard(&self) -> MutexGuard<'_, T> {
        self.inner.lock().expect("Mutex poisoned")
    }
}
impl<T> Clone for Mtx<T> {
    fn clone(&self) -> Self {
        Mtx {
            inner: Arc::clone(&self.inner),
        }
    }
}
