#![allow(non_snake_case)]

mod android_ndk;
mod android_sdk;
mod rustc;

pub use android_ndk::*;
pub use android_sdk::*;
pub use rustc::*;

use crate::error::Result;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CheckInfo {
    pub dependency_name: String,
    pub check_name: String,
    pub passed: bool,
}

impl CheckInfo {
    fn invert_passed(mut self) -> CheckInfo {
        self.passed = !self.passed;
        self
    }
}

pub trait IntoCheckInfo: Sized {
    fn check_passed(self) -> CheckInfo;
    fn check_failed(self) -> CheckInfo {
        self.check_passed().invert_passed()
    }
}

pub trait Checks {
    fn check() -> Result<HashSet<CheckInfo>>;
}

impl Checks for () {
    fn check() -> Result<HashSet<CheckInfo>> {
        Ok(HashSet::new())
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Checks),+> Checks for ($($name,)+)
        {
            fn check() -> Result<HashSet<CheckInfo>> {
                let mut merged = HashSet::new();
                for s in vec![$($name::check()?,)+] {
                    merged.extend(s);
                }
                Ok(merged)
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
    use std::collections::HashSet;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    struct Dep1 {
        pub path: String,
    }

    impl Checks for Dep1 {
        fn check() -> Result<HashSet<CheckInfo>> {
            let mut checks = HashSet::new();
            println!("checked first check of dep1");
            checks.insert(CheckInfo {
                dependency_name: "dep1".to_owned(),
                check_name: "first check".to_owned(),
                passed: false,
            });
            println!("checked second check of dep1");
            checks.insert(CheckInfo {
                dependency_name: "dep1".to_owned(),
                check_name: "second check".to_owned(),
                passed: false,
            });
            Ok(checks)
        }
    }

    struct Dep2 {
        pub dep1: Rc<Dep1>,
    }

    impl Dep2 {
        pub fn hello(&self) {
            println!("hello!");
        }
    }

    impl Checks for Dep2 {
        fn check() -> Result<HashSet<CheckInfo>> {
            let mut checks = HashSet::new();
            println!("checked only one check of dep2");
            checks.insert(CheckInfo {
                dependency_name: "dep2".to_owned(),
                check_name: "only one check".to_owned(),
                passed: false,
            });
            Ok(checks)
        }
    }

    #[test]
    fn test_checks() {
        // init deps
        let dep1 = Rc::new(Dep1 {
            path: "very/nice/".to_owned(),
        });
        let dep2 = Dep2 { dep1: dep1.clone() };

        // check deps
        let _dep1_check_info = Dep1::check().unwrap();
        let _dep2_check_info = Dep2::check().unwrap();
        // then you can show check info to user
        // println!("{} {}", dep1_check_info, dep2_check_info);

        // run custom function
        dep2.hello();
    }
}
