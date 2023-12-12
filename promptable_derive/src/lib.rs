use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data};

#[proc_macro_derive(Promptable)]
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
        .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
        .collect::<Vec<_>>();
    let mut field_values = vec![];
    for (ident, ty) in fields {
        // si type est un option, le mettre à null pour ne pas le demander à l'utilisateur puisque ce n'est pas un champ obligatoire
        if let syn::Type::Path(ref typath) = ty {
            if typath.qself.is_none() && typath.path.segments[0].ident == "Option" {
                field_values.push(quote! {
                    #ident: None
                })
            }
        }
        // pour chaque champ, créer un prompt
        field_values.push(quote! {
            #ident: #ty::new_prompt()
        });
    }
    // récupérer les champs

    let generation = quote! {
        impl Promptable for #nom {
            fn new_prompt() -> #nom {
                #nom {
                    // effectuer new_prompt à tout champs non Option. Mettre null aux champs Option.
                #( #field_values ),*
                }
            }
        }
    };
    generation.into()
}
