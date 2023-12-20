# ROADMAP

## Modularity.

Right now, the use of inquire is pretty much forced in this crate. The dev user should be able to use features to add different backend.

For example maybe he wants a much more basic prompt which is not using inquire or, on the contrary, wants to leave the terminal and use a gui prompt or even render a web page.

This should be possible with the use of features. Contributors could add a feature which would enable a new backend for prompt.


## Preview for menu multiple_by_prompt

Add a preview of present structs in the menu.
Those structs should have a method to be called. This method should be made by the user and the result should be display-able.
Can this method be automaticly made and if yes how to override it ?

## Internationalization

Some prompts possess default messages which are in english. Their provenance is from the inquire crate or from this crate.

The translation should be made easy with languages files.
Because the dev user can use any function for any prompts, nothing block a dev from translating or customize those messages.

Why not use the crate [rust-i18n](https://lib.rs/crates/rust-i18n)