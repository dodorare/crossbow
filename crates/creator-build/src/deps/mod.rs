mod android_sdk;
mod rustc;

use crate::error::StdResult;

pub use android_sdk::*;
pub use rustc::*;

pub trait Dependencies {
    fn check() -> StdResult<()>;
}

pub trait Dependency: Sized {
    type Input;

    fn check() -> StdResult<()>;
    fn get(input: Self::Input) -> StdResult<Self>;
}

impl Dependencies for () {
    fn check() -> StdResult<()> {
        Ok(())
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Dependency),+> Dependencies for ($($name,)+)
        {
            fn check() -> StdResult<()> {
                $($name::check()?;)+
                Ok(())
            }
        }
    };
}

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

    struct Dep1 {
        path: String,
    }

    impl Dependency for Dep1 {
        type Input = ();

        fn check() -> StdResult<()> {
            println!("checked dep1");
            Ok(())
        }

        fn get(_: Self::Input) -> StdResult<Self> {
            Ok(Dep1 {
                path: "dep1".to_owned(),
            })
        }
    }

    struct Dep2;

    impl Dependency for Dep2 {
        type Input = String;

        fn check() -> StdResult<()> {
            println!("checked dep2");
            Ok(())
        }

        fn get(path: Self::Input) -> StdResult<Self> {
            println!("dep2 got input: {}", path);
            Ok(Dep2)
        }
    }

    impl Dep2 {
        fn hello(&self) {
            println!("hello from dep2");
        }
    }

    #[test]
    fn test_checks() {
        Dep1::check().unwrap();
        <(Dep1, Dep2)>::check().unwrap();

        let dep1 = Dep1::get(()).unwrap();
        let dep2 = Dep2::get(dep1.path).unwrap();
        dep2.hello();
    }
}
