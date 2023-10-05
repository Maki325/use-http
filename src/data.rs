use std::str::FromStr;

use proc_macro2::{Delimiter, Group, Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  Error, LitStr, Result, Token,
};

pub enum Data {
  Named {
    name: Ident,
    destructures: Vec<Ident>,
    url: String,
  },
  Nameless {
    url: String,
  },
}

impl Parse for Data {
  fn parse(input: ParseStream) -> Result<Self> {
    let import = input.parse::<Ident>()?;

    if import != "import" {
      return Err(Error::new(import.span(), "Expected keyword `import`!"));
    }

    if let Some(url) = input.parse::<Option<LitStr>>()? {
      input.parse::<Token![;]>()?;
      return Ok(Self::Nameless { url: url.value() });
    }

    let name = input.parse::<Ident>()?;

    let mut destructures = Vec::new();
    if let Some(_) = input.parse::<Option<Token![:]>>()? {
      input.parse::<Token![:]>()?;
      let group = input.parse::<Group>()?;
      if group.delimiter() != Delimiter::Brace {
        return Err(Error::new(group.span(), "Expected braces!"));
      }

      destructures = syn::parse::Parser::parse2(
        Punctuated::<Ident, Token![,]>::parse_terminated,
        group.stream(),
      )?
      .into_iter()
      .collect::<Vec<Ident>>();
    }

    let from = input.parse::<Ident>()?;

    if from != "from" {
      return Err(Error::new(import.span(), "Expected keyword `from`!"));
    }

    let url = input.parse::<LitStr>()?;
    input.parse::<Token![;]>()?;

    return Ok(Data::Named {
      name,
      destructures,
      url: url.value(),
    });
  }
}

fn url_to_token_stream(url: &String) -> TokenStream {
  let response = match reqwest::blocking::get(url) {
    Ok(response) => response,
    Err(err) => {
      panic!(
        "[ERROR] Couldn't fetch the code from \"{}\", because {}",
        url, err
      );
    }
  };

  let text = match response.text() {
    Ok(text) => text,
    Err(err) => {
      panic!(
        "[ERROR] Couldn't get text from response from \"{}\", because {}",
        url, err
      );
    }
  };

  let stream = match TokenStream::from_str(&text) {
    Ok(stream) => stream,
    Err(err) => {
      panic!(
        "[ERROR] Couldn't parse the code from \"{}\", because {}",
        url, err
      );
    }
  };
  return stream;
}

impl ToTokens for Data {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      Data::Nameless { url } => tokens.extend(url_to_token_stream(url)),
      Data::Named {
        name,
        destructures,
        url,
      } => {
        let stream = url_to_token_stream(url);
        let destructures = if destructures.len() == 0 {
          TokenStream::new()
        } else {
          quote! {
            use #name::{#(#destructures),*};
          }
        };
        tokens.extend(quote! {
          mod #name {
            #stream
          }
          #destructures
        });
      }
    }
  }
}
