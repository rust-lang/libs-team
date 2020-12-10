# Unstable API Dump

This tool will dump the public API for an unstable feature.

## Usage

From this directory, run something like:

```shell
cargo run --release -- --feature $feature --repo-root $path_to_rust | rustfmt
```

where `$feature` is the name of the unstable feature and `$path_to_rust` is the path to a local clone of `rust-lang/rust`. You'll probably need to run `x.py` first to make sure submodules are cloned.

You can also install it as a Cargo tool and use it that way:

```shell
cargo install --path .
cargo unstable-api --feature $feature --repo-root $path_to_rust | rustfmt
```

It'll output something like:

```rust
// mod core
pub mod lazy {}

// mod core::lazy
pub struct OnceCell<T> {}

// mod core::lazy
impl<T> Default for OnceCell<T> {
    fn default() -> Self {}
}

// mod core::lazy
impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {}
}

// mod core::lazy
impl<T: Clone> Clone for OnceCell<T> {
    fn clone(&self) -> OnceCell<T> {}
}

// mod core::lazy
impl<T: PartialEq> PartialEq for OnceCell<T> {
    fn eq(&self, other: &Self) -> bool {}
}

// mod core::lazy
impl<T: Eq> Eq for OnceCell<T> {}

// mod core::lazy
impl<T> From<T> for OnceCell<T> {
    fn from(value: T) -> Self {}
}

// mod core::lazy
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {}
    pub fn get(&self) -> Option<&T> {}
    pub fn get_mut(&mut self) -> Option<&mut T> {}
    pub fn set(&self, value: T) -> Result<(), T> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
    }
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
    }
    pub fn into_inner(self) -> Option<T> {}
    pub fn take(&mut self) -> Option<T> {}
}

// mod core::lazy
pub struct Lazy<T, F = fn() -> T> {}

// mod core::lazy
impl<T: fmt::Debug, F> fmt::Debug for Lazy<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {}
}

// mod core::lazy
impl<T, F> Lazy<T, F> {
    pub const fn new(init: F) -> Lazy<T, F> {}
}

// mod core::lazy
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn force(this: &Lazy<T, F>) -> &T {}
}

// mod core::lazy
impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {}
}

// mod core::lazy
impl<T: Default> Default for Lazy<T> {
    fn default() -> Lazy<T> {}
}

// mod std
pub mod lazy {}

// mod std::lazy
pub use core::lazy::*;

// mod std::lazy
pub struct SyncOnceCell<T> {}

// mod std::lazy
unsafe impl<T: Sync + Send> Sync for SyncOnceCell<T> {}

// mod std::lazy
unsafe impl<T: Send> Send for SyncOnceCell<T> {}

// mod std::lazy
impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for SyncOnceCell<T> {}

// mod std::lazy
impl<T: UnwindSafe> UnwindSafe for SyncOnceCell<T> {}

// mod std::lazy
impl<T> Default for SyncOnceCell<T> {
    fn default() -> SyncOnceCell<T> {}
}

// mod std::lazy
impl<T: fmt::Debug> fmt::Debug for SyncOnceCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {}
}

// mod std::lazy
impl<T: Clone> Clone for SyncOnceCell<T> {
    fn clone(&self) -> SyncOnceCell<T> {}
}

// mod std::lazy
impl<T> From<T> for SyncOnceCell<T> {
    fn from(value: T) -> Self {}
}

// mod std::lazy
impl<T: PartialEq> PartialEq for SyncOnceCell<T> {
    fn eq(&self, other: &SyncOnceCell<T>) -> bool {}
}

// mod std::lazy
impl<T: Eq> Eq for SyncOnceCell<T> {}

// mod std::lazy
impl<T> SyncOnceCell<T> {
    pub const fn new() -> SyncOnceCell<T> {}
    pub fn get(&self) -> Option<&T> {}
    pub fn get_mut(&mut self) -> Option<&mut T> {}
    pub fn set(&self, value: T) -> Result<(), T> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
    }
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
    }
    pub fn into_inner(mut self) -> Option<T> {}
    pub fn take(&mut self) -> Option<T> {}
}

// mod std::lazy
pub struct SyncLazy<T, F = fn() -> T> {}

// mod std::lazy
impl<T: fmt::Debug, F> fmt::Debug for SyncLazy<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {}
}

// mod std::lazy
unsafe impl<T, F: Send> Sync for SyncLazy<T, F> where SyncOnceCell<T>: Sync {}

// mod std::lazy
impl<T, F: UnwindSafe> RefUnwindSafe for SyncLazy<T, F> where SyncOnceCell<T>: RefUnwindSafe {}

// mod std::lazy
impl<T, F: UnwindSafe> UnwindSafe for SyncLazy<T, F> where SyncOnceCell<T>: UnwindSafe {}

// mod std::lazy
impl<T, F> SyncLazy<T, F> {
    pub const fn new(f: F) -> SyncLazy<T, F> {}
}

// mod std::lazy
impl<T, F: FnOnce() -> T> SyncLazy<T, F> {
    pub fn force(this: &SyncLazy<T, F>) -> &T {}
}

// mod std::lazy
impl<T, F: FnOnce() -> T> Deref for SyncLazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {}
}

// mod std::lazy
impl<T: Default> Default for SyncLazy<T> {
    fn default() -> SyncLazy<T> {}
}
```
