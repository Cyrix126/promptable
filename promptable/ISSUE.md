# ISSUES


## Traits created only for one type.

For different parameters be passed through this declarative macro, a trait (actually two, one to be implemented on the struct and another for Vec<T>) need to be made for each struct. I'm not sure that's a good way of coding.

## Low Debugging Output

The dev user using the declarative macro could get errors if for example a field doesn't implement Display or if the function he passed as params attribut get consumed by a function he also passed if this function get called twice or more.

cargo expand can be used to see the result of the macro.


## No analyze of code from LSP server on strings

Because the attributs takes String as argument, rust-analyze can't process them for correctness and autocompletion.

So for example if a params takes a parameter which is not in the scope, it will just complain but do not offer the solution to bring the type in the scope.

Same to verify and complete the name of function passed through.


## inquire Custom Type has low help for end user.

The help output for the CustomType implemented for basic types is not very helpfull for the end user. Maybe an implementation more precise for types should be better (for example with a validator for number).

A new attribut for helper message could be also done.

## Limit the power of inquire

The attribut can't use all the methods of inquire, so the dev user is very limited in that regard, unless he's writing a function to pass through.
Is it better to add attributs, which will be only for inquire (not very modular), or are the function enough ?