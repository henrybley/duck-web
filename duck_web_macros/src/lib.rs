use proc_macro::TokenStream;
use quote::quote;
use stringcase::Caser;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, ItemFn, LitStr, Token};

struct RouteArgs {
    path: String,
    method: String,
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let method = input.parse::<syn::Ident>()?.to_string();
        Ok(RouteArgs { path, method })
    }
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let RouteArgs { path, method } = parse_macro_input!(args as RouteArgs);
    let func_name = &func.sig.ident;
    let struct_name = syn::Ident::new(&format!("{}Handler", func_name.to_string().to_pascal_case()), func_name.span());
    let func_body = &func.block;
    let func_input = &func.sig.inputs.first().unwrap(); // Get the first (and likely only) input
    let registration_name = format!("{}_registration", func_name);
    let register_ident = syn::Ident::new(&registration_name, func_name.span());

    let expanded = quote! {
        #[derive(Clone)]
        pub struct #struct_name {
            route_pattern: ::duck_web_core::router::RoutePattern,
        }
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    route_pattern: ::duck_web_core::router::RoutePattern::new(#path, #method),
                }
            }
        }
        impl ::duck_web_core::handler::RouteHandler for #struct_name {
            fn path_pattern(&self) -> &::duck_web_core::router::RoutePattern {
                &self.route_pattern
            }
            fn handle(&self, req: ::duck_web_core::http::Request) -> ::duck_web_core::http::Response {
                #struct_name::call(req)
            }
        }
        impl #struct_name {
            fn call(#func_input) -> ::duck_web_core::http::Response {
                #func_body
            }
        }

        #[ctor::ctor]
        fn #register_ident() {
            ::duck_web_core::router::register_route(Box::new(#struct_name::new()));
        }
    };

    TokenStream::from(expanded)
}
