mod android_sdk;
mod rustc;

use crate::error::StdResult;

pub use android_sdk::*;
pub use rustc::*;

pub trait Dependencies {
    fn check() -> StdResult<()>;
}

pub trait Dependency {
    fn check() -> StdResult<()>;
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

    struct Dep1;

    impl Dependency for Dep1 {
        fn check() -> StdResult<()> {
            println!("checked dep1");
            Ok(())
        }
    }

    struct Dep2;

    impl Dependency for Dep2 {
        fn check() -> StdResult<()> {
            println!("checked dep2");
            Ok(())
        }
    }

    #[test]
    fn test_checks() {
        Dep1::check().unwrap();
        <(Dep1, Dep2)>::check().unwrap();
    }
}
