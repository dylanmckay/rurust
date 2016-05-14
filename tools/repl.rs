extern crate rurust;

use std::io::Write;

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

        let result = match vm.eval(&line) {
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
