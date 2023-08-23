# focus-lang
## Contents:
 - [Syntax](Syntax.md)
 - [Examples](#examples)

## Examples:
```ocaml
    type Point = {x: int, y: int}

    let distance_between_points a b: (Point -> Point -> int) = 
        let x = Std.Math.pow 2 (a.x - b.x)
        let y = Std.Math.pow 2 (a.y - b.y)
        Std.Math.sqrt (x + y)


    let main = 
        let pointA = Point {x: 0, y: 0}
        let pointB = Point {x: 5, y: 6}
        let length = distance_between_points pointA pointB
        print "The length between the two points is {length}."
```