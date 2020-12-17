#![allow(non_snake_case)]

mod android_ndk;
mod android_sdk;
mod rustc;

pub use android_ndk::*;
pub use android_sdk::*;
pub use rustc::*;

use crate::error::StdResult;
use std::rc::Rc;

pub trait Dependencies: Sized {
    fn check(&self) -> StdResult<()>;
    fn init() -> StdResult<Self>;
}

pub trait Dependency: Sized {
    fn check(&self) -> StdResult<()>;
    fn init() -> StdResult<Rc<Self>>;
}

impl Dependencies for () {
    fn check(&self) -> StdResult<()> {
        Ok(())
    }
    fn init() -> StdResult<Self> {
        Ok(())
    }
}

impl<T: Dependency> Dependencies for Option<Rc<T>> {
    fn check(&self) -> StdResult<()> {
        if let Some(dep) = self {
            dep.check()?;
        }
        Ok(())
    }
    fn init() -> StdResult<Self> {
        Ok(None)
    }
}

// Obsoletes and should be replaced after: https://github.com/rust-lang/rfcs/issues/376
macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Dependency),+> Dependencies for ($(Rc<$name>,)+)
        {
            fn check(&self) -> StdResult<()> {
                let ($($name,)+) = self;
                ($($name.check()?,)+);
                Ok(())
            }
            fn init() -> StdResult<Self> {
                Ok(($($name::init()?,)+))
            }
        }
        impl<$($name: Dependency),+> Dependencies for ($(Option<Rc<$name>>,)+)
        {
            fn check(&self) -> StdResult<()> {
                let ($($name,)+) = self;
                ($(if let Some($name) = $name { $name.check()?; },)+);
                Ok(())
            }
            fn init() -> StdResult<Self> {
                Ok(($(Option::<Rc<$name>>::None,)+))
            }
        }
    };
}

// TODO: Replace with proc macro or better macro_rules
tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O}
tuple_impls! { A B C D E F G H I J K L M N O P }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Dep1 {
        path: String,
    }

    impl Dependency for Dep1 {
        fn check(&self) -> StdResult<()> {
            println!("checked dep1");
            Ok(())
        }

        fn init() -> StdResult<Rc<Self>> {
            Ok(Dep1 {
                path: "dep1".to_owned(),
            }
            .into())
        }
    }

    struct Dep2 {
        dep1: Rc<Dep1>,
    }

    impl Dependency for Dep2 {
        fn check(&self) -> StdResult<()> {
            println!("checked dep2 for later run with {:?}", self.dep1);
            Ok(())
        }

        fn init() -> StdResult<Rc<Self>> {
            let dep1 = Dep1::init()?;
            Ok(Dep2 { dep1 }.into())
        }
    }

    impl Dep2 {
        fn new(dep1: Rc<Dep1>) -> Rc<Self> {
            Dep2 { dep1 }.into()
        }

        fn hello(&self) {
            println!("hello from dep2");
        }
    }

    #[test]
    fn test_checks() -> StdResult<()> {
        let dep1 = Dep1::init()?;
        let dep2 = Dep2::new(dep1.clone());
        // Run checks
        dep1.check()?;
        dep2.check()?;
        // Run custom function
        dep2.hello();
        Ok(())
    }
}
