# focus-lang

Try the project in the [Playground](https://periklis92.github.io/focuslang).

## Contents:
 - [Syntax Rules](docs/SyntaxRules.md)
 - Concepts:
    - [Iter](docs/Iter.md)
    - [Functors](docs/Functors.md)
 - [Examples](#examples)

## Examples:
```ocaml
type Point = {x: float, y: float}

let distance_between_points a b: (Point -> Point -> float) = 
    let square f = Std.Math.pow 2 f
    Std.Math.sqrt ( (square (a.x - b.x)) + (square (a.y - b.y)))


let main = 
    let pointA = Point {x: 0.0, y: 0.0}
    let pointB = Point {x: 5.0, y: 6.0}
    let length = distance_between_points pointA pointB
    print "The length between the two points is {length}."
```