#![allow(non_snake_case)]

mod android_ndk;
mod android_sdk;
mod rustc;

pub use android_ndk::*;
pub use android_sdk::*;
pub use rustc::*;

use crate::error::StdResult;
use std::sync::Arc;

pub trait Dependencies {
    fn check(&self) -> StdResult<()>;
}

pub trait OptionalDependencies {
    fn check(&self) -> StdResult<()>;
}

pub trait Dependency: Sized {
    type Input;

    fn check(&self) -> StdResult<()>;
    fn init(input: Option<Self::Input>) -> StdResult<Arc<Self>>;
}

impl Dependencies for () {
    fn check(&self) -> StdResult<()> {
        Ok(())
    }
}

impl OptionalDependencies for () {
    fn check(&self) -> StdResult<()> {
        Ok(())
    }
}

impl<T: Dependency> OptionalDependencies for Option<Arc<T>> {
    fn check(&self) -> StdResult<()> {
        if let Some(dep) = self {
            dep.check()?;
        }
        Ok(())
    }
}

// Obsoletes and should be replaced after: https://github.com/rust-lang/rfcs/issues/376
macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Dependency),+> Dependencies for ($(Arc<$name>,)+)
        {
            fn check(&self) -> StdResult<()> {
                let ($($name,)+) = self;
                ($($name.check()?,)+);
                Ok(())
            }
        }
        impl<$($name: Dependency),+> OptionalDependencies for ($(Option<Arc<$name>>,)+)
        {
            fn check(&self) -> StdResult<()> {
                let ($($name,)+) = self;
                ($(if let Some($name) = $name { $name.check()?; },)+);
                Ok(())
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
        type Input = ();

        fn check(&self) -> StdResult<()> {
            println!("checked dep1");
            Ok(())
        }

        fn init(_: Option<Self::Input>) -> StdResult<Arc<Self>> {
            Ok(Dep1 {
                path: "dep1".to_owned(),
            }
            .into())
        }
    }

    struct Dep2 {
        dep1: Arc<Dep1>,
    }

    impl Dependency for Dep2 {
        type Input = Arc<Dep1>;

        fn check(&self) -> StdResult<()> {
            println!("checked dep2 for later run with {:?}", self.dep1);
            Ok(())
        }

        fn init(dep1: Option<Self::Input>) -> StdResult<Arc<Self>> {
            if let Some(dep1) = dep1 {
                return Ok(Dep2 { dep1 }.into());
            }
            let dep1 = Dep1::init(None)?;
            Ok(Dep2 { dep1 }.into())
        }
    }

    impl Dep2 {
        fn hello(&self) {
            println!("hello from dep2");
        }
    }

    #[test]
    fn test_checks() -> StdResult<()> {
        let dep1 = Dep1::init(None)?;
        let dep2 = Dep2::init(Some(dep1.clone()))?;
        // Run checks
        dep1.check()?;
        dep2.check()?;
        // Run custom function
        dep2.hello();
        Ok(())
    }
}
