# Derive attributs

Here's an explanation of every attributs you can use to customize the methods created for your structs.

All structs using this derive trait must also implement the trait Clone. You will get an error at compile time if it's not the case.


## Struct attributs


- ##### `#[promp(params = String)]`

Transfer the signature to methods to be able to use functions with parameters. It makes only sense if you use it with fields attributs function*.
Put here every parameters that will be needed for the functions to be executed inside the methods.
If you have two functions with attributs, except if you want the same parameter to be passed, you need to put all the parameters for each function.
Example: 
```rust,ignore
#[prompt(params = "msg: &str, is_true: bool,")]
```


- ##### `#[promp(msg_mod = String)]`

Message to show when in the prompt menu to modify the struct.

- ##### `#[promp(custom_prompt_display = bool)]`

Allows you to disable the implementation of PromptableDisplay by the macro to let you make your own custom implementation.
You will need to implement the PromptableDisplay trait for your Struct and for the type of name "VecStruct", Struct as the name of your Struct.
- ##### `#[promp(name = String)]`

To change the default name of your Struct that will appear in prompts.

## Fields attributs

- ##### `#[promptable(short_display = bool)]`

Use only this field for the short_display method. Will take the first 3 visible fields by default. You can apply it on any fields to show multiples fields for short_display.

- ##### `#[promptable(inspect = bool)]`

The field will be suggested in the list of fields to inspect. True by default.
You need to put it to false if the type or inner type of the field does not implement [Inspectable](Inspectable).


- ##### `#[promptable(default = bool)]`

This field will not be prompt for new ones, but can be edited later.  The value will be the default type value. For Option\<T\> type, the value will be None.
Can not be used with attribut fonction*

- ##### `#[promptable(visible = bool)]`

Will skip the field, the user **can not** edit it by prompt later.
The value will be the default type value. For Option\<T\> type, the value will be None.
Uselfull to skip types with no support for Promptable trait.
Can not be used with attribut fonction*

- ##### `#[promptable(name = String)]`

What name to be displayed on various prompt.

- ##### `#[promptable(msg = String)]` 

Message to transfer to user while interacting with the prompt. This message can be overriden by a function_new or function_modify attribut.

- ##### `#[promptable(multiple_once = String)]`

Precise if this field should be asked only once when creating a vec of this struct.


### Functions attributs

Specific functions with any parameters can be used instead of calling the methods of the trait Promptable.

If you want to use in a field a type Promptable which has an implementation with different parameters, you need to use function* attributs to set the method with the right parameters that you need to put in [#prompt(params ="")] if they are not present. 


Every functions declared must return Option\<T\>.
If you want to propagate the error of the function, use Result\<Option\<T\> with the ? operator in the declaration attribut.


- ##### `#[promptable(function_new = String)]`

Calls the function described as String instead of promptable::Promptable::new_by_prompt() for this field while creating an instance of this struct.

Your function needs to return <Option\<T\>.
If the field is an Option\<T\>, the Some\(T\) or None will be passed to it.
If the field is a T, if return is Some\(T\), T will be passed.
If returned is None, the prompt will be canceled Ok(None).

- ##### `#[promptable(function_mod = String)]`

Calls the function described as String instead of promptable::Promptable::modify_by_prompt() for this field while modifying an instance of this struct.
This function **SHOULD** take "field" as parameter (field: &mut T) (except if you do not want this function to modify your field).
This parameter is a mutable borrowed of the value of the field.

- ##### `#[promptable(function_add = String)]`

Calls the function described as String instead of promptable::Promptable::new_by_prompt() for this field while adding an instance of this struct on a Vec. This function can use self to use the vec. Refer to the example "complex_form" to see it in action.

This is different from function_new. For example function_add can implement some differences when creating a new element using already existing elements of the vec.
This function can take immutable borrow self as parameter, to access the values in the entire vec.

- ##### `#[promptable(function_render = String)]`

This function will be executed to render the value of the field when the user is asked to select fields. (modify_by_prompt).
field_value mut be used as a parameter. Do not add it in the "prompt" attribut because the value is already inside the macro in a variable called field_value.
This function must return a type which implement the trait Display or you will get an error from the expanded macro.

You can see an example in "complex_form".

