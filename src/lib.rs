#![doc = include_str!("../README.md")]
//!
//! ## Limitations
//!
//! It is currently not possible to use [`CondType`] with a generic constant,
//! since Rust does not yet consider trait implementations based on booleans to
//! be exhaustive (see [issue](https://github.com/rust-lang/project-const-generics/issues/26)).
//!
//! ```rust,ignore
//! # use condtype::CondType;
//! fn generic<const B: bool>() {
//!     let x: CondType<B, i32, u8> = ...
//! }
//! ```

#![no_std]
#![warn(missing_docs)]

/// A type alias determined by a boolean condition.
///
/// This is equivalent to [`std::conditional_t` in C++](https://en.cppreference.com/w/cpp/types/conditional).
///
/// # Examples
///
/// ```
/// use condtype::CondType;
///
/// let str: CondType<true,  &str, i32> = "hello";
/// let int: CondType<false, &str, i32> = 42;
/// ```
///
/// This can also be used with <code>\![Sized]</code> types:
///
/// ```
/// # use condtype::CondType;
/// let str: &CondType<true, str, [u8]> = "world";
/// ```
pub type CondType<const B: bool, T, F> = <imp::CondType::<B, T, F> as imp::AssocType>::Type;

mod imp {
    use core::marker::PhantomData;

    pub struct CondType<const B: bool, T: ?Sized, F: ?Sized>(PhantomData<F>, PhantomData<T>);

    pub trait AssocType {
        type Type: ?Sized;
    }

    impl<T: ?Sized, F: ?Sized> AssocType for CondType<false, T, F> {
        type Type = F;
    }

    impl<T: ?Sized, F: ?Sized> AssocType for CondType<true, T, F> {
        type Type = T;
    }
}
