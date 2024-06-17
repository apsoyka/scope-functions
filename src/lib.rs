//! This crate provides Kotlin-inspired scope functions for use in almost any situation.

mod apply;
mod run;

pub use apply::Apply;
pub use run::Run;

/// Calls the specified `closure` and returns its result.
#[inline(always)]
pub fn run<T>(closure: impl FnOnce() -> T) -> T {
    closure()
}

/// Calls the specified `closure` and returns a `Result`.
#[inline(always)]
pub fn try_run<T, E>(closure: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
    closure()
}

/// Calls the specified `closure` with the given `receiver` and returns its result.
#[inline(always)]
pub fn with<R, T>(receiver: &R, closure: impl FnOnce(&R) -> T) -> T {
    closure(receiver)
}

/// Calls the specified `closure` with the given mutable `receiver` and returns its result.
#[inline(always)]
pub fn with_mut<R, T>(receiver: &mut R, closure: impl FnOnce(&mut R) -> T) -> T {
    closure(receiver)
}

/// Calls the specified `closure` with the given `receiver` and returns a `Result`.
#[inline(always)]
pub fn try_with<R, T, E>(receiver: &R, closure: impl FnOnce(&R) -> Result<T, E>) -> Result<T, E> {
    closure(receiver)
}

/// Calls the specified `closure` with the given mutable `receiver` and returns a `Result`.
#[inline(always)]
pub fn try_with_mut<R, T, E>(receiver: &mut R, closure: impl FnOnce(&mut R) -> Result<T, E>) -> Result<T, E> {
    closure(receiver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_works() {
        let x = 21;
        let y = run(|| x + 21);

        assert_eq!(y, 42);
    }

    #[test]
    fn try_run_works() {
        let x: Result<i32, ()> = try_run(|| Ok(42));
        assert_eq!(x, Ok(42));

        let y: Result<(), &str> = try_run(|| Err("error"));
        assert_eq!(y, Err("error"));
    }

    #[test]
    fn with_works() {
        let x = with(&14, |x| x + 28);

        assert_eq!(x, 42);
    }

    #[test]
    fn with_mut_works() {
        let x = with_mut(&mut 28, |x| {
            *x += 14;
            *x
        });

        assert_eq!(x, 42);
    }

    #[test]
    fn try_with_works() {
        let x: Result<i32, ()> = try_with(&42, |x| Ok(*x));
        assert_eq!(x, Ok(42));

        let y: Result<(), &str> = try_with(&0, |_| Err("error"));
        assert_eq!(y, Err("error"));
    }

    #[test]
    fn try_with_mut_works() {
        let x: Result<i32, ()> = try_with_mut(&mut 28, |x| {
            *x += 14;
            Ok(*x)
        });
        assert_eq!(x, Ok(42));

        let y: Result<(), &str> = try_with_mut(&mut 0, |_| Err("error"));
        assert_eq!(y, Err("error"));
    }
}
