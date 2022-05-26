use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, LitStr};

fn str_to_char_array(s: &str) -> String {
    let mut result = "[".to_string();
    for c in s.chars() {
        result += &format!("'{c}',");
    }
    result += "]";
    result
}

#[proc_macro]
pub fn as_char_array(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    str_to_char_array(&lit.value()).parse().unwrap()
}

#[proc_macro]
pub fn ident_as_char_array(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    str_to_char_array(&ident.to_string()).parse().unwrap()
}
