use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use git_version_macro_internal::git_describe;

pub use git_version_macro_internal::declare;
