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

## New Struct should be more user friendly.

If a struct call a method new_by_prompt, the user must anwser every questions without the possiblity of going back or cancelling. The user should be able to do that and it is to the dev user to manage the return type or a way should be available to him to change the way the new struct manage the anwser.

## Use of Dynamic Function instead of reading function from string ?

This could be a better way so that the structs don't create a trait for each of them.


## Menu Helper

Offer a Menu struct that is composed of const and a method to get those consts. I would be usefull to match against after giving them as options to a Select prompt.

## Different Display Implementation

Structs could be displayed with intermediate level of verbose.

Implementation for Vec<T> would be a simple join.
https://docs.rs/display_utils/latest/display_utils/
