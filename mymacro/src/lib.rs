use syn::{ItemFn,FnArg,Pat, parse_macro_input};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote,ToTokens};
use proc_macro_error::{proc_macro_error,abort,set_dummy};


#[proc_macro_error]
#[proc_macro_attribute]
pub fn wrap_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let visibility = &func.vis;
    let signature = &func.sig;
    let inputs = &signature.inputs;
    let block = &func.block;
    let funcname = &signature.ident;
    let comma = ",".parse::<TokenStream2>().unwrap();
    let attrs = func.attrs
        .iter()
        // put all inner attributes outside
        .map(|x| syn::Attribute{style:syn::AttrStyle::Outer, ..x.clone()})
        .map(|x| x.to_token_stream())
        .reduce(|mut x, y| { y.to_tokens(& mut x); x} );

    // build up a list of arguments from the list of parameters:
    let mut args_mut = TokenStream2::new();
    for (i, q) in inputs.iter().enumerate() {
        match q {
            FnArg::Typed(p) => {
                if let Pat::Ident(i) = &*p.pat {
                    i.ident.to_tokens(&mut args_mut);
                    comma.to_tokens(&mut args_mut);
                } else {
                    set_dummy(func.to_token_stream());
                    abort!(
                        p,
                        format!(
                            "Cannot decorate this function. \
                             Does not know how to handle {}th parameter",
                            i+1,
                        )
                    );
                }
            },
            FnArg::Receiver(r) => {
                set_dummy(func.to_token_stream());
                abort!(r, "Cannot decorate method with self receiver.");
            },
        };
    }
    let args = args_mut;
    let tokens2 = quote! {
        #attrs #visibility #signature {
            #attrs #signature #block;
            println!("before calling `{}`", stringify!(#funcname) );
            let res = #funcname(#args);
            println!("after calling `{}`", stringify!(#funcname) );
            res
        }
    };

    let tokens: TokenStream = tokens2.into();
    println!("decorating {}", funcname);
    println!("generated code = {}", tokens);
    tokens
}
