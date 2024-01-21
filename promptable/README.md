# README

## WIP

This is a **work in progress**, but you are welcome to help me think and construct this library.  
I advise to not use this library in end products until first stable version has been released !  

## Promptable

Library to make structs prompt-able.

**promptable** is a library bringing the trait [Promptable](promptable_derive/README.md) and implementations for basic types.

This library use [inquire-time](https://github.com/Cyrix126/inquire-time), a fork of inquire using the [time](https://docs.rs/time/latest/time) crate instead of chrono for the *date* feature of inquire.

The crate inquire allows to get more customizable and intuitive prompts for specific types.

One of the goal of this crate is to be modular to be able to change the prompt backend. Nevertheless, For my own personnal use case I only need inquire as a prompt backend right now.

The big part of this library is to bring the declarative macro [Promptable](promptable_derive) to make structs prompt-able very quickly but also with a lot of possibilities.


## Features:

- implement the trait Promptable for basic types.
- customizable message prompt
- manage prompt for Option\<T\>
- allows to make your own implementation of the Promptable trait
- let you handle cancel from user.
- bring a declarative macro Promptable for structs, which is why you want to use this library.


## Technical Details:

With the trait Promptable, you have two methods. One for creating a new value and a one to modify self value. Both methods will call a prompt to interact with user. You can pass a customizable message.

### Example:

```rust,no_run
use promptable::Promptable;
if let Some(mut anwser) = bool::new_by_prompt("Do you agree ?")? {
anwser.modify_by_prompt("Are you sure ?")?;
let age = i32::new_by_prompt("What is your age ?");
}
let mut ages: Vec<u32> = Vec::new();
ages.modify_by_prompt("Manage multiples ages")?;
# Ok::<(), anyhow::Error>(())
```

You can see more examples in the /examples folder.  
You can run them with:  

```bash,ignore
cargo run --example name_example
```

### Limitation / ISSUE

The concept of this library has some limitation that are described here [ISSUES](ISSUE.md)
