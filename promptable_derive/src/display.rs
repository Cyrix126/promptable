use crate::is_option;
use crate::prepare_value;
use crate::FieldParams;
use crate::GlobalParams;
use proc_macro2::TokenStream;
use quote::quote;
pub(crate) fn option2bool(o: Option<bool>) -> bool {
    o.unwrap_or(false)
}
pub(crate) fn field_display_short_get(
    fields_precise: &mut Vec<proc_macro2::TokenStream>,
    fields: &mut Vec<proc_macro2::TokenStream>,
    opts: &FieldParams,
) {
    if !fields_precise.is_empty() && !opts.short_display {
        return;
    } else {
        let line;
        let ident = opts.ident;
        if is_option(opts.ty) {
            line = quote! {
                if let Some(v) = &self.#ident {
                str.push(format!("{}", v));
                } else {
                str.push(format!("None"));
                }
            };
        } else {
            line = quote! {
            str.push(format!("{}", self.#ident));
            }
        }

        if opts.short_display {
            fields_precise.push(line)
        } else if fields.len() < 3 {
            fields.push(line)
        }
    }
}

pub(crate) fn field_display_short_fusion(
    mut fields_precise: Vec<proc_macro2::TokenStream>,
    fields: Vec<proc_macro2::TokenStream>,
) -> Vec<proc_macro2::TokenStream> {
    if fields_precise.len() <= 3 {
        fields_precise.extend(fields);
    }
    fields_precise.truncate(3);
    fields_precise
}
pub(crate) fn field_display_human_get(opts: &FieldParams) -> proc_macro2::TokenStream {
    let pre_value = prepare_value(opts);
    let field_name = &opts.name;
    quote! {
        #pre_value
        let field = #field_name;
        str.push(format!("{}: {}", field, value));
    }
}

pub(crate) fn impl_prompt_display_generate(
    fields_short_precise: Vec<TokenStream>,
    fields_short: Vec<TokenStream>,
    fields_human: Vec<TokenStream>,
    global_params: &GlobalParams,
) -> TokenStream {
    if !global_params.custom_prompt_display {
        let short_fields = field_display_short_fusion(fields_short_precise, fields_short);
        let name = &global_params.name;
        let vec_name: TokenStream = format!("Vec{}", name).parse().unwrap();
        quote! {
            impl promptable::display::PromptableDisplay for #name {
                fn display_short(&self) -> String {
                    let mut str = vec![];
                    #( #short_fields )*
                    str.join("\n")
                }
                fn display_human(&self) -> String {
                    let mut str = vec![];
                    #( #fields_human)*
                    str.join("\n")
                }
            }
            impl promptable::display::PromptableDisplay for #vec_name {
                fn display_short(&self) -> String {
                    self.0.display_short()
                }
                fn display_human(&self) -> String {
                  self.0.display_short()
                  }
            }
        }
    } else {
        quote! {}
    }
}
