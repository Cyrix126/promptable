use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{is_option, option_type, prepare_value, FieldParams, GlobalParams};
pub(crate) fn impl_promptable_struct(
    field_values_new: Vec<TokenStream>,
    fields_options: Vec<TokenStream>,
    choix_action: Vec<TokenStream>,
    global_params: &GlobalParams,
    idents_visible: &[(&Ident, bool)],
) -> proc_macro2::TokenStream {
    let tuple = &global_params.tuple;
    let name = &global_params.name;
    let params_as_named_value = &global_params.params_as_named_value;
    // let msg_mod = &global_params.msg_mod;
    let msg_mod = format!("Modification of {}", name);
    let msg_mod_pretty = format!("{}\n{:-<2$}", msg_mod, "-", (msg_mod.len() + 2));

    let method_inspect = generate_method_inspect(&fields_options, idents_visible);
    quote! {
                impl promptable::Promptable<(#tuple)> for #name {
                    fn new_by_prompt(params: (#tuple)) -> promptable::anyhow::Result<Option<#name>> {
                        promptable::clear_screen();
                        #( #params_as_named_value )*
                     return Ok(Some(#name {
                        #( #field_values_new ),*
                        }))

                    }
                     fn modify_by_prompt(&mut self, params: (#tuple)) -> promptable::anyhow::Result<()> {
                        #( #params_as_named_value )*
                         let self_restore = self.clone();
                         let mut last_choice = 0;
                         loop {
                         promptable::clear_screen();
                         let mut options = vec![];
                         #( #fields_options )*
        options.push(promptable::menu::MenuClassic::CANCEL.to_string());
        options.push(promptable::menu::MenuClassic::CONFIRM.to_string());

                         if let Some(choix) = promptable::inquire::Select::new(#msg_mod_pretty, options.clone()).with_starting_cursor(last_choice).prompt_skippable()? {
                         #( #choix_action)*
                         } else {
                             break
                         }
                         }
                Ok(())
                     }
        #method_inspect
    }
    }
}
pub(crate) fn add_last_actions_menu_modify(choix_action: &mut Vec<proc_macro2::TokenStream>) {
    choix_action.push(quote! {
        if &choix == promptable::menu::MenuClassic::CANCEL {
            if promptable::menu::menu_cancel(&self_restore, self)? {
                return Ok(())
            }
        }
    });
    choix_action.push(quote! {
        if &choix == options.last().unwrap() {
            return Ok(())
        }
    });
}

pub(crate) fn prepare_value_from_field_modify(
    opts: &FieldParams,
    fields_options: &mut Vec<proc_macro2::TokenStream>,
    choix_action: &mut Vec<proc_macro2::TokenStream>,
) {
    let name = &opts.name;
    let ident = opts.ident;
    let nb = opts.nb;
    let msg = &opts.msg;
    if opts.visible {
        // modify_by_prompt
        let prepare_value = prepare_value(opts); // préparer les choix

        fields_options.push(quote! {
            #prepare_value
            let name_field = #name;
            options.push(format!("{}: {}", name_field, value ));
        });

        // utiliser le choix
        // let nb_choice = nb.checked_sub(1).expect(&format!("{nb}"));
        let nb_choice = nb;
        if let Some(fm) = &opts.function_mod {
            let function_mod: proc_macro2::TokenStream = fm.parse().unwrap();
            choix_action.push(quote! {
                if choix == options[#nb_choice] {
                    last_choice = #nb_choice;
                    let field = &mut self.#ident;
                        promptable::clear_screen();
                    #function_mod;
                }
            });
        } else if is_option(opts.ty) {
            let inner = option_type(opts.ty).expect("could not find inner type of Option");
            choix_action.push(quote! {
                if choix == options[#nb_choice] {
                    last_choice = #nb_choice;
                    // if is option, the inner value must be passed.
                    let inner_value_origin = self.#ident.clone().unwrap_or_default();
                    let mut inner_value = inner_value_origin.clone();
                    // the modify prompt will always put Some(v) or do nothing.
                    // if the value is the same, do not change self.#ident.
                    // This way, it will preserve the None if there was one before the modify_by_prompt method.
                        promptable::clear_screen();
                    <#inner as promptable::Promptable<&str>>::modify_by_prompt(&mut inner_value, #msg)?;
                    if inner_value != inner_value_origin {
                        self.#ident = Some(inner_value)
                    }

                }
            });
        } else {
            let ty = opts.ty;
            choix_action.push(quote! {
                if choix == options[#nb_choice] {
                    last_choice = #nb_choice;
                        promptable::clear_screen();
                    <#ty as promptable::Promptable<&str>>::modify_by_prompt(&mut self.#ident, #msg)?
                }
            });
        }
    }
}

fn generate_method_inspect(
    fields_options: &Vec<TokenStream>,
    idents_visible: &[(&Ident, bool)],
) -> TokenStream {
    let mut lines_match_ident = Vec::new();

    for (n, (ident, option)) in idents_visible.iter().enumerate() {
        let field: TokenStream = ident.to_string().parse().unwrap();
        lines_match_ident.push(if *option {
            quote! {
                #n => {if let Some(v) = &self.#field {
                    v.inspect()?;
                } else {
                        continue
                    }

                }
            }
        } else {
            quote! {
                #n => promptable::Promptable::inspect(&self.#field)?
            }
        });
    }

    if cfg!(feature = "inspect") {
        quote! {
                fn inspect(&self) -> promptable::anyhow::Result<()> {

        // for structs, inspect must put human_description
        // add a menu to select any fields visible
        // the selected field will have his method inspect called.
                        let mut options = Vec::new();
                        // name of field
                            #( #fields_options)*
                        options.push("Go back".to_string());
                        loop {
                            promptable::clear_screen();
                            println!("{}", promptable::display::PromptableDisplay::display_human(self));
                            match inquire::Select::new("Select the field to view", options.clone()).raw_prompt() {
                    Ok(l) => {
                            match l.index {
                                #(#lines_match_ident),*,
                                _=> break
                            }
                        },
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
    }
}
