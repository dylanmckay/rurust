use {classes,ErrorKind, VM};

// We cannot have more than two VMs at a time, and so we have a single
// test that calls the other tests.
#[test]
fn tests() {
    let mut vm = VM::new().unwrap();

    self::returns_syntax_errors_as_results(&mut vm);
    self::can_eval_simple_assignment(&mut vm);
}

fn returns_syntax_errors_as_results(vm: &mut VM) {
    let e = vm.eval("a <===- 2").unwrap_err();

    match e {
        ErrorKind::Exception(e) => {
            assert_eq!(classes::SyntaxError(), e.class());
        },
        _ => panic!("unexpected exception type"),
    }
}

fn can_eval_simple_assignment(vm: &mut VM) {
    vm.eval("a = 1").unwrap();
}

