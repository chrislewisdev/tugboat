In no particular order, things I need to add support for or consider:

- Expressions - arithmetic, logical, bitwise
- 'Synchronise' logic when parsing errors occur
- if statements, for loops, break/continue
- Type system - pointers, arrays, function pointers(?)
- extern references
- asm blocks
- Function calls
- Built-in functions (copy)
- Function parameters
- Multiple files - e.g. imports or handling multiple translation units
- Structures

Open questions:

- Should static checks (e.g. undefined variables, type checks) happen before or during codegen stage?
