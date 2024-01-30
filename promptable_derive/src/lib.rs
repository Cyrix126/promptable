#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use crate::impl_inspect::impl_inspectable_struct;
use core::panic;
use darling::FromAttributes;
use display::{
    field_display_human_get, field_display_short_get, impl_prompt_display_generate, option2bool,
};
use impl_struct::{
    add_last_actions_menu_modify, impl_promptable_struct, prepare_value_from_field_modify,
};
use impl_struct_vec::{generate_values_add_by_prompt, impl_promptable_vec_struct};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote_tool_params::{get_from_params, prepare_values_from_params};
use syn::{self, Attribute, Data, Type};

mod display;
#[cfg(feature = "inspect")]
mod impl_inspect;
mod impl_struct;
mod impl_struct_vec;

const PATH_PROMPTABLEDISPLAY_TRAIT: &str = "promptable::basics::display::PromptableDisplay";
const PATH_PROMPTABLE_TRAIT: &str = "promptable::basics::promptable::Promptable";
const PATH_ANYHOW_TRAIT: &str = "promptable::anyhow";
const PATH_CLEARSCREEN: &str = "promptable::basics::display::clear_screen()";
const PATH_INQUIRE: &str = "promptable::inquire";
#[cfg(feature = "inspect")]
const PATH_INSPECT: &str = "promptable::inspect::Inspectable";
const PATH_MENU: &str = "promptable::basics::menu";
const PATH_DERIVE_MORE: &str = "promptable::derive_more";
#[derive(FromAttributes)]
#[darling(attributes(promptable))]
struct FieldOpts {
    short_display: Option<bool>,
    default: Option<bool>,
    name: Option<String>,
    visible: Option<bool>,
    #[cfg(feature = "inspect")]
    inspect: Option<bool>,
    function_render: Option<String>, // field_value mut be in parameter
    msg: Option<String>,
    function_new: Option<String>,
    function_mod: Option<String>,
    function_add: Option<String>, // self.ident peut-être utilisé
    multiple_once: Option<bool>,
}

struct FieldParams<'a> {
    nb: usize,
    msg: String,
    name: String,
    ident: &'a Ident,
    visible: bool,
    #[cfg(feature = "inspect")]
    inspect: bool,
    ty: &'a Type,
    function_new: &'a Option<String>,
    function_mod: &'a Option<String>,
    function_add: &'a Option<String>,
    function_render: &'a Option<String>, // field_value mut be in parameter
    multiple_once: bool,
    short_display: bool,
    default: bool,
}
#[derive(FromAttributes, Debug)]
#[darling(attributes(prompt))]
struct StructOpts {
    params: Option<String>,
    custom_prompt_display: Option<bool>,
    name: Option<String>,
    function_del: Option<String>,
}

struct GlobalParams {
    custom_prompt_display: bool,
    name: TokenStream,
    params_as_named_value: Vec<TokenStream>,
    tuple: TokenStream,
    function_del: Option<String>, // can acess element on deletion of element in vec.
}

fn opts2global(ast: &syn::DeriveInput) -> GlobalParams {
    let attrs_struct = StructOpts::from_attributes(&ast.attrs).expect("Wrong attributes on struct");
    let custom_prompt_display = option2bool(attrs_struct.custom_prompt_display);
    let name = attrs_struct
        .name
        .unwrap_or_else(|| ast.ident.to_string())
        .parse()
        .unwrap();
    let params = &attrs_struct.params.unwrap_or_default();
    let tuple: TokenStream = get_from_params(params, false).parse().unwrap();
    let params_as_named_value = prepare_values_from_params(params, "params");
    GlobalParams {
        custom_prompt_display,
        name,
        params_as_named_value,
        tuple,
        function_del: attrs_struct.function_del,
    }
}

/// This is the derive trait to add to struct you want to be prompt-able.
#[doc = include_str!("../doc/attributs_derive.md")]
#[proc_macro_derive(Promptable, attributes(promptable, prompt))]
pub fn promptable_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_promptable(&ast)
}

fn fields(data: &Data) -> Vec<(&Ident, &Type, &Vec<Attribute>)> {
    let data_struct = match data {
        Data::Struct(s) => s,
        _ => panic!("promptable_macro_derive ne peut que être utilisé sur des structs"),
    };

    data_struct
        .fields
        .iter()
        .map(|field| (field.ident.as_ref().unwrap(), &field.ty, &field.attrs))
        .collect::<Vec<_>>()
}

fn get_opts_field<'a>(
    nb: usize,
    opts: &'a FieldOpts,
    ident: &'a Ident,
    ty: &'a Type,
) -> FieldParams<'a> {
    let name = if let Some(n) = &opts.name {
        n.to_owned()
    } else {
        ident.to_string()
    };
    let msg = if let Some(msg) = &opts.msg {
        msg.to_string()
    } else {
        format!("Insert value of {}:", name)
    };

    let visible = if let Some(v) = opts.visible { v } else { true };
    FieldParams {
        nb,
        msg,
        name,
        ident,
        ty,
        visible,
        #[cfg(feature = "inspect")]
        inspect: opts.inspect.unwrap_or(true),
        function_new: &opts.function_new,
        function_mod: &opts.function_mod,
        function_add: &opts.function_add,
        function_render: &opts.function_render,
        short_display: opts.short_display.unwrap_or_default(),
        default: opts.default.unwrap_or_default(),
        multiple_once: opts.multiple_once.unwrap_or_default(),
    }
}

fn impl_promptable(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let global_params = opts2global(ast);

    // génération de new_by_prompt struct fields
    let mut fields_struct = vec![];

    // génération de modify_by_prompt fields
    // options for prompt
    let mut fields_options = vec![];
    #[cfg(feature = "inspect")]
    let mut fields_options_inspect = vec![];
    // match and action
    let mut choix_action = vec![];

    let mut fields_display_short_precise = vec![];
    let mut fields_display_short = None;
    let mut fields_display_human = vec![];
    let mut prepare_values_fields_new = vec![];
    let mut prepare_values_fields_add = vec![];
    #[cfg(feature = "inspect")]
    let mut idents_inspect = vec![];

    for (nb, (ident, ty, attrs)) in fields(&ast.data).into_iter().enumerate() {
        let opts: FieldOpts = FieldOpts::from_attributes(attrs).expect("Wrong options");
        let field_params = get_opts_field(nb, &opts, ident, ty);
        #[cfg(feature = "inspect")]
        if field_params.visible && field_params.inspect {
            prepare_value_from_field_modify(
                &field_params,
                &mut fields_options_inspect,
                &mut vec![],
            );
            #[cfg(feature = "inspect")]
            idents_inspect.push((ident, ty))
        }
        if !global_params.custom_prompt_display {
            field_display_short_get(
                &mut fields_display_short_precise,
                &mut fields_display_short,
                &field_params,
            );
            fields_display_human.push(field_display_human_get(&field_params));
        }

        // add the line for this field for new_by_prompt
        let value_new = generate_value_from_field(&field_params, true);
        let value_add = generate_values_add_by_prompt(
            &field_params,
            &generate_value_from_field(&field_params, false),
        );
        prepare_values_fields_new.push(quote! {
           let #ident = #value_new;
        });
        prepare_values_fields_add.push(quote! {
           let #ident = #value_add;
        });
        fields_struct.push(quote! {
            #ident
        });

        // add the lines for this field for modify_by_prompt
        prepare_value_from_field_modify(&field_params, &mut fields_options, &mut choix_action);

        // add line for this field for multiple add_by_prompt
        // multiple mod
    }
    add_last_actions_menu_modify(&mut choix_action);

    // paramètres nécéssaires pour utiliser les fonctions à mettre dans la signature de la méthode

    // display fields
    let impl_prompt_display = impl_prompt_display_generate(
        fields_display_short_precise,
        fields_display_short,
        fields_display_human,
        &global_params,
    );
    let impl_prompt_struct = impl_promptable_struct(
        &prepare_values_fields_new,
        &fields_struct,
        &fields_options,
        &choix_action,
        &global_params,
    );
    let impl_prompt_vec_struct =
        impl_promptable_vec_struct(&prepare_values_fields_add, &fields_struct, &global_params);

    let mut generation = quote! {
        #impl_prompt_display
        #impl_prompt_struct
        #impl_prompt_vec_struct
    };
    #[cfg(feature = "inspect")]
    generation.extend(impl_inspectable_struct(
        &fields_options_inspect,
        &global_params,
        &idents_inspect,
    ));
    generation.into()
}

#[doc(hidden)]
fn is_option(ty: &Type) -> bool {
    if let syn::Type::Path(ref typath) = ty {
        typath.qself.is_none() && typath.path.segments[0].ident == "Option"
    } else {
        false
    }
}

#[doc(hidden)]
fn option_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(ty) = ty else { return None };
    if ty.qself.is_some() {
        return None;
    }

    let ty = &ty.path;

    if ty.segments.is_empty() || ty.segments.last().unwrap().ident != "Option" {
        return None;
    }

    if !(ty.segments.len() == 1
        || (ty.segments.len() == 3
            && ["core", "std"].contains(&ty.segments[0].ident.to_string().as_str())
            && ty.segments[1].ident == "option"))
    {
        return None;
    }

    let last_segment = ty.segments.last().unwrap();
    let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None;
    };
    if generics.args.len() != 1 {
        return None;
    }
    let syn::GenericArgument::Type(inner_type) = &generics.args[0] else {
        return None;
    };

    Some(inner_type)
}
fn prepare_value(opts: &FieldParams) -> TokenStream {
    let name_field = opts.ident;
    let path: TokenStream = PATH_PROMPTABLEDISPLAY_TRAIT.parse().unwrap();
    if let Some(f) = &opts.function_render {
        let function: TokenStream = f.parse().unwrap();
        quote! {
            let field_value = &self.#name_field;
            let value = #function;
        }
    } else if is_option(opts.ty) {
        quote! {
        let value =
        if let Some(v) = &self.#name_field {
            format!("{}", #path::display_short(v))
        } else {
            String::from("None")
        };
        }
    } else {
        quote! {
            let value = #path::display_short(&self.#name_field);
        }
    }
}

fn generate_value_from_field(opts: &FieldParams, new_or_add: bool) -> TokenStream {
    let msg = &opts.msg;
    let ty = opts.ty;
    let path: TokenStream = PATH_PROMPTABLE_TRAIT.parse().unwrap();
    let clear_screen: TokenStream = PATH_CLEARSCREEN.parse().unwrap();
    let cancel_value = if new_or_add {
        quote! {return Ok(None)}
    } else {
        quote! {return Ok(false)}
    };

    // if function is present, visible and default should not be used for this field.
    // if function or method, propagate errors with ?
    // if !is_option and !default, extract value from Some or return Cancel (Ok(None)) instead of Ok(Some(T))

    // execute functions, propagating errors, stop if function return None when a value was needed.

    if let Some(f) = opts.function_new {
        let func: TokenStream = f.parse().unwrap();
        if is_option(ty) {
            return quote! {
                #func
            };
        } else {
            return quote! {
                if let Some(v) = #func {
                    v
                } else {
                    #cancel_value
                 }
            };
        }
    }

    // if not function, see if visible and default value

    if opts.default {
        if is_option(ty) {
            quote! {None}
        } else {
            quote! {
                #ty::default()
            }
        }
    } else if opts.visible && !is_option(ty) {
        quote! {
        if let Some(prompt) =  {
                #clear_screen;
             <#ty as #path<&str>>::new_by_prompt(#msg)?
            }
         {
        prompt
            } else {
                #cancel_value
        }
        }
    } else if opts.visible && is_option(ty) {
        let inner = option_type(ty).expect("could not find inner type of Option");
        // transfer the Option<T> directly
        quote! {
            {
                #clear_screen;
                println!("Escape to put an empty value");
                <#inner as #path<&str>>::new_by_prompt(#msg)?
            }
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
