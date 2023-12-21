# README

the crate promptable_derive bring the declarative macro.

## Features:

- method to create and modify a promptable struct with menu.
- method to create and modify a vec of struct with menu..
- lot of optional attributs to customize prompts.
- transfer parameters to called functions.
- manage cancellation from user.

### Traits

This declarative macro when used on structs, will crate two new traits:

```rust,ignore
        trait #trait_name {
               fn new_by_prompt(#params) -> #nom;
               fn modify_by_prompt(&mut self, #params);
        }
```

```rust,ignore
        trait #trait_name_multiple {
            fn new() -> Vec<#nom>;
            fn ajout(&mut self, #params);
            fn delete(&mut self);
            fn modify(&mut self, #params);
            fn multiple_by_prompt(#params) -> Vec<#nom>;
        }
```

It will implement them on the struct of type T and Vec\<T\>, depending of the types and attributes of each fields.

#### Methods


##### for T

```rust,ignore
fn new_by_prompt(#params) -> #nom;
```

This method will construct the struct, prompting each fields by default with their implementation of the [Promptable](../promptable::Promptable) trait.  
This behavior can be overriden by attributs.

```rust,ignore
fn modify_by_prompt(&mut self, #params); 
```

This method will allow the values of the struct to be selected and modified in a menu.

##### for Vec\<T\>

```rust,ignore
fn multiple_by_prompt(&mut, #params);
```

This method allows to create multiple T with a menu to correct entries.