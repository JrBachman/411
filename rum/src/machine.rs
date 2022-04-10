use std::collections::HashMap;
use std::io::prelude::*;
pub struct Field {
    width: u32,
    lsb: u32,
}
static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };
#[derive(Debug, PartialEq, Clone, Copy)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}
// Invariants
// ● Every 32 bit identifier is unique, only matches to one word.
// ● M[0] will always be mapped throughout program, otherwise
// program would crash.
// ● Instruction will never output a value larger than 255.
// ● A segment will only ever be categorized as mapped or unmapped,
// never both at the same time

///Virtual Machine
/// # Parameters:
/// * `registers`: Vectors of u32, contents represent what is stored within the register.
/// * `memory`: Hashmap of u32 keys, and values of Vec<u32> that represent memory segments and their identifiers.
/// * `program_counter`: Tracks the current instruction.
/// * `last_key`: Tracks the last used identifier, so the UM can map with new identifiers.
pub struct VirtualMachine {
    pub registers: Vec<u32>,
    pub memory: HashMap<u32, Vec<u32>>,
    pub program_counter: i32,
    pub last_key: u32,
}
impl VirtualMachine {
    ///Initializes machine
    /// # Arguments:
    ///  * `program`: program in binary to be run
    pub fn initialize_machine(&mut self, program: Vec<u32>) {
        // Initializes the virtual machine by creating empty register vector, empty memory vector.
        for _i in 0..8 {
            self.registers.push(0);
        }

        // Stores program in memory[0].
        self.memory = HashMap::new();
        self.memory.insert(0, program);
        self.program_counter = 0;
    }
    /// Conditional Move
    /// if $r[C] != 0 then $r[A] := $r[B]
    fn conditional_move(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        if self.registers[c as usize] != 0 {
            self.registers[a as usize] = self.registers[b as usize];
        }
    }
    /// Segmented Load
    /// $r[A] := $m[$r[B]][$r[C]]
    fn load_into(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        // if $r[C] != 0 then $r[A] := $r[B]
        self.registers[a as usize] =
            self.memory[&self.registers[b as usize]][self.registers[c as usize] as usize];
    }
    /// Segmented Store
    /// $m[$r[A]][$r[B]] := $r[C]
    fn store(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        (*self.memory.get_mut(&self.registers[a as usize]).unwrap())
            [self.registers[b as usize] as usize] = self.registers[c as usize];
    }
    /// Addition
    /// $r[A] := ($r[B] + $r[C]) mod 2^32
    fn add(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        // $r[A] := ($r[B] + $r[C]) mod 2^32
        self.registers[a as usize] = ((self.registers[b as usize] as usize
            + self.registers[c as usize] as usize)
            % usize::pow(2, 32)) as u32;
    }
    ///Multiplication
    /// $r[A] := ($r[B] × $r[C]) mod 2^32
    fn multiply(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        self.registers[a as usize] = ((self.registers[b as usize] as usize
            * self.registers[c as usize] as usize)
            % usize::pow(2, 32)) as u32;
    }

    ///Division
    /// $r[A] := ($r[B] ÷ $r[C]) (integer division)
    fn divide(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        self.registers[a as usize] = self.registers[b as usize] / self.registers[c as usize];
    }
    ///Bitwise nand
    /// $r[A] :=¬($r[B]∧$r[C])
    fn nand(&mut self, instruction: u32) {
        let a = get(&RA, instruction);
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        // $r[A] := ($r[B] ÷ $r[C]) (integer division)
        self.registers[a as usize] = !(self.registers[b as usize] & self.registers[c as usize]);
    }
    ///Map segment
    /// # Task:
    /// A new segment is created with a number of
    /// words equal to the value in $r[C]. Each word in
    ///the new segment is initialized to zero. A bit
    /// pattern that is not all zeroes and does not
    /// identify any currently mapped segment is placed
    /// in $r[B]. The new segment is mapped as
    ///$m[$r[B]].
    fn map_segment(&mut self, instruction: u32, pool: &mut Vec<u32>) {
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        //check pool
        // ● A segment will only ever be categorized as mapped or unmapped,
        // never both at the same time
        let new_segment = vec![0; self.registers[c as usize] as usize];
        if pool.len() > 0 {
            let key = pool.pop().unwrap();
            self.registers[b as usize] = key;
            *self.memory.get_mut(&self.registers[b as usize]).unwrap() = new_segment;
        } else {
            self.last_key += 1;
            self.registers[b as usize] = self.last_key;
            self.memory.insert(self.registers[b as usize], new_segment);
        }
    }
    ///Unmap Segment
    ///The segment $m[$r[C]] is unmapped. Future Map Segment instructions may reuse the identifier $r[C].
    fn unmap_segment(&mut self, instruction: u32, pool: &mut Vec<u32>) {
        let c = get(&RC, instruction);

        pool.push(self.registers[c as usize]);
    }

    ///Output
    /// # Task:
    /// The value in $r[C] is displayed on the I/O
    /// device immediately. Only values from 0 to 255
    /// are allowed.
    fn output(&mut self, instruction: u32) {
        //Instruction will never output a value larger than 255.
        let c = get(&RC, instruction);
        print!("{}", self.registers[c as usize] as u8 as char);
    }
    ///Input
    /// # Task:
    /// The UM waits for input on the I/O device.
    /// When input arrives, $r[c] is loaded with the
    /// input, which must be a value from 0 to 255. If
    /// the end of input has been signaled, then $r[C] is
    /// loaded with a full 32-bit word in which every bit
    /// is 1.
    fn input(&mut self, instruction: u32) {
        let c = get(&RC, instruction);
        //Proper reading in?
        let x = std::io::stdin().bytes().next();
        match x {
            Some(x) => {
                self.registers[c as usize] = x.unwrap() as u32;
            }
            None => {
                self.registers[c as usize] = 4294967295;
            }
        }
    }

    ///Load program
    /// # Task:
    /// Segment $m[$r[B]] is duplicated, and the
    /// duplicate replaces $m[0], which is abandoned.
    /// The program counter is set to point to
    /// $m[0][$r[C]]. If $r[B]=0, the load program
    /// operation should be extremely quick, as this is
    /// effectively a jump
    fn load_program(&mut self, instruction: u32) {
        let b = get(&RB, instruction);
        let c = get(&RC, instruction);
        // ● M[0] will always be mapped throughout program, otherwise
        // program would crash.
        if self.registers[b as usize] == 0 {
            //jump
            self.program_counter = self.registers[c as usize] as i32 - 1;
        } else {
            let dupe = self.memory[&self.registers[b as usize]].clone();
            *self.memory.get_mut(&0).unwrap() = dupe;
            // self.program_counter =
            //     self.memory[&0][self.registers[c as usize] as usize];

            self.program_counter = self.registers[c as usize] as i32 - 1;
        }
    }
    ///Load value
    /// # Task:
    /// The three bits immediately less significant than
    /// opcode describe a single register A. The remaining 25 bits indicate a value,
    /// which is loaded into $r[A].
    fn load_value(&mut self, instruction: u32) {
        let a = get(&RL, instruction);
        let v = get(&VL, instruction);
        self.registers[a as usize] = v;
    }

    ///Runs the given program
    pub fn run_program(&mut self) {
        //Contains unmapped segment identifiers
        let mut pool: Vec<u32> = vec![];
        // Loops through execution cycle.

        loop {
            let instruction = self.memory[&0][self.program_counter as usize];
            // Handles instructions similar to lab
            match get(&OP, instruction) {
                o if o == Opcode::CMov as u32 => {
                    self.conditional_move(instruction);
                }
                o if o == Opcode::Load as u32 => {
                    self.load_into(instruction);
                }
                o if o == Opcode::Store as u32 => {
                    self.store(instruction);
                }
                o if o == Opcode::Add as u32 => {
                    self.add(instruction);
                }
                o if o == Opcode::Mul as u32 => {
                    self.multiply(instruction);
                }
                o if o == Opcode::Div as u32 => {
                    self.divide(instruction);
                }
                o if o == Opcode::Nand as u32 => {
                    self.nand(instruction);
                }
                o if o == Opcode::Halt as u32 => {
                    return;
                }
                o if o == Opcode::MapSegment as u32 => {
                    self.map_segment(instruction, &mut pool);
                }
                o if o == Opcode::UnmapSegment as u32 => {
                    self.unmap_segment(instruction, &mut pool);
                }
                o if o == Opcode::Output as u32 => {
                    self.output(instruction);
                }
                o if o == Opcode::Input as u32 => {
                    self.input(instruction);
                }
                o if o == Opcode::LoadProgram as u32 => {
                    self.load_program(instruction);
                }
                o if o == Opcode::LoadValue as u32 => {
                    self.load_value(instruction);
                }

                _ => {}
            }
            self.program_counter = self.program_counter + 1;
        }
    }
}

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

type Umi = u32;

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
