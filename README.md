`tugboat` (or `tug` for short) is my first attempt at creating a compiled programming language specifically for the Gameboy. Its primary intent is to be a learning project, so its design is likely to be sup-optimal in many ways.

## Goals

The general goal of `tugboat` is to try and find some kind of middle-ground between the detailed control of assembly using [RGBDS](https://rgbds.gbdev.io/) and the convenience of C using [GBDK](https://gbdk-2020.github.io/gbdk-2020/). By creating a language targeting the Gameboy exclusively, the idea is to embrace the limits of the hardware while still facilitating code that is less verbose and tedious to write than assembly.

Additionally, a goal of `tugboat` is to integrate readily into the existing Gameboy tools ecosystem. For this reason, it currently compiles to RGBDS-compatible assembly, rather than compiling straight to a ROM file. This also means we can use RGBDS as a launchpad to get the language up and running without having to handle things like linking ourselves.

## Planned features

- Simple function definitions and calling
- Nested expressions (e.g. `(1 + x) * (y - 3)`
- Looping/branching constructs (`while`/`for`/`if`)
- Arrays
- Basic string support (e.g. `u8[12] string = "Hello World!";`)

## Current progress

The general structure of the compiler is in place, with lexing/parsing/code-generation all working together. You can compile a `.tg` file into working assembly code, but only a few language features are actually working so far:

- Defining and accessing/setting variables (including array indexing)
- Defining functions (but can't call them yet!)
- `while` loops
- Basic non-nested addition and subtraction e.g. `1 + 2`

Getting even just this much implemented has opened my eyes to a lot of hairy problems I hadn't considered when coming into the project (such as register allocation), so right now I think I have some researching and planning to do before I could successfully get everything else working.

## Example

Here is an example of a working `.tg` file using only what is already implemented:

```
u8 i;
u8[10] array;

fn main() {
    // A janky way to initialise an array since there are no comparison operators yet!
    i = 11;
    while (i) {
        i = i - 1;
        array[i] = i;
    }

    while (true) {
        halt;
    }
}
```
