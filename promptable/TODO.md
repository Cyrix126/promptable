# TODO

## Interface

- [ ] menu multiple_by_prompt, add a preview of the present structs.
- [x] Cancel from mod menu and multiple menu should return to the menu instead of unwrap.

## Stability

- [x] cancel should in any case crash.It should be ignored or return None.

## Fonctionnality

- [x] for new_by_prompt() method, add possiblity to ask for an Option<T> but without forcing an anwser.
- [x] attributs to give a function to new_by_prompt and modify_by_prompt in the same time.
- [ ] add a way to complete the value of multiples fields with only one function.

This function will need to have a signature identical to the fields in order.

## For Public Release

- [x] add metadata for cargo following [api guidelines](https://rust-lang.github.io/api-guidelines/documentation.html#cargotoml-includes-all-common-metadata-c-metadata)
- [x] learn how to organize the two crates
- [x] write documentation for promptable and promptable_derive

## Modularity

- [ ] add a way to replace inquire, time and termion.

## Internationalization

- [ ] search how to use languages files