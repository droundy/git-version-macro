extern crate proc_macro;

use proc_macro::{ TokenStream };
use quote::{ quote };

use std::path::Path;

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let identifier = proc_macro2::TokenStream::from(input);
    if !Path::new(".git").exists() {
        println!("I don't see .git...");
    }
    let cwd = std::env::current_dir().unwrap();
    let head = cwd.join(".git/HEAD").to_str().unwrap().to_string();
    let mut interesting_files = vec![head];
    let refs = Path::new(".git/refs/heads");
    for entry in refs.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            interesting_files.push(cwd.join(entry.path()).to_str().unwrap().to_string());
        }
    }
    let x = quote!{
        fn __unused_by_git_version() {
            // This is included simply to cause cargo to rebuild when
            // a new commit is made.
            #( include_str!(#interesting_files); )*
            // let _head = include_str!(#head);
        }
        const #identifier: &'static str = {
            // let _master = include_str!(".git/refs/heads/master");
            "foo"
        };
    };
    println!("tokens are {}", x);
    x.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
