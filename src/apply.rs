//! Defines several scope functions that perform additional actions on `Self`.

pub trait Apply: Sized {
    /// Calls the specified `closure` with `Self` as an argument and returns `Self`.
    fn apply(&self, closure: impl FnOnce(&Self) -> ()) -> &Self;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns `Self`.
    fn apply_mut(&mut self, closure: impl FnOnce(&mut Self) -> ()) -> &mut Self;

    /// Calls the specified `closure` with `Self` as an argument and returns a `Result`.
    fn try_apply<E>(&self, closure: impl FnOnce(&Self) -> Result<(), E>) -> Result<&Self, E>;

    /// Calls the specified `closure` with mutable `Self` as an argument and returns a `Result`.
    fn try_apply_mut<E>(&mut self, closure: impl FnOnce(&mut Self) -> Result<(), E>) -> Result<&mut Self, E>;
}

impl<A: Sized> Apply for A {
    #[inline(always)]
    fn apply(&self, closure: impl FnOnce(&Self) -> ()) -> &Self {
        closure(self);
        self
    }

    #[inline(always)]
    fn apply_mut(&mut self, closure: impl FnOnce(&mut Self) -> ()) -> &mut Self {
        closure(self);
        self
    }

    #[inline(always)]
    fn try_apply<E>(&self, closure: impl FnOnce(&Self) -> Result<(), E>) -> Result<&Self, E> {
        closure(&self)?;
        Ok(self)
    }

    #[inline(always)]
    fn try_apply_mut<E>(&mut self, closure: impl FnOnce(&mut Self) -> Result<(), E>) -> Result<&mut Self, E> {
        closure(self)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_works() {
        let mut x = 14;
        let y = 14.apply(|_| x += 14);

        assert_eq!(x + y, 42);
    }

    #[test]
    fn apply_mut_works() {
        let mut binding = String::from("one two");
        let x = binding.apply_mut(|x| x.truncate(3));

        assert_eq!(x, "one");
    }

    #[test]
    fn try_apply_works() {
        let x: Result<&i32, ()> = 42.try_apply(|_| Ok(()));
        assert_eq!(x, Ok(&42));

        let y = 0.try_apply(|_| Err("error"));
        assert_eq!(y, Err("error"));
    }

    #[test]
    fn try_apply_mut_works() {
        let mut binding = String::from("one two");

        let x: Result<&mut String, ()> = binding.try_apply_mut(|x| {
            x.truncate(3);

            Ok(())
        });
        assert_eq!(x, Ok(&mut String::from("one")));

        let y = binding.try_apply_mut(|_| Err("error"));
        assert_eq!(y, Err("error"));
    }
}
