use core::panic;

use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Type};

#[derive(FromAttributes)]
#[darling(attributes(promptable))]
struct Opts {
    default: Option<bool>,
    name: Option<String>,
    visible: Option<bool>,
    msg: Option<String>,
}

#[proc_macro_derive(Promptable, attributes(promptable))]
pub fn promptable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_promptable(&ast)
}

fn impl_promptable(ast: &syn::DeriveInput) -> TokenStream {
    let nom = &ast.ident;
    let data = &ast.data;
    let data_struct = match data {
        Data::Struct(s) => s,
        _ => panic!("promptable_macro_derive ne peut que être utilisé sur des structs"),
    };

    let fields = data_struct
        .fields
        .iter()
        .map(|field| (field.ident.as_ref().unwrap(), &field.ty, &field.attrs))
        .collect::<Vec<_>>();
    let mut field_values = vec![];
    let mut fields_options = vec![];
    let mut choix_action = vec![];
    for (nb, (ident, ty, attrs)) in fields.iter().enumerate() {
        let opts = Opts::from_attributes(&attrs).expect("Wrong options");
        // récupérer attribut du champ

        // récupérer nom field
        let name = if let Some(n) = &opts.name {
            n.to_owned()
        } else {
            ident.to_string()
        };
        let msg = if let Some(msg) = &opts.msg {
            msg.to_string()
        } else {
            format!("Renseignez la valeur de {}:", name)
        };
        let visible = if let Some(v) = opts.visible {
            v
        } else {
            if is_option(ty) {
                false
            } else {
                true
            }
        };
        if let Some(default) = opts.default {
            if default {
                field_values.push(quote! {
                    #ident: #ty::default()
                });
            }
        } else if visible && !is_option(ty) {
            field_values.push(quote! {
                #ident: #ty::new_by_prompt(#msg)
            });
        } else if visible && is_option(ty) {
            let inner = option_type(&ty).unwrap();
            field_values.push(quote! {
            #ident: Some(#inner::new_by_prompt(#msg))
                            })
        } else {
            field_values.push(quote! {
                #ident: None
            });
        }

        let prepare_value = if is_option(ty) {
            quote! {
            let value =
            if let Some(v) = &self.#ident {
                v.to_string()
            } else {
                String::from("None")
            };
            }
        } else {
            quote! {
                let value = &self.#ident;
            }
        };
        // pour chaque champ, créer un prompt

        // dans les options du menu, ajouter des valeurs qui seront des strings
        fields_options.push(quote! {
            #prepare_value
            options.push(format!("{}: {}", #name, value ))
        });
        choix_action.push(quote! {
            if choix == options[#nb] {
                last_choice = #nb;
                self.#ident.modify_by_prompt(#msg)
            }
        });
        // #ident {:#?}", p.longueur
    }
    fields_options.push(quote! {
        options.push(format!("Valider"))
    });
    choix_action.push(quote! {
        if &choix == options.last().unwrap() {
            return
        }
    });
    // récupérer les champs

    let generation = quote! {
        impl promptable::Promptable for #nom {
            fn new_by_prompt(_msg: &str) -> #nom {
                #nom {
                    // effectuer new_prompt à tout champs non Option. Mettre null aux champs Option.
                #( #field_values ),*
                }
            }
            fn modify_by_prompt(&mut self, msg: &str) {
                let mut last_choice = 0;
                loop {
                promptable::clear_screen();
                let mut options = vec![];
                #( #fields_options );*;
                let choix = inquire::Select::new(msg, options.clone()).with_starting_cursor(last_choice).prompt().unwrap();
                #( #choix_action)*
                }

            }
        }
    };
    generation.into()
}

// fn get_value_attribut(attrs: &Vec<Attribute>, attribut_name: &str, replace: String) -> String {
//     let attr = attrs
//         .iter()
//         .filter(|x| x.path().is_ident(attribut_name))
//         .collect::<Vec<_>>();

//     if !attr.is_empty() {
//         match attr[0].parse_args().unwrap() {
//             syn::Meta::NameValue(syn::MetaNameValue { path: _, value, .. }) => match value {
//                 Expr::Lit(v) => v.lit.suffix().to_string(),
//                 _ => panic!("pas de valeur litéral ?"),
//             },
//             _ => panic!("malformed attribute syntax"),
//         }
//     } else {
//         replace
//     }
// }

fn is_option(ty: &Type) -> bool {
    if let syn::Type::Path(ref typath) = ty {
        if typath.qself.is_none() && typath.path.segments[0].ident == "Option" {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn option_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(ty) = ty else { return None };
    if ty.qself.is_some() {
        return None;
    }

    let ty = &ty.path;

    if ty.segments.is_empty() || ty.segments.last().unwrap().ident.to_string() != "Option" {
        return None;
    }

    if !(ty.segments.len() == 1
        || (ty.segments.len() == 3
            && ["core", "std"].contains(&ty.segments[0].ident.to_string().as_str())
            && ty.segments[1].ident.to_string() == "option"))
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
