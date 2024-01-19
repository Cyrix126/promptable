use proc_macro2::TokenStream;
use quote::quote;

use crate::{is_option, option_type, prepare_value, FieldParams, GlobalParams};
pub(crate) fn impl_promptable_struct(
    field_values_new: Vec<TokenStream>,
    fields_options: Vec<TokenStream>,
    choix_action: Vec<TokenStream>,
    global_params: &GlobalParams,
) -> proc_macro2::TokenStream {
    let tuple = &global_params.tuple;
    let name = &global_params.name;
    let params_as_named_value = &global_params.params_as_named_value;
    let msg_mod = &global_params.msg_mod;
    quote! {
                impl promptable::Promptable<(#tuple)> for #name {
                    fn new_by_prompt(params: (#tuple)) -> promptable::anyhow::Result<Option<#name>> {
                        promptable::clear_screen();
                        #( #params_as_named_value )*
                    loop {
                     return Ok(Some(#name {
                        #( #field_values_new ),*
                        }))

                    }
                // Ok(None)
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

                         if let Some(choix) = inquire::Select::new(#msg_mod, options.clone()).with_starting_cursor(last_choice).prompt_skippable()? {
                         #( #choix_action)*
                         } else {
                             break
                         }
                         }
                Ok(())
                     }

    }
    }
}
pub(crate) fn generate_value_from_field_new(opts: &FieldParams) -> proc_macro2::TokenStream {
    let msg = &opts.msg;
    let ty = opts.ty;
    if let Some(f) = &opts.function_new {
        f.parse().unwrap()
    } else if let Some(ref f) = opts.function {
        f.parse().unwrap()
    } else if opts.default {
        if is_option(ty) {
            quote! {None}
        } else {
            quote! {
                #ty::default()
            }
        }
    } else if opts.visible && !is_option(ty) {
        quote! {
        loop {
            if let Some(prompt) = <#ty as promptable::Promptable<&str>>::new_by_prompt(#msg)?
             {
            break prompt
                }
            }
        }
    } else if opts.visible && is_option(ty) {
        let inner = option_type(ty).expect("could not find inner type of Option");
        quote! {
                <#inner as promptable::Promptable<&str>>::new_by_prompt(#msg)?
        }
    } else if !opts.visible && is_option(ty) {
        quote! {
            None
        }
    } else if !opts.visible && !is_option(ty) {
        quote! {
            #ty::default()
        }
    } else {
        panic!("attribut non attendu")
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
                    self.#ident = #function_mod
                }
            });
        } else if let Some(fm) = &opts.function {
            let function_mod: proc_macro2::TokenStream = fm.parse().unwrap();
            choix_action.push(quote! {
                if choix == options[#nb_choice] {
                    last_choice = #nb_choice;
                    self.#ident = #function_mod
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
                    <#ty as promptable::Promptable<&str>>::modify_by_prompt(&mut self.#ident, #msg)?
                }
            });
        }
    }
}
