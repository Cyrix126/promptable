#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use core::panic;
use darling::FromAttributes;
use display::{
    field_display_human_get, field_display_short_get, impl_prompt_display_generate, option2bool,
};
use impl_struct::{
    add_last_actions_menu_modify, generate_value_from_field_new, impl_promptable_struct,
    prepare_value_from_field_modify,
};
use impl_struct_vec::{generate_line_add_by_prompt, impl_promptable_vec_struct};
use params::{get_from_params, prepare_value_as_function_param};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{self, Attribute, Data, Type};

mod display;
mod impl_struct;
mod impl_struct_vec;
mod params;

#[derive(FromAttributes)]
#[darling(attributes(promptable))]
struct FieldOpts {
    short_display: Option<bool>,
    default: Option<bool>,
    name: Option<String>,
    visible: Option<bool>,
    function_render: Option<String>, // field_value mut be in parameter
    msg: Option<String>,
    function: Option<String>,
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
    ty: &'a Type,
    function: &'a Option<String>,
    function_new: &'a Option<String>,
    function_mod: &'a Option<String>,
    function_add: &'a Option<String>,
    multiple_once: bool,
    function_render: &'a Option<String>, // field_value mut be in parameter
    short_display: bool,
    default: bool,
}
#[derive(FromAttributes, Debug)]
#[darling(attributes(prompt))]
struct StructOpts {
    params: Option<String>,
    msg_mod: Option<String>,
    custom_prompt_display: Option<bool>,
    name: Option<String>,
}

struct GlobalParams {
    msg_mod: String,
    custom_prompt_display: bool,
    name: TokenStream,
    params_as_named_value: Vec<TokenStream>,
    tuple: TokenStream,
}

fn opts2global(ast: &syn::DeriveInput) -> GlobalParams {
    let attrs_struct = StructOpts::from_attributes(&ast.attrs).expect("Wrong attributes on struct");
    let custom_prompt_display = option2bool(attrs_struct.custom_prompt_display);
    let name = attrs_struct
        .name
        .unwrap_or_else(|| ast.ident.to_string())
        .parse()
        .unwrap();
    let msg_mod = attrs_struct
        .msg_mod
        .unwrap_or(String::from("Select the field to modify"));
    let params = &attrs_struct.params.unwrap_or_default();
    let tuple: TokenStream = get_from_params(params, false).parse().unwrap();
    let params_as_named_value = prepare_value_as_function_param(params);
    GlobalParams {
        msg_mod,
        custom_prompt_display,
        name,
        params_as_named_value,
        tuple,
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

fn prepare_value(opts: &FieldParams) -> proc_macro2::TokenStream {
    let name_field = opts.ident;
    if let Some(f) = &opts.function_render {
        let function: proc_macro2::TokenStream = f.parse().unwrap();
        quote! {
            let field_value = &self.#name_field;
            let value = #function;
        }
    } else if is_option(opts.ty) {
        quote! {
        let value =
        if let Some(v) = &self.#name_field {
            v.to_string()
        } else {
            String::from("None")
        };
        }
    } else {
        quote! {
            let value = &self.#name_field;
        }
    }
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
        function: &opts.function,
        visible,
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
    let mut field_values_new = vec![];

    // génération de modify_by_prompt fields
    // options for prompt
    let mut fields_options = vec![];
    // match and action
    let mut choix_action = vec![];

    let mut fields_multiple_add = vec![];

    let mut fields_display_short_precise = vec![];
    let mut fields_display_short = vec![];
    let mut fields_display_human = vec![];
    for (nb, (ident, ty, attrs)) in fields(&ast.data).into_iter().enumerate() {
        let opts: FieldOpts = FieldOpts::from_attributes(attrs).expect("Wrong options");
        let field_params = get_opts_field(nb, &opts, ident, ty);

        if !global_params.custom_prompt_display {
            field_display_short_get(
                &mut fields_display_short_precise,
                &mut fields_display_short,
                &field_params,
            );
            fields_display_human.push(field_display_human_get(&field_params));
        }

        // add the line for this field for new_by_prompt
        let value = generate_value_from_field_new(&field_params);
        field_values_new.push(quote! {
            #ident: #value
        });

        // add the lines for this field for modify_by_prompt
        prepare_value_from_field_modify(&field_params, &mut fields_options, &mut choix_action);

        // add line for this field for multiple add_by_prompt
        fields_multiple_add.push(generate_line_add_by_prompt(&field_params, &value));
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
        field_values_new,
        fields_options,
        choix_action,
        &global_params,
    );
    let impl_prompt_vec_struct = impl_promptable_vec_struct(fields_multiple_add, &global_params);

    let generation = quote! {
        #impl_prompt_display
        #impl_prompt_struct
        #impl_prompt_vec_struct
    };
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
