#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]

use proc_macro::{TokenStream,TokenTree};
/// Rewrites path of module imports in a folder to 'modname/[modname].rs', idea from <users.rust-lang.org/t/rename-mod-rs-to-mod-rs/37688/6>
#[proc_macro] pub fn _mod(item:TokenStream) -> TokenStream {
  let ret = format!(r#"#[path="{}/[{}].rs"] pub mod {};"#
    , item,item,item).parse().unwrap();
  ret
}

/// Allows getting a struct's name as a static string via .as_str() method (with a Derive macro) and impls a similar trait
#[proc_macro_derive(AsStr)] pub fn derive_struct_name_as_str(item:TokenStream) -> TokenStream {
  let mut it = item.into_iter();
  while let Some(tt) = it.next() { match tt {
    TokenTree::Ident(id) => {
      if id.to_string() == "struct" {
        let struct_name = it.next().unwrap().to_string();
        return format!(r#"pub trait AsStr {{fn as_str(&self) -> &'static str;           }}
          impl AsStr for {}               {{fn as_str(&self) -> &'static str {{ "{}" }} }}"#
          ,         struct_name,                                            struct_name).parse().unwrap()
      }}
    _                    => {}
  }}
  panic!("no ident found")
}
