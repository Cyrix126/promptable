#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use core::panic;
use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Type};

#[derive(FromAttributes)]
#[darling(attributes(promptable))]
struct FieldOpts {
    default: Option<bool>,
    name: Option<String>,
    visible: Option<bool>,
    msg: Option<String>,
    function: Option<String>,
    function_new: Option<String>,
    function_mod: Option<String>,
    function_add: Option<String>, // self.ident peut-être utilisé
    multiple_once: Option<bool>,
}
#[derive(FromAttributes, Debug)]
#[darling(attributes(prompt))]
struct StructOpts {
    params: Option<String>,
    msg_mod: Option<String>,
}

/// This is the derive trait to add to struct you want to be prompt-able.
#[doc = include_str!("../doc/attributs_derive.md")]
#[proc_macro_derive(Promptable, attributes(promptable, prompt))]
pub fn promptable_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_promptable(&ast)
}

fn impl_promptable(ast: &syn::DeriveInput) -> TokenStream {
    let attrs_struct = StructOpts::from_attributes(&ast.attrs).expect("Wrong attributes on struct");
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
    let mut field_values_new = vec![];
    let mut fields_options = vec![];
    let mut choix_action = vec![];
    let mut fields_multiple_new = vec![];
    let mut fields_multiple_add = vec![];
    let mut nb: usize = 0;
    for (ident, ty, attrs) in fields.iter() {
        let opts: FieldOpts = FieldOpts::from_attributes(attrs).expect("Wrong options");
        // si champ est unique dans multiple, le créer pour multiple_new_by_prompt

        // paniquer si certains attributs non compatibles sont utilisés en même temps.

        if opts.default.is_some() && opts.msg.is_some() {
            panic!("default et msg attribut en même n'a pas de sens.")
        }
        if opts.function_new.is_some() && opts.msg.is_some() {
            panic!("function et msg attribut en même n'a pas de sens.")
        }
        if opts.function_new.is_some() && attrs_struct.params.is_none() {
            panic!("function ne peux pas être utilisé si l'attribut params du struct n'est pas définit.{:?}\n utilisez deux guillements sans contenu si aucunb paramètres n'est nécéssaire", attrs_struct)
        }

        // récupérer nom field
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
        // tous les types de champs doivent implémenter Promptable ou au moins Default (il ne doit pas alors être demandé) si l'attribut fonction n'est pas renseigné.

        let value = if let Some(f) = opts.function_new {
            f.parse().unwrap()
        } else if let Some(ref f) = opts.function {
            f.parse().unwrap()
        } else if opts.default.is_some_and(|c| c) {
            if is_option(ty) {
                quote! {None}
            } else {
                quote! {
                    #ty::default()
                }
            }
        } else if visible && !is_option(ty) {
            quote! {
                <#ty as promptable::Promptable>::new_by_prompt(#msg)
            }
        } else if visible && is_option(ty) {
            let inner = option_type(ty).unwrap();
            quote! {
            Some(<#inner as promptable::Promptable>::new_by_prompt(#msg))
            }
        } else if !visible && is_option(ty) {
            quote! {
                None
            }
        } else if !visible && !is_option(ty) {
            quote! {
                #ty::default()
            }
        } else {
            panic!("attribut non attendu")
        };

        field_values_new.push(quote! {
            #ident: #value
        });

        let value_add = if let Some(f) = opts.function_add {
            let f: proc_macro2::TokenStream = f.parse().unwrap();
            quote! {
                #ident: #f
            }
        } else {
            quote! {#ident: #value}
        };

        if visible {
            // modify_by_prompt
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
            // préparer les choix
            fields_options.push(quote! {
                #prepare_value
                options.push(format!("{}: {}", #name, value ))
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
            } else {
                choix_action.push(quote! {
                    if choix == options[#nb_choice] {
                        last_choice = #nb_choice;
                        <#ty as promptable::Promptable>::modify_by_prompt(&mut self.#ident, #msg)

                    }
                });
            }
            nb += 1;
        }

        // ajout sur multiple
        // field once = field

        // ajout: demande en regardant les valeurs du premier. Ne peux pas être executé sur le vec vide.

        // premier ajout
        // value sur ident
        // new

        fields_multiple_new.push(quote! {
            #ident: #value
        });

        // multiple, add

        if opts.multiple_once.is_some_and(|c| c) {
            fields_multiple_add.push(quote! {
               #ident: self[0].#ident.clone()
            })
        } else {
            fields_multiple_add.push(value_add);
        }
        // multiple mod
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

    // paramètres nécéssaires pour utiliser les fonctions à mettre dans la signature de la méthode

    let params = if let Some(p) = &attrs_struct.params {
        p.parse().unwrap()
    } else {
        quote!()
    };

    let mut literal_params = vec![];
    if let Some(pa) = &attrs_struct.params {
        let params = pa.split(", ");
        for p in params {
            if let Some(p) = p.split_once(": ") {
                literal_params.push(p.0)
            }
        }
    }

    let mut literal_params_separated = String::new();
    for param in literal_params {
        literal_params_separated.push_str(param);
        literal_params_separated.push_str(", ");
    }
    let mut chars = literal_params_separated.chars();
    chars.next_back();
    chars.next_back();
    let lps = chars.as_str();
    let lps: proc_macro2::TokenStream = lps.parse().unwrap();
    let msg_mod = if let Some(msg) = &attrs_struct.msg_mod {
        msg
    } else {
        "Select the field to modify"
    };

    let trait_name: proc_macro2::TokenStream = format!("Promptable{}", nom).parse().unwrap();
    let trait_name_multiple: proc_macro2::TokenStream =
        format!("PromptableVec{}", nom).parse().unwrap();
    let generation = quote! {
        pub trait #trait_name {
                fn new_by_prompt(#params) -> #nom;
               fn modify_by_prompt(&mut self, #params);
        }
            impl #trait_name for #nom {
                fn new_by_prompt(#params) -> #nom {
                    promptable::clear_screen();
                    #nom {
                    #( #field_values_new ),*
                    }
                }
                fn modify_by_prompt(&mut self, #params) {
                    let mut last_choice = 0;
                    loop {
                    promptable::clear_screen();
                    let mut options = vec![];
                    #( #fields_options );*;
                    let choix = inquire::Select::new(#msg_mod, options.clone()).with_starting_cursor(last_choice).prompt().unwrap();
                    #( #choix_action)*
                    }

                }
             }

       pub trait #trait_name_multiple {
            fn new() -> Vec<#nom> {
                Vec::new()
            }
            fn add(&mut self, #params);
            fn delete(&mut self);
            fn modify(&mut self, #params);
            fn multiple_by_prompt(&mut self, #params);
        }
            impl #trait_name_multiple for Vec<#nom> {
            fn add(&mut self, #params) {
                    promptable::clear_screen();
             self.push(#nom {
                    #( #fields_multiple_add ),*
                });
            }
            fn delete(&mut self) {
                    promptable::clear_screen();
                let choix = inquire::Select::new("Sélection de l'objet à supprimer", self.clone()).raw_prompt().unwrap();
                self.remove(choix.index);
            }
            fn modify(&mut self, #params) {
                    promptable::clear_screen();
                let choix = inquire::Select::new("Sélection de l'objet à modifier", self.clone()).raw_prompt().unwrap();
                self.remove(choix.index);
                let mut to_mod = choix.value;
                to_mod.modify_by_prompt(#lps);
                self.push(to_mod);
            }

             fn multiple_by_prompt(&mut self, #params) {
                    promptable::clear_screen();
                    // passer dans une loop menu
                let choix1 = "ajout".to_string();
                let choix2 = "modifier".to_string();
                let choix3 = "supprimer".to_string();
                let choix4 = "valider".to_string();
                let mut options = vec![];
                options.push(choix1.clone());
                options.push(choix2.clone());
                options.push(choix3.clone());
                options.push(choix4.clone());
    loop {
                if self.is_empty() {
                    self.push(#nom::new_by_prompt(#lps));
                }
                    promptable::clear_screen();
                    let choix = inquire::Select::new(#msg_mod, options.clone()).prompt().unwrap();
                    if choix == choix1 {
                        self.add(#lps)
                    } else
                    if choix == choix2 {
                        self.modify(#lps)
                    } else
                    if choix == choix3 {
                        self.delete()
                    } else
                    if choix == choix4 {
                        break                    }

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
