In the module where the type int is defined there is a function that compares two ints and returns an int greater than zero if `l` is greater than `r`, 0 if they are equal or less than zero if `r` is greater than `l`
```ocaml
module int

type int

let compare l r: (int -> int -> int) =
    l - r
```
Here we define a functor that requires the module where `type t` is defined to have a function with the signature and name of `compare`.
```ocaml
functor Comparable
    type t 
    compare: (t -> t -> int)
```
Next we have a function that finds the greater of two values as long as they are `Comparable`. We can use the function as a type in the signature of the function.
```ocaml
let greater l r: (Comparable -> Comparable -> Comparable) = 
    if (Comparable.compare l r) < 0 r else l

```
```ocaml
functor Sub
    type t
    sub: (t -> t -> t)
```
Here we require that both `l` and `r` are `Comparable` and `Sub`.
```ocaml
let positive_sub l r: (Comparable + Sub -> Comparable + Sub -> Comparable + Sub) = 
    if (Comparable.compare l r) < 0
        Sub.sub r l
    else
        Sub.sub l r
```
This is quite verbose so we can use an alias.
```ocaml
type CompSub = Comparable + Sub
let positive_sub l r: (CompSub -> CompSub -> CompSub) = 
    if (CompSub.compare l r) < 0
        CompSub.sub r l
    else
        CompSub.sub l r
```
Or simply let the interpreter infer the types.
```ocaml
let positive_sub l r = 
    if (Comparable.compare l r) < 0
        Sub.sub r l
    else
        Sub.sub l r
```