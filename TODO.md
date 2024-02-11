In no particular order, things I need to add support for or consider:

- Expressions - arithmetic, logical, bitwise
- 'Synchronise' logic when parsing errors occur
- if statements, for loops, break/continue
- Type system - pointers, 16-bit values, function pointers(?)
- extern references
- asm blocks
- Function calls
- Built-in functions (copy)
- Function parameters
- Multiple files - e.g. imports or handling multiple translation units
- Structures

Path to writing a basic text console:
- Essential operators like equality, comparison
- Ability to call built-in functions
- Ways to reference gameboy hardware constants (e.g. hardware.inc)
- Support for 16-bit values, e.g. indexing into tile/map data
- Basic string support

Open questions:

- Should static checks (e.g. undefined variables, type checks) happen before or during codegen stage?
