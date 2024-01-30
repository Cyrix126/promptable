use proc_macro2::TokenStream;
use quote::quote;

use crate::is_option;
use crate::FieldParams;
use crate::GlobalParams;
use crate::PATH_ANYHOW_TRAIT;
use crate::PATH_CLEARSCREEN;
use crate::PATH_DERIVE_MORE;
use crate::PATH_INQUIRE;
use crate::PATH_MENU;
use crate::PATH_PROMPTABLEDISPLAY_TRAIT;
use crate::PATH_PROMPTABLE_TRAIT;
pub(crate) fn impl_promptable_vec_struct(
    prepare_values_fields_add: &Vec<TokenStream>,
    fields_struct: &Vec<TokenStream>,
    global_params: &GlobalParams,
) -> TokenStream {
    let name = &global_params.name;
    let name_str = &global_params.name.to_string();
    let tuple = &global_params.tuple;
    let params_as_named_value = &global_params.params_as_named_value;
    let vec_name: TokenStream = format!("Vec{name}").parse().unwrap();
    let path_promptable: TokenStream = PATH_PROMPTABLE_TRAIT.parse().unwrap();
    let path_anyhow: TokenStream = PATH_ANYHOW_TRAIT.parse().unwrap();
    let clear_screen: TokenStream = PATH_CLEARSCREEN.parse().unwrap();
    let path_inquire: TokenStream = PATH_INQUIRE.parse().unwrap();
    let path_menu: TokenStream = PATH_MENU.parse().unwrap();
    let path_prompt_display: TokenStream = PATH_PROMPTABLEDISPLAY_TRAIT.parse().unwrap();
    let path_derive_more: TokenStream = PATH_DERIVE_MORE.parse().unwrap();
    let f_del = if let Some(f) = &global_params.function_del {
        let func_del: TokenStream = f.parse().unwrap();
        quote! {
            #func_del;
        }
    } else {
        quote! {}
    };

    quote! {
                // least bad solution ?
                #[derive(#path_derive_more::Deref, #path_derive_more::DerefMut, Clone, #path_derive_more::Display)]
                #[display(fmt=#name_str)]
                pub struct #vec_name(pub Vec<#name>);

                        impl #path_promptable<(#tuple)> for #vec_name {
            fn new_by_prompt(params: (#tuple)) -> #path_anyhow::Result<Option<#vec_name>> {
                if let Some(r) = #name::new_by_prompt(params)? {
                    #clear_screen;
                 Ok(Some(#vec_name(vec![r])))
                } else {
                 Ok(None)
                }
            }
            fn modify_by_prompt(&mut self, params: (#tuple)) -> #path_anyhow::Result<bool> {
                let options_menu = #path_menu::MenuClassic::consts();
                // idea: rather than cloning the self and chaning a new self or an old self, why not create a vec and only add what changes and then apply on self if confirmed ?
                let mut modified = false;
                let restore_self = self.clone();
                loop {
                    while self.is_empty() {
                        if let Some(s) = #name::new_by_prompt(params)? {
                            self.push(s);
                            modified = true;
                        }
                    }
                    #clear_screen;
                    let value_name = #name_str;
                    let msg_menu = format!("{} {}:", self.len(), value_name);
                    if let Some(choix) = #path_inquire::Select::new(&msg_menu, options_menu.to_vec()).prompt_skippable()? {
                        match choix {
                            #path_menu::MenuClassic::ADD => {
                                if self.add_by_prompt_vec(params)? {modified = true}},
                            #path_menu::MenuClassic::MODIFY => {
                                if let Some(index) = self.select()? { if
                                    self.modify_by_prompt_vec(params, index)? {modified = true}}
                            },
                            #path_menu::MenuClassic::DELETE => {
                                if let Some(selection) = self.multiselect()? { if
                                self.delete_by_prompt_vec(params, selection)? { modified = true
                                }
                            } },
                            #path_menu::MenuClassic::CANCEL => {
                             if #path_menu::menu_cancel(&restore_self, self)? {
                                    modified = false;
                              break;
                             }
                            }
                            _ => {
                                if #path_menu::menu_confirm(&restore_self, &self)? {
                                    break;
                                }
                            }
                        }
                    } else {
                             if #path_menu::menu_cancel(&restore_self, self)? {
                              break;
                             }
                    }
                }
                    #clear_screen;
                Ok(modified)
            }
                }

    impl #vec_name {
        pub fn add_by_prompt_vec(&mut self, params: (#tuple))  -> #path_anyhow::Result<bool>{
                    // rename tuple parts with name to use with functions if any
                                #( #params_as_named_value )*
                #( #prepare_values_fields_add )*
                            // loop {
                                #clear_screen;
                        // use macro for which fields to ask and how and value to prepare
                let new = #name {
                    #( #fields_struct ),*
                };
                         self.push(new);
                                // break
                            // }
                            Ok(true)
        }
        pub fn delete_by_prompt_vec(&mut self, params: (#tuple), mut selection: Vec<usize>)  -> #path_anyhow::Result<bool> {
            selection.sort_unstable_by(|a, b| b.cmp(a));
            for index in selection {
                let element = &self[index];
                    #f_del
                self.remove(index);
                    return Ok(true);
            }
            Ok(false)
        }
        pub fn modify_by_prompt_vec(
            &mut self,
            params: (#tuple),
            index: usize
        ) -> #path_anyhow::Result<bool> {
             #clear_screen;
            Ok(<#name as #path_promptable<(#tuple)>>::modify_by_prompt(&mut self[index], params)?)
        }
        pub fn multiselect(&self) -> #path_anyhow::Result<Option<Vec<usize>>> {

            if let Some(l) = #path_inquire::MultiSelect::new(
                "Select objects",
                self.iter().map(|e| <#name as #path_prompt_display>::display_short(e)).collect(),
            )
            .raw_prompt_skippable()?
            {
                    return Ok(Some(l.into_iter().map(|c|c.index).collect::<Vec<usize>>()))
            } Ok(None)
        }

        pub fn select(&self) -> #path_anyhow::Result<Option<usize>> {
            match #path_inquire::Select::new(
                "Select object",
                self.iter().map(|e| <#name as #path_prompt_display>::display_short(e)).collect(),
            )
            .raw_prompt() {
                    Ok(c) => Ok(Some(c.index)),
                    Err(#path_inquire::error::InquireError::OperationCanceled) => Ok(None),
                    Err(e) => Err(e)?
                }
            }
        }
        }
}

pub(crate) fn generate_values_add_by_prompt(
    opts: &FieldParams,
    value: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ident = opts.ident;
    if opts.multiple_once {
        quote! {
                self[0].clone().#ident
        }
    } else if let Some(f) = &opts.function_add {
        let f: TokenStream = f.parse().unwrap();
        if is_option(opts.ty) {
            quote! {
                #f
            }
        } else {
            quote! {
             if let Some(v) = #f {
                          v
                    } else {
                        return Ok(false)
                    }
            }
        }
    } else {
        quote! {#value}
    }
}
