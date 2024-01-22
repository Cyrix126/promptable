use proc_macro2::TokenStream;
use quote::quote;

// params comes from the attributs params.
// it is put in this way: name_param: type_a, name_param2: type_b
// it needs to be converted in one tuple: "(type_a, type_b)" for the type in the implementation.
// it also need to be converted to "params: (type_a, type_b)" for the parameter of the methods.
// then in functions retrieved, name_param need to be converted to params.nb, nb being the index corresponding.
// Because of this technical way, at least one parameter must be given by the user when he calls the method, even an empty tuple like this method(()).
// this change is cumbersome for the dev user, but the code will be clearer and no trait will be created by the macro. So the struct will implement the trait Promptable and not a magical out of nowhere new trait.
// because no trait are created, a new type for Vec<T> needs to be created. So instead of typing:
// <Vec<T> as PromptableVecT>::new(),
// The following will be needed:
/// VecT::new_by_prompt(())
/// With no traits created, no need to manage pub for traits and be sure the dev use understand where it is.
// but a wrapper for Vec<T> still needs to exist.
// the issue with generics is that an implementation of Promtable<A> is not the same as Promptable<B>. So Two structs with derive macro Promptable do not share the same trait and a followup of for what type is implemented may add complexity.
// The first advantage would be:

// output in

// for method param
// "params: (type, type)"
// impl<T>
// (type, type)
// for retrieve in method with functions given
// position of param_name. Name of parameter of the function
// for vec, pass to method of simple
// pass directly the params ?

// params_name.iter().position(param_name).unwrap()

pub(crate) fn prepare_value_as_function_param(params: &str) -> Vec<TokenStream> {
    let mut token = Vec::new();
    let params_name = get_from_params(params, true);
    let split = params_name.split(',');
    let unit = split.clone().collect::<Vec<&str>>().len() <= 1;
    for (index, name) in split.enumerate() {
        if !name.is_empty() {
            let name: proc_macro2::TokenStream = name.parse().unwrap();
            let index = syn::Index::from(index);
            if unit {
                token.push(quote! {
                    let #name = params;
                })
            } else {
                token.push(quote! {
                    let #name = params.#index;
                })
            }
        }
    }
    token
}

// get a string of name of params or type of params dilimeted by comma.
pub(crate) fn get_from_params(params: &str, name_or_type: bool) -> String {
    let mut literal = Vec::new();
    for p in params.split(", ") {
        // p is name_param: type
        if let Some(p) = p.split_once(": ") {
            if name_or_type {
                literal.push(p.0.trim())
            } else {
                literal.push(p.1.trim())
            }
        }
    }
    literal.join(",")
}
