//! Defines several scope functions which take `Self` as an argument and return a value.

pub trait Run: Sized {
    /// Calls the specified `closure` with `Self` as an argument and returns its result.
    fn run<T>(&self, closure: impl FnOnce(&Self) -> T) -> T;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns its result.
    fn run_mut<T>(&mut self, closure: impl FnOnce(&mut Self) -> T) -> T;

    /// Calls the specified `closure` with `Self` as an argument and returns a `Result`.
    fn try_run<T, E>(&self, closure: impl FnOnce(&Self) -> Result<T, E>) -> Result<T, E>;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns a `Result`.
    fn try_run_mut<T, E>(&mut self, closure: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<T, E>;
}

impl<A: Sized> Run for A {
    #[inline(always)]
    fn run<T>(&self, closure: impl FnOnce(&Self) -> T) -> T {
        closure(self)
    }

    #[inline(always)]
    fn run_mut<T>(&mut self, closure: impl FnOnce(&mut Self) -> T) -> T {
        closure(self)
    }

    #[inline(always)]
    fn try_run<T, E>(&self, closure: impl FnOnce(&Self) -> Result<T, E>) -> Result<T, E> {
        closure(self)
    }

    #[inline(always)]
    fn try_run_mut<T, E>(&mut self, closure: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<T, E> {
        closure(self)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::SubAssign;

    use super::*;

    #[test]
    fn run_works() {
        let x = 21;
        let y = x.run(|x| x + 21);

        assert_eq!(y, 42);
    }

    #[test]
    fn run_mut_works() {
        let x = &mut 28;
        let y = 14.run_mut(|_| {
            x.sub_assign(14);
            28
        });
        let z = *x + y;

        assert_eq!(z, 42);
    }

    #[test]
    fn try_run_works() {
        let x: Result<i32, ()> = 42.try_run(|x| Ok(*x));
        assert_eq!(x, Ok(42));

        let y: Result<(), &str> = 0.try_run(|_| Err("error"));
        assert_eq!(y, Err("error"));
    }

    #[test]
    fn try_run_mut_works() {
        let x: Result<i32, ()> = 42.try_run_mut(|x| Ok(*x));
        assert_eq!(x, Ok(42));

        let y: Result<(), &str> = 0.try_run_mut(|_| Err("error"));
        assert_eq!(y, Err("error"));
    }
}
