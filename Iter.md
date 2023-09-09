```ocaml
module Iter

pub type 'a Iter = {
    pub next: ( () -> 'a Option ),
}

pub let 'a 'b map f it: (('a -> 'b) -> 'a Iter -> 'b Iter) = 
    let mapped = it.next () @ Option.map map
    Iter {
        next: mapped,
    }

pub let 'a to_vec it: ('a Iter -> 'a Vec) =
    Vec.from_iter it

```

```ocaml
module Vec

pub type 'a Vec = {
    ptr: 'a Ptr,
    size: Size,
    cap: Size,
}

pub let 'a iter vec: ('a Vec -> 'a Iter) = 
    let index = 0
    let next = 
        fn ->
            if index >= vec.size 
                None
            else
                let el = Ptr.add index vec.ptr @ Ptr.read
                index += 1
                Some el
    Iter {
        next
    }

```