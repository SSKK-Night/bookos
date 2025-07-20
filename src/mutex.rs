pub struct Mutex<T> {
    locked: AtomicBool,
    value: T,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value,
        }
    }

    pub fn lock(&self) {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {

        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}