use crate::is_option;
use crate::GlobalParams;
use crate::PATH_ANYHOW_TRAIT;
use crate::PATH_CLEARSCREEN;
use crate::PATH_INQUIRE;
use crate::PATH_INSPECT;
use crate::PATH_PROMPTABLEDISPLAY_TRAIT;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;
pub fn impl_inspectable_struct(
    fields_options_inspect: &Vec<TokenStream>,
    global_params: &GlobalParams,
    idents_inspect: &[(&Ident, &Type)],
) -> TokenStream {
    // if this type possess no visible fields, just use the method from the trait to see it, else make a menu to select fields.
    let name = &global_params.name;
    let path_inspect = PATH_INSPECT.parse().unwrap();
    let vec_name: TokenStream = format!("Vec{name}").parse().unwrap();
    let method_inspect_vec = generate_method_inspect_vec(&path_inspect);

    let method_inspect = generate_method_inspect(fields_options_inspect, idents_inspect);
    quote! {
        impl #path_inspect for #name {
            #method_inspect
        }
        impl #path_inspect for #vec_name {
            #method_inspect_vec
        }
    }
}

fn generate_method_inspect(
    fields_options: &Vec<TokenStream>,
    idents_visible: &[(&Ident, &Type)],
) -> TokenStream {
    let mut lines_match_ident: Vec<TokenStream> = Vec::new();
    let path_inspect: TokenStream = PATH_INSPECT.parse().unwrap();
    let clear_screen: TokenStream = PATH_CLEARSCREEN.parse().unwrap();
    let path_anyhow: TokenStream = PATH_ANYHOW_TRAIT.parse().unwrap();
    let path_prompt_display: TokenStream = PATH_PROMPTABLEDISPLAY_TRAIT.parse().unwrap();
    let path_inquire: TokenStream = PATH_INQUIRE.parse().unwrap();

    for (n, (ident, ty)) in idents_visible.iter().enumerate() {
        let index = syn::Index::from(n);
        lines_match_ident.push(if is_option(ty) {
            quote! { #index => {
                if let Some(v) = &self.#ident {
                    #path_inspect::inspect_menu(v)?;
                } else {
                    continue;
                }
                }
            }
        } else {
            quote! {
                            #index => #path_inspect::inspect_menu(&self.#ident)?
            }
        });
    }

    lines_match_ident.push(quote! {
        _=> break
    });
    quote! {
            fn inspect_menu(&self) -> #path_anyhow::Result<()> {
    // for structs, inspect must put human_description
    // add a menu to select any fields visible
    // the selected field will have his method inspect called.
                    let mut options = Vec::new();
                    // name of field
                        #( #fields_options)*
                    options.push("Go back".to_string());
                    loop {
                        #clear_screen;
                        println!("{}", #path_prompt_display::display_human(self));
                        match #path_inquire::Select::new("Select the field to view", options.clone()).raw_prompt() {
                Ok(l) => {
                        match l.index {
                            #(#lines_match_ident),*,
                        }
                    },
                Err(#path_inquire::InquireError::OperationCanceled) => break,
                Err(e) => return Err(e.into()),
                        }
                    }
                    #clear_screen;
                    Ok(())
                }
            }
}

fn generate_method_inspect_vec(path_inspect: &TokenStream) -> TokenStream {
    let clear_screen: TokenStream = PATH_CLEARSCREEN.parse().unwrap();
    let path_anyhow: TokenStream = PATH_ANYHOW_TRAIT.parse().unwrap();
    let path_prompt_display: TokenStream = PATH_PROMPTABLEDISPLAY_TRAIT.parse().unwrap();
    let path_inquire: TokenStream = PATH_INQUIRE.parse().unwrap();
    quote! {
    fn inspect_menu(&self) -> #path_anyhow::Result<()> {
        let options = self
            .iter()
            .map(|e| #path_prompt_display::display_short(e))
            .collect::<Vec<String>>();
        loop {
                #clear_screen;
            match #path_inquire::Select::new("Choose the element to see.\nEscape to quit the view", options.clone()).raw_prompt() {
                Ok(l) => #path_inspect::inspect_menu(&self[l.index])?,
                Err(#path_inquire::InquireError::OperationCanceled) => break,
                Err(e) => return Err(e.into()),
            }
        }
        #clear_screen;
        Ok(())
    }
        }
}
