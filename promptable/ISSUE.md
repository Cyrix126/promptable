# ISSUES


## Traits created only for one type.

For different parameters be passed through this declarative macro, a trait (actually two, one to be implemented on the struct and another for Vec<T>) need to be made for each struct. I'm not sure that's a good way of coding.

## Low Debugging Output

The dev user using the declarative macro could get errors if for example a field doesn't implement Display or if the function he passed as params attribut get consumed by a function he also passed if this function get called twice or more.

cargo expand can be used to see the result of the macro.

See how derive_more arrives to get debugging with strings.


## No analyze of code from LSP server on strings

Because the attributs takes String as argument, rust-analyze can't process them for correctness and autocompletion.

So for example if a params takes a parameter which is not in the scope, it will just complain but do not offer the solution to bring the type in the scope.

Same to verify and complete the name of function passed through.


## inquire Custom Type has low help for end user.

The help output for the CustomType implemented for basic types is not very helpfull for the end user. Maybe an implementation more precise for types should be better (for example with a validator for number).

A new attribut for helper message could be also done.

## Limit the power of inquire

The attribut can't use all the methods of inquire, so the dev user is very limited in that regard, unless he's writing a function to pass through.
Is it better to add attributs, which will be only for inquire (not very modular), or are the function enough ?

## Which type of input

Trait with generic <P> parameters have issue:
- How to pass value to method which can be different. tuple with generic or different Trait.
- How to pass the good value to downstream structs ? with function passed.

With generics, simple type could have a default &str so that the macros knows what to call. Impl Promptable<&str> for Vec<T> would allow to use Vec<impl Ptomptable> in structs without adding more complexity to macro.

If a type isn't Promptable<&str>, the user need to specify a function to use, for example if it's a struct impl<&str, &str>, it will use the function Type::new_by_prompt(&str_value, &str_value). The values can still be inserted in the call of the original struct, the name will be in prompt attributs.

If a struct is only <&str>, it will implement Promptable for Vec<T> automaticcly. Unless imple for Vec<T> use trait_gen is used to specify which type can be Vec<T>.

But it will also have a new type VecType created to implement Vec<T> with features of the proc macro. 

VecType will be public in the macro.


If I use a trait without generics, the trait of the structs implemented would be created by the macro. 

The type would be more easy to use:
Type::new_by_prompt()
Type::new_by_prompt(())
but the VecType would be:
<VecType as VecTypePromptable>::modify_by_prompt()
Vec<Type>::modify_by_prompt(())

Two branches can be created, one for using a generic trait and one for creating traits in the macro.

 
