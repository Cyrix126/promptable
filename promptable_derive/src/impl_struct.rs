use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    is_option, option_type, prepare_value, FieldParams, GlobalParams, PATH_ANYHOW_TRAIT,
    PATH_CLEARSCREEN, PATH_INQUIRE, PATH_MENU, PATH_PROMPTABLE_TRAIT,
};
pub(crate) fn impl_promptable_struct(
    prepare_values_fields_new: &Vec<TokenStream>,
    fields_struct: &Vec<TokenStream>,
    fields_options: &Vec<TokenStream>,
    choix_action: &Vec<TokenStream>,
    global_params: &GlobalParams,
) -> proc_macro2::TokenStream {
    let tuple = &global_params.tuple;
    let name = &global_params.name;
    let params_as_named_value = &global_params.params_as_named_value;
    // let msg_mod = &global_params.msg_mod;
    let msg_mod = format!("Modification of {}", name);
    let msg_mod_pretty = format!("{}\n{:-<2$}", msg_mod, "-", (msg_mod.len() + 2));
    let path_promptable: TokenStream = PATH_PROMPTABLE_TRAIT.parse().unwrap();
    let path_anyhow: TokenStream = PATH_ANYHOW_TRAIT.parse().unwrap();
    let clear_screen: TokenStream = PATH_CLEARSCREEN.parse().unwrap();
    let path_inquire: TokenStream = PATH_INQUIRE.parse().unwrap();
    let path_menu: TokenStream = PATH_MENU.parse().unwrap();
    quote! {
                impl #path_promptable<(#tuple)> for #name {
                    fn new_by_prompt(params: (#tuple)) -> #path_anyhow::Result<Option<#name>> {
                        #clear_screen;
                // value from params
                        #( #params_as_named_value )*
                // value from prompt, one by one so that value after can take already made value.
                        #( #prepare_values_fields_new )*

                     return Ok(Some(#name {
                        #( #fields_struct ),*
                        }))

                    }
                     fn modify_by_prompt(&mut self, params: (#tuple)) -> #path_anyhow::Result<bool> {
                        #( #params_as_named_value )*
                         let self_restore = self.clone();
                         let mut last_choice = 0;
                         loop {
                         #clear_screen;
                         let mut options = vec![];
                         #( #fields_options )*
        options.push(#path_menu::MenuClassic::CANCEL.to_string());
        options.push(#path_menu::MenuClassic::CONFIRM.to_string());

                         if let Some(choix) = #path_inquire::Select::new(#msg_mod_pretty, options.clone()).with_starting_cursor(last_choice).prompt_skippable()? {
                         #( #choix_action)*
                         } else {
                             break
                         }
                         }

                Ok(false)
                     }
    }
    }
}
pub(crate) fn add_last_actions_menu_modify(choix_action: &mut Vec<TokenStream>) {
    let path_menu: TokenStream = PATH_MENU.parse().unwrap();
    choix_action.push(quote! {
        if &choix == #path_menu::MenuClassic::CANCEL {
            if #path_menu::menu_cancel(&self_restore, self)? {
                return Ok(false)
            }
        }
    });
    choix_action.push(quote! {
        if &choix == options.last().unwrap() {
            return Ok(true)
        }
    });
}

pub(crate) fn prepare_value_from_field_modify(
    opts: &FieldParams,
    fields_options: &mut Vec<TokenStream>,
    choix_action: &mut Vec<TokenStream>,
) {
    let clear_screen: TokenStream = PATH_CLEARSCREEN.parse().unwrap();
    let path_promptable: TokenStream = PATH_PROMPTABLE_TRAIT.parse().unwrap();
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
            let function_mod: TokenStream = fm.parse().unwrap();
            choix_action.push(quote! {
                if choix == options[#nb_choice] {
                    last_choice = #nb_choice;
                    let field = &mut self.#ident;
                        #clear_screen;
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
                        #clear_screen;
                    <#inner as #path_promptable<&str>>::modify_by_prompt(&mut inner_value, #msg)?;
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
                        #clear_screen;
                    <#ty as #path_promptable<&str>>::modify_by_prompt(&mut self.#ident, #msg)?;
                }
            });
        }
    }
}
