use std::collections::HashMap;
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
    // LOOP,
    // JUMP,
    // IF_NOT_EQUAL,
}

impl<T, R> Instruction<T, R> {
    fn parse(s: &str) -> Instruction<T, R> {
        unimplemented!()
    }
}

pub struct ByteCode<T, R> {
    instruction: Instruction<T, R>,
    // arg: Option<T>,
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

// Very simplified Interpreter
fn interpreter<R>(code: &mut Vec<String>) -> Result<Option<i32>> {
    // Track Variables
    let mut stack: Vec<i32> = Vec::new(); // To put temp values
    let mut map: HashMap<&str, i32> = HashMap::new(); // to put variables in

    let fold_r: Result<Option<i32>> = code.iter().fold(Result { val: None }, |mut a, v| {
        // parse into ByteCode
        let b_code = ByteCode::parse(v);

        // Fold value
        let result = match b_code.instruction {
            Instruction::LOAD_VAL(val) => {
                // Put LOAD_VAL into stack
                stack.push(val);
                a
            }
            Instruction::WRITE_VAR(key) => {
                // write to the map from the stack
                let val = stack.pop().unwrap();
                map.insert(key, val);
                a
            }

            Instruction::READ_VAR(key) => {
                // read from the map
                let val = map.get(key).unwrap();
                a.val = Some(Some(val.clone()));
                a
            }
            Instruction::ADD => {
                // Perform Operation 'ADD'on 'a' and temp val in the stack
                let r = if let Some(re) = a.val {
                    let temp = stack.pop().unwrap();
                    Some(re.clone().unwrap() + temp)
                } else {
                    None
                };
                Result { val: Some(r) }
            }
            Instruction::MULTIPLY => {
                // Perform Operation 'MULTIPLY' on 'a' and temp val in the stack
                let r = if let Some(r) = a.val {
                    let temp = stack.pop().unwrap_or_default();
                    Some(r.clone().unwrap() * temp)
                } else {
                    None
                };
                Result { val: Some(r) }
            }
            Instruction::RETURN_VALUE => a,
        };
        result
    });
    fold_r
}
mod live;

fn main() {
    println!("Hello, world!");
}
