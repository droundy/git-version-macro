extern crate proc_macro;

use proc_macro::{ TokenStream };
use quote::{ quote };

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let identifier = proc_macro2::TokenStream::from(input);
    let x = quote!{
        const #identifier: &'static str = "foo";
    };
    x.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
