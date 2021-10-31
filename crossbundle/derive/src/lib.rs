extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

fn crossbundle_main_logic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    if input.sig.ident != "main" {
        panic!("crossbundle_main can only be used on a function called 'main'")
    }
    TokenStream::from(quote! {
        #[no_mangle]
        #[cfg(target_os = "android")]
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity: *mut std::os::raw::c_void,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize,
        ) {
            crossbow::ndk_glue::init(
                activity as _,
                saved_state as _,
                saved_state_size as _,
                main,
            );
        }

        #[allow(unused)]
        #input
    })
}

#[proc_macro_attribute]
pub fn crossbundle_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    crossbundle_main_logic(attr, item)
}
