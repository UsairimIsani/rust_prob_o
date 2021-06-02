use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug)]
pub enum Instruction<T, R> {
    // Values
    LOAD_VAL(T),

    // Variables
    READ_VAR(R),
    WRITE_VAR(R),

    // Operations
    ADD,
    MULTIPLY,

    RETURN_VALUE,
    LOOP(usize),
    JUMP(usize),
    IF_NOT_EQUAL,
}
impl<T, R> Instruction<T, R> {
    fn parse(s: &str) -> Instruction<T, R> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ByteCode<T, R> {
    instruction: Instruction<T, R>,
}
impl<T, R> ByteCode<T, R> {
    fn parse(s: &str) -> ByteCode<T, R> {
        let instruction = Instruction::parse(s);
        ByteCode { instruction }
    }
}

pub struct Result<T> {
    val: Option<T>,
}

mod live;

pub struct Interpreter<T, R> {
    stack: Vec<T>,
    map: HashMap<R, T>,
    prog_count: usize,
}
impl<T, R> Interpreter<T, R> {
    fn new() -> Self {
        Self {
            stack: vec![],
            map: HashMap::new(),
            prog_count: 0,
        }
    }
    fn interpret(&mut self) {}
}
fn interpreter<T, R>(byte_code: &[ByteCode<T, R>]) -> Option<T>
// fn interpreter(byte_code: &[ByteCode<i32, &str>]) -> Option<i32>
where
    // Solved an issue regarding pushing to vec. https://stackoverflow.com/questions/37647248/mismatched-types-when-returning-the-result-of-adding-two-generics
    T: std::fmt::Debug
        + Default
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Copy
        + Eq
        + PartialEq,
    R: std::fmt::Debug + Default + Eq + Hash + Copy,
{
    // A Program Counter to keep track of current execution.
    let mut prog_count = 0;

    // Stack to store temp vars.
    let mut stack: Vec<T> = Vec::new();
    // let mut stack: Vec<i32> = Vec::new();

    // A look up table to keep track of variables
    let mut map: HashMap<R, T> = HashMap::new();
    // let mut map: HashMap<&str, i32> = HashMap::new();

    let mut r = None;

    let mut current_iter = 0;
    let mut loop_limit = 0;

    while prog_count < byte_code.len() {
        // Advance the program counter by
        let mut adv_pc = 1;

        match byte_code[prog_count].instruction {
            Instruction::LOAD_VAL(val) => {
                // Put temp var into stack
                // println!("Stack in LOAD_VAL: {:?}\n", stack);
                stack.push(val);
            }
            Instruction::WRITE_VAR(key) => {
                // write to the lookup table from the stack
                let val = stack.pop().unwrap();
                map.insert(key, val);
                // println!("Map : {:?}", map);
            }

            Instruction::READ_VAR(key) => {
                // read from the map and push it to the stack
                let val = map.get(&key).unwrap();
                stack.push(val.clone());
            }
            Instruction::ADD => {
                // Perform Operation 'ADD'on 'a' and temp val in the stack
                // println!("Stack in ADD: {:?}\n", stack);
                if let Some((a, b)) = stack.pop().zip(stack.pop()) {
                    stack.push(a + b);
                }
            }
            Instruction::MULTIPLY => {
                // Perform Operation 'MULTIPLY' on 'a' and temp val in the stack
                if let Some((a, b)) = stack.pop().zip(stack.pop()) {
                    stack.push(a * b);
                }
            }

            Instruction::RETURN_VALUE => {
                r = stack.pop();
            }
            Instruction::LOOP(l) => {
                // map.insert(key, loop_limit);
                // println!("Stack in LOOP: {:?}\n", stack);
                loop_limit = l;
                current_iter = 0;
            }
            Instruction::JUMP(p_c) => {
                // println!("Stack in JUMP: {:?}\n", stack);
                prog_count = prog_count - p_c - 1;
                current_iter += 1;
            }
            Instruction::IF_NOT_EQUAL => {
                // println!("Current : {:?}", current_iter);
                // println!("Stack : in IFNOT{:?}\n", stack);
                // println!("Map  : {:?}", map);
                if loop_limit == current_iter {
                    adv_pc = 2;
                }
            }
        }
        prog_count += adv_pc;
    }

    r
}
fn main() {}

#[cfg(test)]
mod tests {
    use crate::{interpreter, ByteCode, Instruction};

    #[test]
    fn load_val() {
        let b_code: Vec<ByteCode<i32, &str>> = vec![
            ByteCode {
                instruction: Instruction::LOAD_VAL(2),
            },
            ByteCode {
                instruction: Instruction::RETURN_VALUE,
            },
        ];
        let r = interpreter(&b_code);

        println!("{:?}", r);

        assert_eq!(Some(2), r);
    }

    #[test]
    fn write_var() {
        let b_code: Vec<ByteCode<i32, &str>> = vec![
            ByteCode {
                instruction: Instruction::LOAD_VAL(2),
            },
            ByteCode {
                instruction: Instruction::WRITE_VAR("x"),
            },
            ByteCode {
                instruction: Instruction::LOAD_VAL(44),
            },
            ByteCode {
                instruction: Instruction::WRITE_VAR("y"),
            },
            ByteCode {
                instruction: Instruction::RETURN_VALUE,
            },
        ];
        let r = interpreter(&b_code);
        println!("{:?}", r);
        assert_eq!(None, r);
    }

    #[test]
    fn read_var() {
        let b_code: Vec<ByteCode<i32, &str>> = vec![
            ByteCode {
                instruction: Instruction::LOAD_VAL(2),
            },
            ByteCode {
                instruction: Instruction::WRITE_VAR("x"),
            },
            ByteCode {
                instruction: Instruction::READ_VAR("x"),
            },
            ByteCode {
                instruction: Instruction::RETURN_VALUE,
            },
        ];
        let r = interpreter(&b_code);

        println!("{:?}", r);

        assert_eq!(Some(2), r);
    }

    #[test]
    fn loop_prog() {
        let b_code: Vec<ByteCode<i32, &str>> = vec![
            ByteCode {
                instruction: Instruction::LOAD_VAL(2),
            },
            ByteCode {
                instruction: Instruction::LOOP(1),
            },
            ByteCode {
                instruction: Instruction::LOAD_VAL(2),
            },
            ByteCode {
                instruction: Instruction::ADD,
            },
            ByteCode {
                instruction: Instruction::IF_NOT_EQUAL,
            },
            ByteCode {
                instruction: Instruction::JUMP(3),
            },
            ByteCode {
                instruction: Instruction::RETURN_VALUE,
            },
        ];
        let r = interpreter(&b_code);

        println!("{:?}", r);

        assert_eq!(Some(6), r);
    }
}
