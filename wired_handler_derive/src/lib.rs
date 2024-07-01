use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, Ident, Lit, Meta, PathArguments, Type,
};

/// Derive `State`
#[proc_macro_derive(State)]
pub fn derive_state(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl State for #ident {}
    }
    .into()
}

// ######################
// ### SYNC IMMUTABLE ###
// ######################

#[proc_macro_derive(StateSyncGet)]
pub fn derive_state_sync_get(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncGet for #ident {
            fn get<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) -> ::std::option::Option<impl ::std::ops::Deref<Target = T>> {
                self.0.get::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncGetMut)]
pub fn derive_state_sync_get_mut(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncGetMut for #ident {
            fn get_mut<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) -> ::std::option::Option<impl ::std::ops::DerefMut<Target = T>> {
                self.0.get_mut::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncGetCloned)]
pub fn derive_state_sync_get_cloned(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncGetCloned for #ident {
            fn get_cloned<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&self) -> ::std::option::Option<T> {
                self.0.get_cloned::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncProvide)]
pub fn derive_state_sync_provide(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncProvide for #ident {
            fn provide<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self, data: T) {
                self.0.provide(data)
            }
            fn remove<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) {
                self.0.remove::<T>()
            }
            fn remove_get<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&self) -> ::std::option::Option<T> {
                self.0.remove_get::<T>()
            }
        }
    }
    .into()
}

// ####################
// ### SYNC MUTABLE ###
// ####################

#[proc_macro_derive(StateSyncMutableGetMut)]
pub fn derive_state_sync_mutable_get_mut(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableGetMut for #ident {
            fn get_mut<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) -> ::std::option::Option<impl ::std::ops::DerefMut<Target = T>> {
                self.0.get_mut::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncMutableProvide)]
pub fn derive_state_sync_mutable_provide(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableProvide for #ident {
            fn provide<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self, data: T) {
                self.0.provide(data)
            }
            fn remove<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) {
                self.0.remove::<T>()
            }
            fn remove_get<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&mut self) -> ::std::option::Option<T> {
                self.0.remove_get::<T>()
            }
        }
    }
    .into()
}

// #######################
// ### ASYNC IMMUTABLE ###
// #######################

#[proc_macro_derive(StateAsyncGet)]
pub fn derive_state_async_get(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncGet for #ident {
            async fn get<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) -> ::std::option::Option<impl ::std::ops::Deref<Target = T>> {
                self.0.get::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncGetMut)]
pub fn derive_state_async_get_mut(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncGetMut for #ident {
            async fn get_mut<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) -> ::std::option::Option<impl ::std::ops::DerefMut<Target = T>> {
                self.0.get_mut::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncGetCloned)]
pub fn derive_state_async_get_cloned(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncGetCloned for #ident {
            async fn get_cloned<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&self) -> ::std::option::Option<T> {
                self.0.get_cloned::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncProvide)]
pub fn derive_state_async_provide(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncProvide for #ident {
            async fn provide<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self, data: T) {
                self.0.provide(data).await
            }
            async fn remove<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) {
                self.0.remove::<T>().await
            }
            async fn remove_get<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&self) -> ::std::option::Option<T> {
                self.0.remove_get::<T>().await
            }
        }
    }
    .into()
}

// #####################
// ### ASYNC MUTABLE ###
// #####################

#[proc_macro_derive(StateAsyncMutableGetMut)]
pub fn derive_state_async_mutable_get_mut(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncMutableGetMut for #ident {
            async fn get_mut<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) -> ::std::option::Option<impl ::std::ops::DerefMut<Target = T>> {
                self.0.get_mut::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncMutableProvide)]
pub fn derive_state_async_mutable_provide(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncMutableProvide for #ident {
            async fn provide<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self, data: T) {
                self.0.provide(data).await
            }
            async fn remove<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) {
                self.0.remove::<T>().await
            }
            async fn remove_get<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&mut self) -> ::std::option::Option<T> {
                self.0.remove_get::<T>().await
            }
        }
    }
    .into()
}

// ###############
// ### CONTEXT ###
// ###############

fn attr_is(attr: &Attribute, value: &str) -> bool {
    let Meta::Path(path) = &attr.meta else {
        return false;
    };

    if path.leading_colon.is_some() {
        return false;
    }

    if path.segments.len() != 1 {
        return false;
    }

    let Some(first_segment) = path.segments.first() else {
        return false;
    };

    let PathArguments::None = first_segment.arguments else {
        return false;
    };

    if first_segment.ident != value {
        return false;
    };

    true
}

/// Derive `Context`
#[proc_macro_derive(Context)]
pub fn derive_context(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl Context for #ident {}
    }
    .into()
}

/** Derive `ContextBuilder`
* `#[builder_ident = "MyBuilder"]` defines the name of the builder (required)
* `#[error_ident = "MyBuilderError"]` defines the name of the builder error (required)
* `#[global_state]` marks the global state (field) which is inserted when building (required)
* `#[state]` marks a state (field) which has to be inserted before building
*/
#[proc_macro_derive(
    ContextBuilder,
    attributes(global_state, state, builder_ident, error_ident)
)]
pub fn derive_context_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let request_ctx_ident = input.ident;
    let (builder_ident, error_ident) = {
        let mut builder_ident = None;
        let mut error_ident = None;

        for attr in input.attrs {
            let syn::Meta::NameValue(name_value) = attr.meta else {
                continue;
            };

            let Some(first_path_segment) = name_value.path.segments.first() else {
                continue;
            };

            if &first_path_segment.ident != "builder_ident"
                && &first_path_segment.ident != "error_ident"
            {
                continue;
            }

            let Expr::Lit(value_expr_lit) = name_value.value else {
                continue;
            };

            let Lit::Str(str_lit) = value_expr_lit.lit else {
                continue;
            };

            let ident = Some(Ident::new(&str_lit.value(), request_ctx_ident.span()));

            match &first_path_segment.ident {
                path_ident if path_ident == "builder_ident" => builder_ident = ident,
                path_ident if path_ident == "error_ident" => error_ident = ident,
                _ => unreachable!("can only be either builder_ident or error_ident"),
            }

            if builder_ident.is_some() && error_ident.is_some() {
                break;
            }
        }

        (
            builder_ident.expect("please define a #[builder_ident]"),
            error_ident.expect("please define a #[error_ident]"),
        )
    };

    let request_ctx_vis = input.vis;

    let Data::Struct(data_struct) = input.data else {
        panic!("A RequestCtx must be a struct");
    };

    let (state_field_idents, state_field_types): (Vec<Ident>, Vec<Type>) = data_struct
        .clone()
        .fields
        .into_iter()
        .filter_map(|field| {
            if !field.attrs.iter().any(|attr| attr_is(attr, "state"))
                || field.attrs.iter().any(|attr| attr_is(attr, "global_state"))
            {
                return None;
            }
            Some((field.ident?, field.ty))
        })
        .unzip();

    let (global_field_ident, global_field_type) = data_struct
        .fields
        .into_iter()
        .find_map(|field| {
            if !field.attrs.iter().any(|attr| attr_is(attr, "global_state")) {
                return None;
            }

            Some((field.ident?, field.ty))
        })
        .expect("A RequestCtx must have one field that is #[global_state]");

    quote! {
        #[derive(::std::fmt::Debug)]
        #request_ctx_vis enum #error_ident {
            MissingField,
        }

        impl ::std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("missing field")
            }
        }

        impl ::std::error::Error for #error_ident {}

        #request_ctx_vis struct #builder_ident {
            #(
                #state_field_idents: ::std::option::Option<#state_field_types>,
            )*
        }

        impl #builder_ident {
            pub fn new() -> Self {
                Self {
                    #(
                        #state_field_idents: ::std::option::Option::None,
                    )*
                }
            }
            #(
                pub fn #state_field_idents(&mut self, #state_field_idents: #state_field_types) -> &mut Self {
                    self.#state_field_idents = ::std::option::Option::Some(#state_field_idents);
                    self
                }
            )*
        }

        impl ContextBuilder<#global_field_type> for #builder_ident {
            /// The `RequestCtx` generated, usually fed to a `Request`
            type Output = #request_ctx_ident;
            type Error = #error_ident;

            /// Combines `self` and the global state of the `Router` into `Self::Output`
            async fn build(
                self,
                global_state: #global_field_type,
            ) -> ::std::result::Result<Self::Output, Self::Error> {
                ::std::result::Result::Ok(#request_ctx_ident {
                    #(
                        #state_field_idents: self.#state_field_idents.ok_or(Self::Error::MissingField)?,
                    )*
                    #global_field_ident: global_state,
                })
            }
        }
    }
    .into()
}

/// Derive `GetState` for all fields marked with either `#[state]` or `#[global_state]`
#[proc_macro_derive(GetState, attributes(state, global_state))]
pub fn derive_get_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(data) = input.data else {
        panic!("GetState can only be implemented for structs");
    };
    let ident = input.ident;
    let (get_state_fields, get_state_types): (Vec<Ident>, Vec<Type>) = data
        .fields
        .into_iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr_is(attr, "state") || attr_is(attr, "global_state"))
        })
        .map(|field| {
            (
                field
                    .ident
                    .expect("GetState can only be implemented for named structs"),
                field.ty,
            )
        })
        .unzip();

    quote! {
        #(
            impl GetState<#ident> for #get_state_types {
                async fn get_from_ctx(ctx: &#ident) -> impl ::std::ops::Deref<Target = Self> {
                    &ctx.#get_state_fields
                }
                async fn get_mut_from_ctx(ctx: &mut #ident) -> impl ::std::ops::DerefMut<Target = Self> {
                    &mut ctx.#get_state_fields
                }
            }
        )*
    }
    .into()
}
