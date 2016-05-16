# rurust

[![Build Status](https://travis-ci.org/dylanmckay/rurust.svg?branch=master)](https://travis-ci.org/dylanmckay/rurust)
[![Crates.io](https://img.shields.io/crates/v/rurust.svg)]()
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Rust wrapper over the MRI Ruby VM.

Allows you to create a Ruby VM, `eval` code, plug classes,
define modules, and insert C functions into the environment.

## Examples

### A simple REPL

``` rust
extern fn callable_from_ruby() {
    println!("Hello World!");
}

fn main() {
    let mut vm = rurust::VM::new().unwrap();

    vm.class("Rust").
        method("hello_world", callable_from_ruby as *const _, 0).
        method("foo", callable_from_ruby as *const _, 0).
        build();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let result = vm.eval(&line);
        println!("{:?}", result);
    }
}

```
