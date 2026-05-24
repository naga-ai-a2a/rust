# rust
`rust`, the fascinating language - almost low-level, safe, reliable

## My thoughts on Rust
 * Great (relief, boon) for C/C++ developers
 * Like Go, it is a modern well-designed language
 * Traps 99.99% of all possible issues at compile-time itself
 * It is like a stick-shift car, so great (and necessary) user-control on the runtime (memory, etc)
 * Deep knowledge of the constructs helps in managing the memory (stack, heap) efficiently and thus improving performance
 * [The Rust Book](https://doc.rust-lang.org/book/) is a great resource, very well documented

## Running the code snippets in this repo
The short and sweet way ...
```
./compile-run-one.sh hello-world
./compile-run-one.sh immutable-variable
./compile-run-one.sh mutable-variable
./compile-run-one.sh vec-same-type
./compile-run-one.sh vec-any-type-enum
./compile-run-one.sh vec-any-type-trait
./compile-run-one.sh genai <prompt>
```
A little lengthier way ...
```
./compile-run-one.sh src/hello-world
./compile-run-one.sh src/immutable-variable
./compile-run-one.sh src/mutable-variable
./compile-run-one.sh src/vec-same-type
./compile-run-one.sh src/vec-any-type-enum
./compile-run-one.sh src/vec-any-type-trait
./compile-run-one.sh src/genai <prompt>
```
The full length way ...
```
./compile-run-one.sh src/hello-world.rs
./compile-run-one.sh src/immutable-variable.rs
./compile-run-one.sh src/mutable-variable.rs
./compile-run-one.sh src/vec-same-type.rs
./compile-run-one.sh src/vec-any-type-enum.rs
./compile-run-one.sh src/vec-any-type-trait.rs
./compile-run-one.sh src/genai.rs <prompt>
```
To run all ...
```
./compile-run-all.sh <model> <prompt>
```
