# rust
`rust`, the fascinating language - almost low-level, safe, reliable

## My thoughts on Rust
 * Great (relief, boon) for C/C++ developers
 * Like Go, it is a modern well-designed language
 * Traps 99.99% of all possible issues at compile-time itself
 * It is like a stick-shift car, so great (and necessary) user-control on the runtime (memory, etc)

## Running
The short and sweet way ...
```
./compile-run.sh hello-world
./compile-run.sh immutable-variable
./compile-run.sh mutable-variable
./compile-run.sh genai <prompt>
```
A little lengthier way ...
```
./compile-run.sh src/hello-world.rs # the .rs is optional
./compile-run.sh src/immutable-variable.rs # the .rs is optional
./compile-run.sh src/mutable-variable.rs # the .rs is optional
./compile-run.sh src/genai.rs <prompt> # the .rs is optional
```
