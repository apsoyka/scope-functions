//! This crate provides Kotlin-inspired scope functions for use in almost any situation.

/// Defines several scope functions that perform additional actions on `Self`.
pub trait Apply: Sized {
    /// Calls the specified `closure` with `Self` as an argument and returns `Self`.
    fn apply(self, closure: fn(&Self) -> ()) -> Self;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns `Self`.
    fn apply_mut(self, closure: fn(&mut Self) -> ()) -> Self;

    /// Calls the specified `closure` with `Self` as an argument and returns a `Result`.
    fn try_apply<E>(self, closure: fn(&Self) -> Result<(), E>) -> Result<Self, E>;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns a `Result`.
    fn try_apply_mut<E>(self, closure: fn(&mut Self) -> Result<(), E>) -> Result<Self, E>;
}

/// Defines several scope functions which take `Self` as an argument and return a value.
pub trait Run: Sized {
    /// Calls the specified `closure` with `Self` as an argument and returns its result.
    fn run<T>(self, closure: fn(&Self) -> T) -> T;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns its result.
    fn run_mut<T>(self, closure: fn(&mut Self) -> T) -> T;

    /// Calls the specified `closure` with `Self` as an argument and returns a `Result`.
    fn try_run<T, E>(self, closure: fn(&Self) -> Result<T, E>) -> Result<T, E>;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns a `Result`.
    fn try_run_mut<T, E>(self, closure: fn(&mut Self) -> Result<T, E>) -> Result<T, E>;
}

impl<A: Sized> Apply for A {
    #[inline(always)]
    fn apply(self, closure: fn(&Self) -> ()) -> A {
        closure(&self);
        self
    }

    #[inline(always)]
    fn apply_mut(mut self, closure: fn(&mut Self) -> ()) -> A {
        closure(&mut self);
        self
    }

    #[inline(always)]
    fn try_apply<E>(self, closure: fn(&Self) -> Result<(), E>) -> Result<Self, E> {
        closure(&self)?;
        Ok(self)
    }

    #[inline(always)]
    fn try_apply_mut<E>(mut self, closure: fn(&mut Self) -> Result<(), E>) -> Result<Self, E> {
        closure(&mut self)?;
        Ok(self)
    }
}

impl<A: Sized> Run for A {
    #[inline(always)]
    fn run<T>(self, closure: fn(&Self) -> T) -> T {
        closure(&self)
    }

    #[inline(always)]
    fn run_mut<T>(mut self, closure: fn(&mut Self) -> T) -> T {
        closure(&mut self)
    }

    #[inline(always)]
    fn try_run<T, E>(self, closure: fn(&Self) -> Result<T, E>) -> Result<T, E> {
        closure(&self)
    }

    #[inline(always)]
    fn try_run_mut<T, E>(mut self, closure: fn(&mut Self) -> Result<T, E>) -> Result<T, E> {
        closure(&mut self)
    }
}

/// Calls the specified `closure` and returns its result.
#[inline(always)]
pub fn run<T>(closure: fn() -> T) -> T {
    closure()
}

/// Calls the specified `closure` and returns a `Result`.
#[inline(always)]
pub fn try_run<T, E>(closure: fn() -> Result<T, E>) -> Result<T, E> {
    closure()
}

/// Calls the specified `closure` with the given `receiver` and returns its result.
#[inline(always)]
pub fn with<R, T>(receiver: R, closure: fn(R) -> T) -> T {
    closure(receiver)
}

/// Calls the specified `closure` with the given mutable `receiver` and returns its result.
#[inline(always)]
pub fn with_mut<R, T>(mut receiver: R, closure: fn(&mut R) -> T) -> T {
    closure(&mut receiver)
}

/// Calls the specified `closure` with the given `receiver` and returns a `Result`.
#[inline(always)]
pub fn try_with<R, T, E>(receiver: R, closure: fn(R) -> Result<T, E>) -> Result<T, E> {
    closure(receiver)
}

/// Calls the specified `closure` with the given mutable `receiver` and returns a `Result`.
#[inline(always)]
pub fn try_with_mut<R, T, E>(mut receiver: R, closure: fn(&mut R) -> Result<T, E>) -> Result<T, E> {
    closure(&mut receiver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_works() {
        let x = run(|| 42);

        assert_eq!(x, 42);
    }

    #[test]
    fn run_with_self_works() {
        let x = 21;
        let y = x.run(|x| x + 21);

        assert_eq!(y, 42);
    }

    #[test]
    fn try_run_works() {
        let x: Result<i32, ()> = run(|| Ok(42));
        assert_eq!(x, Ok(42));

        let y: Result<(), &str> = run(|| Err("error"));
        assert_eq!(y, Err("error"));
    }
}
