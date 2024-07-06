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
            fn get<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) -> ::std::option::Option<&T> {
                self.0.get::<T>()
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

// ####################
// ### SYNC MUTABLE ###
// ####################

#[proc_macro_derive(StateSyncMutableGetMut)]
pub fn derive_state_sync_mutable_get_mut(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableGetMut for #ident {
            fn get_mut<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) -> ::std::option::Option<&mut T> {
                self.0.get_mut::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncMutableInsert)]
pub fn derive_state_sync_mutable_insert(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableInsert for #ident {
            fn insert<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self, data: T) {
                self.0.insert(data)
            }
            fn remove<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) {
                self.0.remove::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncMutableRemoveGetCloned)]
pub fn derive_state_sync_mutable_remove_get_cloned(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableRemoveGetCloned for #ident {
            fn remove_get_cloned<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&mut self) -> ::std::option::Option<T> {
                self.0.remove_get_cloned::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncMutableRemoveGet)]
pub fn derive_state_sync_mutable_remove_get(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableRemoveGet for #ident {
            fn remove_get<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&mut self) -> ::std::option::Option<T> {
                self.0.remove_get::<T>()
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateSyncMutableGetMutOrInsert)]
pub fn derive_state_sync_mutable_get_mut_or_insert(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateSyncMutableGetMutOrInsert for #ident {
            /// Returns data of type `T` mutably, inserts if not found
            fn get_mut_or_insert_with<T: 'static + ::core::marker::Send + ::core::marker::Sync>(
                &mut self,
                get_data: impl ::std::ops::FnOnce() -> T,
            ) -> &mut T {
                self.0.get_mut_or_insert_with(get_data)
            }
            /// Returns data of type `T` mutably, inserts if not found
            fn get_mut_or_insert<T: 'static + ::core::marker::Send + ::core::marker::Sync>(
                &mut self,
                data: T,
            ) -> &mut T {
                self.0.get_mut_or_insert(data)
            }
            /// Returns data of type `T` mutably, inserts default if not found
            fn get_mut_or_insert_default<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::default::Default>(
                &mut self,
            ) -> &mut T {
                self.0.get_mut_or_insert_default::<T>()
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

#[proc_macro_derive(StateAsyncInsert)]
pub fn derive_state_async_insert(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncInsert for #ident {
            async fn insert<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self, data: T) {
                self.0.insert(data).await
            }
            async fn remove<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) {
                self.0.remove::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncRemoveGetCloned)]
pub fn derive_state_async_remove_get_cloned(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncRemoveGetCloned for #ident {
            async fn remove_get_cloned<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::clone::Clone>(&self) -> ::std::option::Option<T> {
                self.0.remove_get_cloned::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncRemoveGet)]
pub fn derive_state_async_remove_get(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncRemoveGet for #ident {
            async fn remove_get<T: 'static + ::core::marker::Send + ::core::marker::Sync>(&self) -> ::std::option::Option<T> {
                self.0.remove_get::<T>().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(StateAsyncGetMutOrInsert)]
pub fn derive_state_async_get_mut_or_insert(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    let ident = derive_input.ident;
    quote! {
        impl StateAsyncGetMutOrInsert for #ident {
            /// Returns data of type `T` mutably, inserts if not found
            async fn get_mut_or_insert_with<T: 'static + ::core::marker::Send + ::core::marker::Sync>(
                &self,
                get_data: impl ::std::ops::FnOnce() -> T + ::core::marker::Send,
            ) -> impl ::std::ops::DerefMut<Target = T> {
                self.0.get_mut_or_insert_with(get_data).await
            }
            /// Returns data of type `T` mutably, inserts if not found
            async fn get_mut_or_insert<T: 'static + ::core::marker::Send + ::core::marker::Sync>(
                &self,
                data: T,
            ) -> impl ::std::ops::DerefMut<Target = T> {
                self.0.get_mut_or_insert(data).await
            }
            /// Returns data of type `T` mutably, inserts default if not found
            async fn get_mut_or_insert_default<T: 'static + ::core::marker::Send + ::core::marker::Sync + ::std::default::Default>(
                &self,
            ) -> impl ::std::ops::DerefMut<Target = T> {
                self.0.get_mut_or_insert_default::<T>().await
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
#[proc_macro_derive(ContextBuilder, attributes(global_state, state, builder_ident))]
pub fn derive_context_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let request_ctx_ident = input.ident;
    let builder_ident = {
        let mut builder_ident = None;

        for attr in input.attrs {
            let syn::Meta::NameValue(name_value) = attr.meta else {
                continue;
            };

            let Some(first_path_segment) = name_value.path.segments.first() else {
                continue;
            };

            if &first_path_segment.ident != "builder_ident" {
                continue;
            }

            let Expr::Lit(value_expr_lit) = name_value.value else {
                panic!("builder_ident must be string literal");
            };

            let Lit::Str(str_lit) = value_expr_lit.lit else {
                panic!("builder_ident must be string literal");
            };

            builder_ident = Some(Ident::new(&str_lit.value(), request_ctx_ident.span()));

            break;
        }

        builder_ident.expect("please define a #[builder_ident]")
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
        #request_ctx_vis struct #builder_ident {
            #(
                pub #state_field_idents: #state_field_types,
            )*
        }

        impl ContextBuilder<#global_field_type> for #builder_ident {
            /// The `RequestCtx` generated, usually fed to a `Request`
            type Output = #request_ctx_ident;

            /// Combines `self` and the global state of the `Router` into `Self::Output`
            async fn build(
                self,
                global_state: #global_field_type,
            ) -> Self::Output {
                #request_ctx_ident {
                    #(
                        #state_field_idents: self.#state_field_idents,
                    )*
                    #global_field_ident: global_state,
                }
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
                fn get_from_ctx(ctx: &#ident) -> &Self {
                    &ctx.#get_state_fields
                }
                fn get_mut_from_ctx(ctx: &mut #ident) -> &mut Self {
                    &mut ctx.#get_state_fields
                }
            }
        )*
    }
    .into()
}
