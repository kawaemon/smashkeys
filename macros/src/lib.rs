use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitChar, LitStr, Result, Token,
};

fn ident_or_litstr(stream: ParseStream) -> Result<String> {
    let lookahead = stream.lookahead1();

    if lookahead.peek(Ident) {
        let ident: Ident = stream.parse()?;
        Ok(ident.to_string())
    } else if lookahead.peek(LitStr) {
        let lit: LitStr = stream.parse()?;
        Ok(lit.value())
    } else {
        Err(lookahead.error())
    }
}

fn str_to_char_array(text: &str) -> TokenStream {
    let text = text.chars().map(|c| LitChar::new(c, Span::call_site()));
    quote! { [ #(#text),* ] }
}

#[proc_macro]
pub fn as_char_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let text = parse_macro_input!(input with ident_or_litstr);
    str_to_char_array(&text).into()
}

#[proc_macro]
pub fn segments(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct Input {
        origin: Vec<String>,
        hira: Vec<String>,
    }

    impl Parse for Input {
        fn parse(input: ParseStream) -> Result<Self> {
            let parse = || -> Result<_> {
                let mut ret = vec![];
                let mut first = true;
                loop {
                    if input.peek(Token![,]) {
                        let _: Token![,] = input.parse()?;
                        break;
                    }
                    if input.is_empty() {
                        break;
                    }
                    if first {
                        first = false
                    } else {
                        let _: Token![/] = input.parse()?;
                    }
                    ret.push(ident_or_litstr(input)?);
                }
                Ok(ret)
            };

            Ok(Self {
                origin: parse()?,
                hira: parse()?,
            })
        }
    }

    impl Input {
        fn expand(self) -> TokenStream {
            let segments = self
                .origin
                .into_iter()
                .map(|t| str_to_char_array(&t))
                .zip(self.hira.into_iter().map(|t| str_to_char_array(&t)))
                .map(
                    |(origin, hira)| quote! { Segment::new(#origin.as_slice(), #hira.as_slice()) },
                );

            quote! { [ #(#segments),* ] }
        }
    }

    parse_macro_input!(input as Input).expand().into()
}
