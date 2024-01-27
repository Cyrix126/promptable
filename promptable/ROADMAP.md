# ROADMAP

## Modularity.

Right now, the use of inquire is pretty much forced in this crate. The dev user should be able to use features to add different backend.

For example maybe he wants a much more basic prompt which is not using inquire or, on the contrary, wants to leave the terminal and use a gui prompt or even render a web page.

This should be possible with the use of features. Contributors could add a feature which would enable a new backend for prompt.

## Internationalization

Some prompts possess default messages which are in english. Their provenance is from the inquire crate or from this crate.

The translation should be made easy with languages files.
Because the dev user can use any function for any prompts, nothing block a dev from translating or customize those messages.

Why not use the crate [rust-i18n](https://lib.rs/crates/rust-i18n)

## Use of Dynamic Function instead of reading function from string ?

This could be a better way so that the structs don't create a trait for each of them.

## Good diagnostic

Right now, because of the nature of the macro, it is difficult to diagnose from the user dev side of view. A lot couldbe done in this regard, specially for common errors when making the struct.

## Make a struct of Clap to make shortcuts of menu.

So the dev user can allow end user to pass args to his program and get to choosen data faster. Use clap_shortcuts crate for this.
