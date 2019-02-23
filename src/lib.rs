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
    let vec = std::process::Command::new("git")
            .args(&["describe", "--always"])
            .output()
        .expect("failed to execute git").stdout;
    let name = std::str::from_utf8(&vec[..vec.len()-1]).expect("non-utf8 error?!");
    let x = quote!{
        fn __unused_by_git_version() {
            // This is included simply to cause cargo to rebuild when
            // a new commit is made.
            #( include_str!(#interesting_files); )*
        }
        const #identifier: &'static str = {
            #name
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
