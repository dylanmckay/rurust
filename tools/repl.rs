extern crate rurust;
extern crate mri_sys as ffi;

use std::io::Write;

#[no_mangle]
pub extern fn thing(self_obj: rurust::Value, num: rurust::Value) -> rurust::Value {
    println!("from rust: {:?} with {}", self_obj, num);
    rurust::Value::nil()
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
            method("thing", thing as *mut _, 1).
            build();

        match vm.eval(&line) {
            Ok(a) => {
                let val = if a.is_nil() { "nil".to_owned() } else { a.inspect_string() };

                println!("=> {}", val);
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
