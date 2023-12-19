# Derive attributs

Here's an explanation of every attributs you can use to customize the methods created for your structs.

All structs using this derive trait must also implement the trait Clone and Display. You will get an error at compile time if it's not the case.


## Struct attributs


- ##### `#[promp(params = String)]`

Transfer the signature to the trait and method to be able to use functions with parameters inside methods. Use with field attribut function_new or function_mod.


- ##### `#[promp(msg_mod = String)]`

Message to show when in the prompt menu to modify the struct.


## Fields attributs

- ##### `#[promptable(default = bool)]`

This field will not be asked, the value will be the type default value. For Option\<T\> type, the value will be None.

- ##### `#[promptable(name = String)]`

What name to be displayed on various prompt.

- ##### `#[promptable(visible = bool)]`

Is the field asked when created. If the field is Option, the value will be None. If the field is a type, it will be the default. The difference with default attribut is that the field will never be asked even if the struct is modified.

- ##### `#[promptable(msg = String)]` 

Message to transfer to user while interacting with the prompt. This message can be overriden by a function_new or function_modify attribut.

- ##### `#[promptable(function_new = String)]`

Calls the function described as String instead of promptable::Promptable::new_by_prompt() for this field while creating an instance of this struct.

- ##### `#[promptable(function_mod = String)]`

Calls the function described as String instead of promptable::Promptable::modify_by_prompt() for this field while modifying an instance of this struct.

- ##### `#[promptable(multiple_once = String)]`

Precise if this field should be asked only once when creating a vec of this struct.

