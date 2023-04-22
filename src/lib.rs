//! [`CondType`]: CondType
//! [`condval!`]: condval
#![doc = include_str!("../README.md")]
#![cfg_attr(not(doc), no_std)]
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

/// Instantiates a [conditionally-typed](CondType) value.
///
/// Attempting to return different types from [`if`]/[`else`] is not normally
/// possible since both branches must produce the same type:
///
/// ```compile_fail
/// let val = if true { "hello" } else { 42 };
/// ```
///
/// This macro enables returning different types by making the type be
/// conditional on a [`const`] [`bool`]:
///
/// ```
/// use condtype::condval;
///
/// let val: &str = condval!(if true { "hello" } else { 42 });
/// let val: i32 = condval!(if false { "hello" } else { 42 });
/// ```
///
/// # Examples
///
/// Given two conditions, the following code will construct either a
/// [`&str`](str), [`i32`], or [`Vec`]:
///
/// ```
/// # use condtype::condval;
/// const COND1: bool = // ...
/// # true;
/// const COND2: bool = // ...
/// # true;
///
/// let str = "hello";
/// let int = 42;
/// let vec = vec![1, 2, 3];
///
/// let val = condval!(if COND1 {
///     str
/// } else if COND2 {
///     int
/// } else {
///     vec
/// });
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
/// const VAL: CondType<COND, &str, i32> = condval!(if COND {
///     "hello"
/// } else {
///     42
/// });
/// ```
///
/// Arguments are lazily evaluated, so there are no effects from unvisited
/// branches:
///
/// ```
/// # use condtype::*;
/// let x;
///
/// let val = condval!(if true {
///     x = 10;
///     "hello"
/// } else {
///     x = 50;
///     42
/// });
///
/// assert_eq!(x, 10);
/// assert_eq!(val, "hello");
/// ```
///
/// Assigning an incorrect type will cause a compile failure:
///
/// ```compile_fail
/// # use condtype::*;
/// let val: bool = condval!(if true {
///     "hello"
/// } else {
///     42
/// });
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
/// let val = condval!(if true {
///     int
/// } else {
///     vec
/// });
///
/// println!("{:?}", vec);
/// ```
///
/// [`const`]: https://doc.rust-lang.org/std/keyword.const.html
/// [`else`]:  https://doc.rust-lang.org/std/keyword.else.html
/// [`if`]:    https://doc.rust-lang.org/std/keyword.if.html
#[macro_export]
macro_rules! condval {
    (if $cond:block   $t:block else $f:block) => {
        match <() as $crate::__private::If<$cond, _, _>>::PROOF {
            $crate::__private::EitherTypeEq::Left(te) => te.coerce($t),
            $crate::__private::EitherTypeEq::Right(te) => te.coerce($f),
        }
    };
    (if $cond:block   $t:block else $($else:tt)+) => {
        $crate::condval!(if $cond $t else { $crate::condval!($($else)+) })
    };
    (if $cond:literal $t:block else $($else:tt)+) => {
        $crate::condval!(if { $cond } $t else $($else)+)
    };
    (if $cond:ident   $t:block else $($else:tt)+) => {
        $crate::condval!(if { $cond } $t else $($else)+)
    };
    (if $cond:path    $t:block else $($else:tt)+) => {
        $crate::condval!(if { $cond } $t else $($else)+)
    };
    (if ($cond:expr)  $t:block else $($else:tt)+) => {
        $crate::condval!(if { $cond } $t else $($else)+)
    };
}

/// Pseudo-public implementation details for `condval!`.
#[doc(hidden)]
pub mod __private {
    use crate::imp::TypeEq;

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

/// Public-in-private implementation details.
mod imp {
    use core::{marker::PhantomData, mem::ManuallyDrop};

    pub struct CondType<const B: bool, T: ?Sized, F: ?Sized>(
        // `CondType` is covariant over `T` and `F`.
        PhantomData<F>,
        PhantomData<T>,
    );

    pub trait AssocType {
        type Type: ?Sized;
    }

    impl<T: ?Sized, F: ?Sized> AssocType for CondType<false, T, F> {
        type Type = F;
    }

    impl<T: ?Sized, F: ?Sized> AssocType for CondType<true, T, F> {
        type Type = T;
    }

    #[allow(clippy::type_complexity)]
    pub struct TypeEq<T, U>(
        PhantomData<(
            // `TypeEq` is invariant over `T` and `U`.
            fn(T) -> T,
            fn(U) -> U,
        )>,
    );

    impl<T> TypeEq<T, T> {
        pub const NEW: Self = TypeEq(PhantomData);
    }

    impl<T, U> TypeEq<T, U> {
        pub const fn coerce(self, from: T) -> U {
            #[repr(C)]
            union Transmuter<From, Into> {
                from: ManuallyDrop<From>,
                into: ManuallyDrop<Into>,
            }

            // SAFETY: `TypeEq` instances can only be constructed if `T` and `U`
            // are the same type.
            unsafe {
                ManuallyDrop::into_inner(
                    Transmuter {
                        from: ManuallyDrop::new(from),
                    }
                    .into,
                )
            }
        }
    }
}
