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
    let name_str = &global_params.name.to_string();
    let tuple = &global_params.tuple;
    let params_as_named_value = &global_params.params_as_named_value;
    let vec_name: TokenStream = format!("Vec{name}").parse().unwrap();
    let f_del = if let Some(f) = &global_params.function_del {
        let func_del: TokenStream = f.parse().unwrap();
        quote! {
            #func_del;
        }
    } else {
        quote! {}
    };

    let inspect_method = if cfg!(feature = "inspect") {
        quote! {
        fn inspect(&self) -> promptable::anyhow::Result<()> {
            let options = self
                .iter()
                .map(|e| promptable::display::PromptableDisplay::display_short(e))
                .collect::<Vec<String>>();
            loop {
            promptable::clear_screen();
                match inquire::Select::new("Choose the element to see.\nEscape to quit the view", options.clone()).raw_prompt() {
                    Ok(l) => self[l.index].inspect()?,
                    Err(inquire::InquireError::OperationCanceled) => break,
                    Err(e) => return Err(e.into()),
                }
            }
            promptable::clear_screen();
            Ok(())
        }
            }
    } else {
        quote! {}
    };

    quote! {
                // least bad solution ?
                #[derive(promptable::derive_more::Deref, promptable::derive_more::DerefMut, Clone, promptable::derive_more::Display)]
                #[display(fmt=#name_str)]
                pub struct #vec_name(pub Vec<#name>);

                        impl promptable::Promptable<(#tuple)> for #vec_name {
            fn new_by_prompt(params: (#tuple)) -> promptable::anyhow::Result<Option<#vec_name>> {
                if let Some(r) = #name::new_by_prompt(params)? {
                    promptable::clear_screen();
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
                    let value_name = #name_str;
                    let msg_menu = format!("{} {}:", self.len(), value_name);
                    if let Some(choix) = promptable::inquire::Select::new(&msg_menu, options_menu.to_vec()).prompt_skippable()? {
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
                    } else {
                             if promptable::menu::menu_cancel(&restore_self, self)? {
                              break;
                             }
                    }
                }
                    promptable::clear_screen();
                Ok(())
            }
                #inspect_method
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
            let choix = match promptable::inquire::MultiSelect::new(
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
                let element = &self[index];
                    #f_del
                self.remove(index);
            }
            promptable::clear_screen();
            Ok(())
                        }
        fn modify_by_prompt_vec(
            &mut self,
            params: (#tuple),
        ) -> promptable::anyhow::Result<()> {
             promptable::clear_screen();
            let choix = match promptable::inquire::Select::new(
                "Select object to modify",
                self.iter().map(|e| <#name as promptable::display::PromptableDisplay>::display_short(e)).collect(),
            )
            .raw_prompt() {
                    Ok(c) => c,
                    Err(promptable::inquire::error::InquireError::OperationCanceled) => return Ok(()),
                    Err(e) => Err(e)?
                };
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
                #ident: #f
            }
        } else {
            quote! {
             #ident: if let Some(v) = #f {
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
