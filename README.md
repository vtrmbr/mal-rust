# mal-rust

A Rust implementation of [Make-A-Lisp](https://github.com/kanaka/mal) (mal), a small Clojure-inspired Lisp, written step by step by following the [mal process guide](https://github.com/kanaka/mal/blob/master/process/guide.md).

It is complete through the last step (step A). It also runs mal's own implementation of mal, written in mal, on top of it (self-hosting).

## Status

- Steps 0 to A are implemented, including the deferrable parts of the guide: strings, keywords, vectors, hash-maps, atoms, macros, quasiquote, `try*`/`catch*`/`throw`, metadata (`meta`, `with-meta`, `^`), `readline`, `time-ms` and the rest of the core library.
- The official tests for every step pass. One optional step A test (`time-ms`) compares two timestamps taken a moment apart, so it can fail on a very fast machine.
- Not implemented: the optional host-language interop function (`rust-eval`). Rust is compiled ahead of time and has no runtime `eval`, so there is nothing sensible to build it on.
- steps 0 through 9 might not buid because fo changes made in later steps to the shared modules.
- prehabs will still add some extra features if I feel like it, this is just a project for study, don't expect anything.

## Build and run

You need a recent stable [Rust toolchain](https://rustup.rs/).

```
cargo build --release --bin stepA_mal

./target/release/stepA_mal                 # start the REPL
./target/release/stepA_mal script.mal a b  # run a file; *ARGV* is ("a" "b")
```

To use it as a command, copy the binary somewhere on your `PATH`:

```
cp target/release/stepA_mal ~/.local/bin/mal
```

Define `DEBUG-EVAL` as a true value (`(def! DEBUG-EVAL true)`) to print every evaluation step.

## Example

```
user> (def! fib (fn* (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))
#<function>
user> (fib 20)
6765
user> (defmacro! unless (fn* (pred a b) `(if ~pred ~b ~a)))
#<function>
user> (unless false "yes" "no")
"yes"
user> (try* (throw {:code 42}) (catch* e (get e :code)))
42
user> (map (fn* (x) (* x x)) [1 2 3 4])
(1 4 9 16)
```

## Project layout

```
src/
  types.rs        MalType (the value enum), hash-map keys, error conversions
  reader.rs       tokenizer, Reader, read_str
  printer.rs      pr_str
  env.rs          environments (shared, with an optional parent)
  mal_core.rs     built-in functions: arithmetic, lists, strings, atoms, ...
  evaluator.rs    the eval loop, apply_fn, quasiquote helpers
  step0_repl.rs ... stepA_mal.rs    one entry point per step of the guide
```

Each `stepN_*.rs` file is a separate binary (see `Cargo.toml`), as in the guide, so the project can be read in the order it was built.

## Design notes

- **Hand-written tokenizer.** The reader walks the input character by character with a peekable iterator, with no regular expressions.
- **Tail-call optimisation as a loop.** `eval` is a `loop`. In tail position (the chosen branch of `if`, the last form of `do`, the body of `let*`, user-defined function calls, macro expansions) it replaces its current form and environment and goes around again, so deep recursion doesn't grow the Rust stack.
- **Environments** are `Rc<RefCell<Env>>` with an optional parent, which lets closures keep their defining environment alive.
- **Errors are values.** `eval` returns `Result<MalType, MalType>`, so `throw` can raise any mal value. Native errors become strings through a `From<String>` conversion.
- **Callable built-ins.** `eval` and `apply_fn` live in their own module so that built-ins like `apply`, `map` and `swap!` can call back into them, and so they are ordinary function values, which the self-hosted interpreter needs.
- **Equality.** `=` compares lists and vectors element by element at every level of nesting, and ignores metadata.

## Running the official tests

The test suite and harness live in the mal repository and are not included here. To run them:

1. Clone [kanaka/mal](https://github.com/kanaka/mal).
2. In the clone, replace the contents of `impls/rust/` with this repository's `src/` and `Cargo.toml`. Keep the `Makefile` and `run` script that are already in that folder, since the harness uses them.
3. From the root of the clone, run a step's tests:

```
make "test^rust^stepA"
```

To run mal's mal-in-mal implementation on top of this interpreter, select this implementation as the host:

```
make MAL_IMPL=rust "test^mal^stepA"
```

## Credits and license

This project follows the Make-A-Lisp guide by Joel Martin ([kanaka/mal](https://github.com/kanaka/mal)), which is licensed under the Mozilla Public License 2.0. The build configuration in `Cargo.toml` started from the template in that repository.

The code made in this project is licensed under  the MIT license

AI was used in to help me understand what and how to do some parts since it was my first making anything like this and using rust