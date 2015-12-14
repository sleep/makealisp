# A [make-a-lisp](https://github.com/kanaka/mal) implementation

**Objective:**
- Write a Lisp interpreter from scratch with a hybrid of systems programming languages.
- Get it to self-host

**Goals:**
*Learn about...*
- PL implementation
- C
- Rust
- anatomy of Lisps

**Constraints:**
- Don't look at any reference implementations!
- Must use only C standard library. *(and GNU readline (and libpcre..))*

**Resources:**
- Kanaka's [make-a-lisp](https://github.com/sleep/mal/blob/master/process/guide.md)
<br/>


## Roadmap/Journal

### Preliminary thoughts

Why? Working up to self-hosting lisp interpreter seems like a pretty pedagogically cost-effective, self-contained project. I chose C and Rust as host languages to fill some holes in my CS knowledge.

Compilers/interpreters exist in a vast decision space, so I'm using Kanaka's Make-a-Lisp as a roadmap to save me from moments of Buridan's assery.
<br/>

### Step 0: The REPL
- [x] set up project
- [x] set up print
- [x] Use gnu readline

First forays into C ecosystem; got familiar with make, valgrind, linking. Managed to wrap my head around pointers and null-terminated strings.

### Step 1: Read and Print

`Scaffolding:`
- [x] make a dynamic list data structure
- [x] design enum-based type system
- [x] make enum-based token type

`Read :: (String) -> AST`
- [x] make a tokenizer
- [x] make a recursive descent parser

`Print :: (AST) -> String`
- [x] implement a way to get string representation of nodes

Figured out more of the mess regarding C declaration syntax. Dealt with const correctectness. Getting the hang of heap vs stack. Grokking function pointers for testing. Immersed myself with conventions of `strings.h`.

I've decided to deal with dynamic strings closer to the metal with `char**`'s, instead of wrapper objects. This is the technique `asprintf` uses to write strings without knowing the length of strings to preallocate a buffer.

### Step 2: Eval

Should I move to Rust? Can I use my C bit and do the rest in Rust, via FFI?
