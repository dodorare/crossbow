mod add_lib_to_apk;
mod align_apk;
mod gen_android_manifest;
mod gen_apple_app;
mod gen_apple_plist;
mod gen_minimal_project;
mod gen_unaligned_apk;
mod rust_compile;
mod search_android_dylibs;

pub use add_lib_to_apk::*;
pub use align_apk::*;
pub use gen_android_manifest::*;
pub use gen_apple_app::*;
pub use gen_apple_plist::*;
pub use gen_minimal_project::*;
pub use gen_unaligned_apk::*;
pub use rust_compile::*;
pub use search_android_dylibs::*;

use crate::deps::*;
use crate::error::Result;
use std::collections::HashSet;

pub trait Command {
    type Deps: Checks;
    type Output;

    fn run(&self) -> Result<Self::Output>;
    fn check() -> Result<HashSet<CheckInfo>> {
        Self::Deps::check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                passed: true,
            });
            Ok(checks)
        }
    }

    struct Dep2 {
        pub dep1: Rc<Dep1>,
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

    struct Dep3;

    impl Checks for Dep3 {
        fn check() -> Result<HashSet<CheckInfo>> {
            Ok(HashSet::new())
        }
    }

    struct Command1 {
        pub dep1: Rc<Dep1>,
    }

    impl Command for Command1 {
        type Deps = Dep1;
        type Output = ();

        fn run(&self) -> Result<Self::Output> {
            println!("running command 1");
            Ok(())
        }
    }

    struct Command2 {
        pub dep2: Rc<Dep2>,
        pub dep3: Rc<Dep3>,
    }

    impl Command for Command2 {
        type Deps = (Dep2, Dep3);
        type Output = ();

        fn run(&self) -> Result<Self::Output> {
            println!("running command 2");
            Ok(())
        }
    }

    #[test]
    fn test_command() -> Result<()> {
        // init deps
        let dep1 = Rc::new(Dep1 {
            path: "very/nice/".to_owned(),
        });
        let dep2 = Rc::new(Dep2 { dep1: dep1.clone() });
        let dep3 = Rc::new(Dep3);

        // init commands
        let cmd1 = Command1 { dep1: dep1.clone() };
        let cmd2 = Command2 { dep2, dep3 };

        // check command1 deps if you want
        let _check_info = Command1::check().unwrap();
        // then you can show check info to user
        // println!("{}", check_info);

        // run command1 and command2
        cmd1.run().unwrap();
        cmd2.run().unwrap();
        Ok(())
    }
}
