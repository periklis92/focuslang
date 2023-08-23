# Syntax


### Statement:

 [Item](#item)  
| [Let](#let)  
| [Expression](#expression)  
    

### Item

[Visibility](#visibility)<sup>?</sup>  
(  
    [Module](#module)  
    | [Use](#use)    
    | [Type Alias](#type-alias)  
    | [Struct](#struct)  
)

### Visibility

`pub` `module`<sup>?</sup>

### Module

`module` [Identifier](#identifier)

### Use

`use` [Simple Path](#simple-path)

### Type Alias

`type` [Identifier](#identifier) (`=` [Simple Path](#simple-path))<sup>?</sup>

### Struct

`type` [Identifier](#identifier) `=` `{` ([Struct Fields](#struct-fields))<sup>?</sup> `}`

#### Struct Fields

[Struct Field](#struct-field)(, [Struct Field](#struct-field))`,`<sup>?</sup>

#### Struct Field

[Visibility](#visibility)<sup>?</sup>  [Identifier](#identifier)`:` [Simple Path](#simple-path) 

### Simple Path

[Identifier](#identifier)(`.`[Identifier](#identifier))`*`

### Let

`let` [Identifier](#identifier) (: [Type](#let-type))<sup>?</sup>

#### Let Type

[Simple Path](#simple-path)  
| [Function Type](#function-type)

#### Function Type

`(` [Simple Path](#simple-path) `->` [Simple Path](#simple-path) (`->` [Simple Path](#simple-path))<sup>*</sup> `)`

### Identifier

(  
    `_`  
    | `[a-z]`  
    | `[A-Z]`  
)`*`