extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

fn creator_main_logic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    if input.sig.ident != "main" {
        panic!("creator_main can only be used on a function called 'main'")
    }
    TokenStream::from(quote! {
        #[no_mangle]
        #[cfg(target_os = "android")]
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity: *mut std::os::raw::c_void,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize,
        ) {
            creator::android_logger::init_once(
                // TODO: Support different log levels
                creator::android_logger::Config::default().with_min_level(log::Level::Trace),
            );
            creator::ndk_glue::init(
                activity as _,
                saved_state as _,
                saved_state_size as _,
                main,
            );
        }

        #[no_mangle]
        #[cfg(target_os = "ios")]
        extern "C" fn main_rs() {
            main();
        }

        #[allow(unused)]
        #input
    })
}

#[proc_macro_attribute]
pub fn creator_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    creator_main_logic(attr, item)
}
