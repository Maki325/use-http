use proc_macro::TokenStream;
mod data;
use data::Data;
use quote::quote;
use syn::parse::ParseStream;

#[proc_macro]
pub fn use_https(input: TokenStream) -> TokenStream {
  let output = syn::parse::Parser::parse2(
    |input: ParseStream<'_>| {
      let mut output = Vec::new();
      while !input.is_empty() {
        let element = input.parse::<Data>()?;
        output.push(element);
      }
      return Ok(output);
    },
    input.into(),
  );

  return match output {
    Ok(data) => quote! {
      #(#data)*
    }
    .into(),
    Err(err) => err.to_compile_error().into(),
  };
}
