use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Wrapper de RwLock<T> para acceso controlado a través de closures.
#[derive(Clone)]
pub struct Rw<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Rw<T> {
    /// Crea un nuevo Rw<T>
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    /// Lectura: permite múltiples accesos concurrentes.
    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.inner.read().expect("RwLock poisoned");
        f(&*guard)
    }

    /// Escritura: acceso exclusivo.
    pub fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.write().expect("RwLock poisoned");
        f(&mut *guard)
    }

    /// Opción para obtener directamente el guard si lo necesitas.
    pub fn read_guard(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().expect("RwLock poisoned")
    }

    pub fn write_guard(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().expect("RwLock poisoned")
    }
}
