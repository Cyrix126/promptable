# README

the crate promptable_derive bring the declarative macro.

## Features:

- implement the trait Promptable\<P\> on your struct, where P is a tuple of your parameter you want to use for your functions.
- create a wrapper around Vec\<YourStruct\> named VecStructName and implement the trait Promptable.
- lot of optional attributs to customize prompts (message to display, skip fields, use your own functions with any parameters).
- cool interface with inquire
- menus for managing vec from a user perspective.
- manage cancellation from user.
- *panic free*

## Examples

See the folder [example](../promptable/examples)

## How to Use

Apply traits Promptable and Clone on your struct.

```rust,ignore
#[derive(Promptable, Clone)]
struct StructExample {
        name: String,
        email: String
}

```
And then you can call a method to create one
```rust,ignore
if let Some(struct) = StructExample::new_by_prompt(())? {};
```
You can then modify it
```rust,ignore
struct.modify_by_prompt(())?;
```
A new type will be created wrapping a Vec\<StructExample\>.
```rust,ignore
let multiples = VecStructExample(Vec::new());
multiples.modify_by_prompt(())?
```

See the documentation locally, later online.
 
```bash, ignore
cargo doc --open
```
