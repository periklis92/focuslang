## Statements


### Statement:

 [Item](#item)  
| [Let](#Item)  
| [Expression](#Item)  
    

### Item

[Visibility](#visibility)<sup>?</sup>  
(  
    [Module](#module)  
    | [Use](#use)    
    | [TypeAlias](#type-alias)  
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

### Identifier

(  
    `_`  
    | `[a-z]`  
    | `[A-Z]`  
)`*`