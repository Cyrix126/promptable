# Derive attributs

Here's an explanation of every attributs you can use to customize the methods created for your structs.

All structs using this derive trait must also implement the trait Clone and Display. You will get an error at compile time if it's not the case.


## Struct attributs


- ##### `#[promp(params = String)]`

Transfer the signature to the trait and method to be able to use functions with parameters inside methods. Use with field attribut function_new or function_mod.
Put here every parameters that will be needed for the functions to be executed inside the methods.
If you have two attributs with functions, except if you want the same parameter to be passed, you need to put all the parameters for each function.

- ##### `#[promp(msg_mod = String)]`

Message to show when in the prompt menu to modify the struct.


## Fields attributs

- ##### `#[promptable(default = bool)]`

This field will not be asked, the value will be the type default value. For Option\<T\> type, the value will be None.

- ##### `#[promptable(name = String)]`

What name to be displayed on various prompt.

- ##### `#[promptable(visible = bool)]`

Is the field proposed on menu and creation. If the field is Option, the value will be None. If the field is a type, it will be the default. The difference with default attribut is that the field will never be asked even if the struct is modified.

- ##### `#[promptable(msg = String)]` 

Message to transfer to user while interacting with the prompt. This message can be overriden by a function_new or function_modify attribut.

- ##### `#[promptable(function_new = String)]`

Calls the function described as String instead of promptable::Promptable::new_by_prompt() for this field while creating an instance of this struct.

- ##### `#[promptable(function_mod = String)]`

Calls the function described as String instead of promptable::Promptable::modify_by_prompt() for this field while modifying an instance of this struct.

- ##### `#[promptable(function_add = String)]`

Calls the function described as String instead of promptable::Promptable::new_by_prompt() for this field while adding an instance of this struct on a Vec. This function can use self to use the vec. Refer to the example "complex_form" to see it in action.

- ##### `#[promptable(function = String)]`

Will put the same function as for function_new and function_mod.

- ##### `#[promptable(function_render = String)]`

This function will be executed to render the value of the field when the user is asked to select fields. (modify_by_prompt).
field_value mut be used as a parameter. Do not add it in the "prompt" attribut because the value is already inside the macro in a variable called field_value.
This function must return a type which implement the trait Display or you will get an error from the expanded macro.

You can see an example in "complex_form".

- ##### `#[promptable(multiple_once = String)]`

Precise if this field should be asked only once when creating a vec of this struct.

