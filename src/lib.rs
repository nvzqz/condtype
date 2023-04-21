//! [`CondType`]: CondType
#![doc = include_str!("../README.md")]
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
pub type CondType<const B: bool, T, F> = <imp::CondType<B, T, F> as imp::AssocType>::Type;

/// Public-in-private implementation details for `CondType`.
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

/// Instantiates a [conditionally-typed](CondType) value.
///
/// # Examples
///
/// Given a [`const`] [`bool`], the following code will construct either a
/// [`&str`](str) or [`i32`]:
///
/// ```
/// use condtype::condval;
///
/// const COND: bool = // ...
/// # true;
///
/// let str = "hello";
/// let int = 42;
///
/// let val = condval!(COND, str, int);
/// ```
///
/// This macro can also construct [`const`] values:
///
/// ```
/// use condtype::{condval, CondType};
///
/// const COND: bool = // ...
/// # true;
///
/// const VAL: CondType<COND, &str, i32> = condval!(COND, "hello", 42);
/// ```
///
/// Arguments are lazily evaluated, so there are no effects from unvisited
/// branches:
///
/// ```
/// # use condtype::*;
/// let x;
///
/// let val = condval!(
///     true,
///     { x = 10; "hello" },
///     { x = 50; 42 },
/// );
///
/// assert_eq!(x, 10);
/// assert_eq!(val, "hello");
/// ```
///
/// Assigning an incorrect type will cause a compile failure:
///
/// ```compile_fail
/// # use condtype::*;
/// let val: bool = condval!(true, "hello", 42);
/// ```
///
/// Attempting to reuse a non-[`Copy`] value from either branch will cause a
/// compile failure, because it has been moved into that branch and can thus no
/// longer be used in the outer context:
///
/// ```compile_fail
/// # use condtype::*;
/// let int = 42;
/// let vec = vec![1, 2, 3];
///
/// let val = condval!(true, int, vec);
/// println!("{:?}", vec);
/// ```
///
/// [`const`]: https://doc.rust-lang.org/std/keyword.const.html
#[macro_export]
macro_rules! condval {
    ($cond:expr, $t:expr, $f:expr $(,)?) => {
        match <() as $crate::__private::If<$cond, _, _>>::PROOF {
            $crate::__private::EitherTypeEq::Left(te) => te.coerce($t),
            $crate::__private::EitherTypeEq::Right(te) => te.coerce($f),
        }
    };
}

/// Pseudo-public implementation details for `condval!`.
#[doc(hidden)]
pub mod __private {
    use crate::TypeEq;

    pub enum EitherTypeEq<L, R, C> {
        Left(TypeEq<L, C>),
        Right(TypeEq<R, C>),
    }

    pub trait If<const B: bool, T, F> {
        type Chosen;
        const PROOF: EitherTypeEq<T, F, Self::Chosen>;
    }

    impl<T, F> If<true, T, F> for () {
        type Chosen = T;
        const PROOF: EitherTypeEq<T, F, Self::Chosen> = EitherTypeEq::Left(TypeEq::NEW);
    }

    impl<T, F> If<false, T, F> for () {
        type Chosen = F;
        const PROOF: EitherTypeEq<T, F, Self::Chosen> = EitherTypeEq::Right(TypeEq::NEW);
    }
}

use crate::type_eq::TypeEq;
mod type_eq {
    use core::marker::PhantomData;

    #[allow(clippy::type_complexity)]
    pub struct TypeEq<T: ?Sized, U: ?Sized>(
        PhantomData<(
            fn(PhantomData<T>) -> PhantomData<T>,
            fn(PhantomData<U>) -> PhantomData<U>,
        )>,
    );

    impl<T: ?Sized> TypeEq<T, T> {
        pub const NEW: Self = TypeEq(PhantomData);
    }

    impl<T, U> TypeEq<T, U> {
        pub const fn coerce(self, from: T) -> U {
            use core::mem::ManuallyDrop;

            #[repr(C)]
            union Transmuter<From, To> {
                from: ManuallyDrop<From>,
                to: ManuallyDrop<To>,
            }

            unsafe {
                ManuallyDrop::into_inner(
                    Transmuter {
                        from: ManuallyDrop::new(from),
                    }
                    .to,
                )
            }
        }
    }
}
