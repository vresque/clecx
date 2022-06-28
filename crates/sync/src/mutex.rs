use core::{
    cell::UnsafeCell,
    fmt::{self, write},
    hint::spin_loop,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

/// # Mutex<T>
/// [A ticket based mutex](https://en.wikipedia.org/wiki/Ticket_lock)
/// Spins if the data is not available
pub struct Mutex<T> {
    data: UnsafeCell<T>,
    next_ticket: AtomicUsize,
    next_serve: AtomicUsize,
}

unsafe impl<T> Send for Mutex<T> {}
unsafe impl<T> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        Self {
            data: UnsafeCell::new(val),
            next_ticket: AtomicUsize::new(0),
            next_serve: AtomicUsize::new(0),
        }
    }

    pub fn lock<'data_lt>(&'data_lt self) -> MutexGuard<'data_lt, T> {
        let ticket = self.next_ticket.fetch_add(1, Ordering::Acquire);

        while self.current_serve() != ticket {
            spin_loop();
        }

        MutexGuard {
            data: unsafe { &mut *self.data.get() },
            my_ticket: ticket,
            next_serve: &self.next_serve,
        }
    }

    /// # Try Lock
    /// This method is not entirely safe, yet it should work in most use cases
    /// If it fails, it will wait until the lock is free
    pub fn try_lock<'data_lt>(&'data_lt self) -> Option<MutexGuard<'data_lt, T>> {
        if self.is_locked() {
            None
        } else {
            Some(self.lock())
        }
    }

    pub fn is_locked(&self) -> bool {
        !self.is_unlocked()
    }

    pub fn is_unlocked(&self) -> bool {
        self.current_serve() == self.current_ticket()
    }

    pub fn current_ticket(&self) -> usize {
        self.next_ticket.load(Ordering::Acquire)
    }

    pub fn current_serve(&self) -> usize {
        self.next_serve.load(Ordering::Relaxed)
    }

    pub unsafe fn force_unlock(&self) {
        if self.is_locked() {
            self.next_serve.fetch_add(1, Ordering::Release);
        }
    }
}

impl<T: core::fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mutex {{")?;
        if let Some(s) = self.try_lock() {
            writeln!(f, "{:?}", *s)?;
        } else {
            writeln!(f, "<<<Locked>>>")?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

pub struct MutexGuard<'data_lt, T> {
    data: &'data_lt mut T,
    my_ticket: usize,
    next_serve: &'data_lt AtomicUsize,
}

impl<'data_lt, T> Deref for MutexGuard<'data_lt, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'data_lt, T> DerefMut for MutexGuard<'data_lt, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'data_lt, T> Drop for MutexGuard<'data_lt, T> {
    fn drop(&mut self) {
        // SAFETY: we cannot just increment here
        self.next_serve.store(self.my_ticket + 1, Ordering::Release);
    }
}
