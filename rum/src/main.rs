use rum::machine;
use rum::rumload;
use std::collections::HashMap;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2);
    let filename = args.iter().nth(1).unwrap();
    let program = rumload::load(Some(filename));
    let mut vm = machine::VirtualMachine {
        registers: vec![],
        memory: HashMap::new(),
        program_counter: 0,
        last_key: 0,
    };
    vm.initialize_machine(program);
    vm.run_program();
}
