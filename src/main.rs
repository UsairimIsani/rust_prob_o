use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Mul},
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
    LOOP(R),
    JUMP,
    IF_NOT_EQUAL(T),
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
where
    // Solved an issue regarding pushing to vec. https://stackoverflow.com/questions/37647248/mismatched-types-when-returning-the-result-of-adding-two-generics
    T: std::fmt::Debug + Default + Add<Output = T> + Mul<Output = T> + Copy,
    R: std::fmt::Debug + Default + Eq + Hash + Copy,
{
    // A Program Counter to keep track of current execution.
    let mut prog_count = 0;

    // Stack to store temp vars.
    let mut stack: Vec<T> = Vec::new();

    // A look up table to keep track of variables
    let mut map: HashMap<R, T> = HashMap::new();

    let mut r = None;

    while prog_count < byte_code.len() {
        // Advance the program counter by
        let mut adv_pc = 1;

        match byte_code[prog_count].instruction {
            Instruction::LOAD_VAL(val) => {
                // Put temp var into stack
                stack.push(val);
            }
            Instruction::WRITE_VAR(key) => {
                // write to the lookup table from the stack
                let val = stack.pop().unwrap();
                map.insert(key, val);
                println!("Map : {:?}", map);
            }

            Instruction::READ_VAR(key) => {
                // read from the map and push it to the stack
                let val = map.get(&key).unwrap();
                stack.push(val.clone());
            }
            Instruction::ADD => {
                // Perform Operation 'ADD'on 'a' and temp val in the stack
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
            Instruction::LOOP(key) => {
                // map.insert("limit", prog_count);
                // map.insert(key, prog_count);
            }
            Instruction::JUMP => {
                // let loop_index = map.get(key).unwrap();
                // prog_count = loop_index as us;
            }
            Instruction::IF_NOT_EQUAL(limit) => {}
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
}
