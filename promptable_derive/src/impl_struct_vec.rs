use proc_macro2::TokenStream;
use quote::quote;

use crate::is_option;
use crate::FieldParams;
use crate::GlobalParams;
pub(crate) fn impl_promptable_vec_struct(
    fields_multiple_add: Vec<TokenStream>,
    global_params: &GlobalParams,
) -> TokenStream {
    let name = &global_params.name;
    let msg_mod = &global_params.msg_mod;
    let tuple = &global_params.tuple;
    let params_as_named_value = &global_params.params_as_named_value;
    let vec_name: TokenStream = format!("Vec{name}").parse().unwrap();

    quote! {
                // least bad solution ?
                #[derive(promptable::derive_more::Deref, promptable::derive_more::DerefMut)]
                pub struct #vec_name(Vec<#name>);

                        impl promptable::Promptable<(#tuple)> for #vec_name {
            fn new_by_prompt(params: (#tuple)) -> promptable::anyhow::Result<Option<#vec_name>> {
                if let Some(r) = #name::new_by_prompt(params)? {
                 Ok(Some(#vec_name(vec![r])))
                } else {
                 Ok(None)
                }
            }
            fn modify_by_prompt(&mut self, params: (#tuple)) -> promptable::anyhow::Result<()> {
                let options_menu = promptable::menu::MenuClassic::consts();
                // idea: rather than cloning the self and chaning a new self or an old self, why not create a vec and only add what changes and then apply on self if confirmed ?
                let restore_self = self.clone();
                loop {
                    while self.is_empty() {
                        if let Some(s) = #name::new_by_prompt(params)? {
                            self.push(s);
                        }
                    }
                    promptable::clear_screen();
                    if let Some(choix) = inquire::Select::new(#msg_mod, options_menu.to_vec()).prompt_skippable()? {
                        match choix {
                            promptable::menu::MenuClassic::ADD => self.add_by_prompt_vec(params)?,
                            promptable::menu::MenuClassic::MODIFY => self.modify_by_prompt_vec(params)?,
                            promptable::menu::MenuClassic::DELETE => self.delete_by_prompt_vec(params)?,
                            promptable::menu::MenuClassic::CANCEL => {
                             if promptable::menu::menu_cancel(&restore_self, self)? {
                              break;
                             }
                            }
                            _ => {// confirm
                                if promptable::menu::menu_confirm(&restore_self, &self)? {
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok(())
            }

                }

    impl #vec_name {
                    fn add_by_prompt_vec(&mut self, params: (#tuple))  -> promptable::anyhow::Result<()>{
                    // rename tuple parts with name to use with functions if any
                                #( #params_as_named_value )*
                            // loop {
                                promptable::clear_screen();
                        // use macro for which fields to ask and how and value to prepare
                let new = #name {
                    #( #fields_multiple_add ),*
                };
                         self.push(new);
                                // break
                            // }
                            Ok(())
                        }

                         fn delete_by_prompt_vec(&mut self, params: (#tuple))  -> promptable::anyhow::Result<()> {
                                promptable::clear_screen();
            let choix = match inquire::MultiSelect::new(
                "Select objects to delete",
                self.iter().map(|e| <#name as promptable::display::PromptableDisplay>::display_short(e)).collect(),
            )
            .raw_prompt_skippable()?
            {
                Some(l) => l,
                None => return Ok(()),
            };
            let mut indexes = Vec::new();
            for c in choix {
                indexes.push(c.index);
            }
            indexes.sort_unstable_by(|a, b| b.cmp(a));
            for index in indexes {
                self.remove(index);
            }
            Ok(())
                        }
        fn modify_by_prompt_vec(
            &mut self,
            params: (#tuple),
        ) -> promptable::anyhow::Result<()> {
                                promptable::clear_screen();
            let choix = inquire::Select::new(
                "Select object to modify",
                self.iter().map(|e| <#name as promptable::display::PromptableDisplay>::display_short(e)).collect(),
            )
            .raw_prompt()?;
            // self[choix.index].modify_by_prompt(params)?;
            <#name as promptable::Promptable<(#tuple)>>::modify_by_prompt(&mut self[choix.index], params)?;
            Ok(())
        }
            }
        }
}

pub(crate) fn generate_line_add_by_prompt(
    opts: &FieldParams,
    value: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ident = opts.ident;
    if opts.multiple_once {
        quote! {
           #ident: {
                self[0].clone().#ident
            }
        }
    } else if let Some(f) = &opts.function_add {
        let f: TokenStream = f.parse().unwrap();
        if is_option(opts.ty) {
            quote! {
                #ident: #f?
            }
        } else {
            quote! {
             #ident: if let Some(v) = #f? {
                          v
                    } else {
                        return Ok(())
                    }
            }
        }
    } else {
        quote! {#ident: #value}
    }
}
