extern crate rurust;
extern crate mri_sys as ffi;

use std::io::Write;

#[no_mangle]
pub extern fn do_rust_thing() {
    println!("from rust");
}

fn main() {
    let mut vm = if let Ok(vm) = rurust::VM::new() {
        vm
    } else {
        println!("could not create VM");
        return;
    };

    loop {
        let mut line = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_owned();

        if line == "exit" || line == "quit" {
            break;
        }


        vm.class("Abc").
            method("thing", do_rust_thing as *const _, 0).
            build();

        match vm.eval(&line) {
            Ok(a) => {
                println!("=> {}", a);
            },
            Err(rurust::ErrorKind::Exception(ref value)) => {
                let ty = value.class();
                println!("{}: {}", ty, value);
                continue;
            },
            Err(rurust::ErrorKind::VM(ref message)) => {
                println!("Internal VM error: {}", message);
                return;
            },
        };
    }
}
