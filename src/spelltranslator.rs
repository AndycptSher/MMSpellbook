use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::COMPONENT_TO_FUNCTION_MAP;

const FUNCTION_NAME_SIZE: usize = 30;

const ON_READY_NAME: &'static str = "when_created:";
const PROCESS_NAME: &'static str = "repeat:";

// ToDo: Rework parse_spell to be more robust. Go step by step, ensuring that the spell_code is in the correct format.
// ToDo: Add in end of scope to the end of the if statement when indent goes down.
pub fn parse_spell(spell_code: &str) -> Result<Vec<u64>, &'static str> {
    let mut instructions: Vec<u64> = vec![];
    let mut character_accumulator = String::new();
    let mut expected_indent: u32 = 0;
    for line in spell_code.lines() {
        let trimmed_line = line.trim();
        for character in line.chars() {
            if character == '(' {
                instructions.extend(parse_component(trimmed_line)?);
                break;
            } else if character == ':' {
                let section: u64 = match trimmed_line {
                    ON_READY_NAME => 500,
                    PROCESS_NAME => 501,
                    _ => return Err("Invalid section name")
                };
                instructions.push(section)
            } else if character_accumulator == "if" {
                instructions.push(400); // Indicates if statement
                instructions.extend(parse_logic(&trimmed_line[3..])?);
                instructions.push(0); // Indicates end of scope for logic
                expected_indent += 1;
            }
            character_accumulator.push(character);
        }
        character_accumulator.clear();
    }
    return Ok(instructions)
}

fn get_precedence(operator: &str) -> u64 {
    match operator {
        "(" | ")" => 0,
        "and" | "or" | "xor" => 1,
        ">" | "<" | "=" | "==" => 2,
        "+" | "-" => 3,
        "x" | "*" | "/" => 4,
        "^" => 5,
        "not" => 6,
        _ => panic!("Not valid operator")
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right
}

fn get_associative_direction(operator: &str) -> Direction {
    match operator {
        "and" | "or" | "xor" | "+" | "-" | "x" | "*" | "/" | "^" | "=" | "==" | ">" | "<" => Direction::Left,
        "not" => Direction::Right,
        _ => panic!("Not valid operator")
    }
}

fn parse_logic(conditions: &str) -> Result<Vec<u64>, &'static str> {
    // Uses the Shunting Yard Algorithm to turn player written infix code into executeable postfix (RPN) code
    let mut holding_stack: Vec<&str> = vec![];
    let mut output: Vec<&str> = vec![];
    for condition in conditions.split_whitespace() {
        match condition {
            "and" | "or" | "xor" | "+" | "-" | "x" | "*" | "/" | "^" | "=" | "==" | ">" | "<" | "not" => {
                while true {
                    if let Some(&operator) = holding_stack.last() {
                        if get_precedence(operator) < get_precedence(condition) {
                            holding_stack.push(condition);
                            break;
                        } else if get_precedence(operator) > get_precedence(condition) {
                            output.push(holding_stack.pop().expect("Shouldn't be possible to reach"));
                        } else { // Must be equal in this case
                            if get_associative_direction(operator) == Direction::Left {
                                output.push(holding_stack.pop().expect("Shouldn't be possible to reach"));
                            } else {
                                output.push(operator);
                            }
                        }
                    } else {
                        holding_stack.push(condition);
                        break;
                    }
                }
            },
            "(" => {
                holding_stack.push(condition)
            }
            ")" => {
                let mut operator = holding_stack.pop().ok_or("Expected opening bracket")?;
                while operator != "(" {
                    output.push(operator);
                    operator = holding_stack.pop().ok_or("Expected opening bracket")?;
                }
            }
            possible_num => {
                if let Ok(_) = possible_num.parse::<f64>() {
                    output.push(possible_num);
                } else {
                    return Err("Invalid condition")
                }
            }
        }
    }
    let mut bit_conditions: Vec<u64> = vec![];
    for condition in output {
        match condition {
            "and" => bit_conditions.push(200),
            "or" => bit_conditions.push(201),
            "not" => bit_conditions.push(202),
            "xor" => bit_conditions.push(203),
            "==" | "=" => bit_conditions.push(300),
            ">" => bit_conditions.push(301),
            "<" => bit_conditions.push(302),
            "x" | "*" => bit_conditions.push(600),
            "/" => bit_conditions.push(601),
            "+" => bit_conditions.push(602),
            "-" => bit_conditions.push(603),
            "^" => bit_conditions.push(604),
            possible_num => {
                if let Ok(num) = possible_num.parse::<f64>() {
                    bit_conditions.push(102); // Indicates number literal
                    bit_conditions.push(num.to_bits());
                } else {
                    return Err("Couldn't translate condition into bytecode")
                }
            }
        }
    }
    return Ok(bit_conditions);
}

fn parse_component(component_call: &str) -> Result<Vec<u64>, &'static str> {
    let mut component_vec: Vec<u64> = vec![103];
    let (component_name, parameters) = parse_component_string(component_call)?;
    let component_num = match get_component_num(&component_name) {
        Some(num) => num,
        None => return Err("Invalid component: mapping doesn't exist")
    };
    component_vec.push(component_num);
    for parameter in parameters {
        component_vec.push(parameter.to_bits())
    }
    return Ok(component_vec)
}

fn pad_component_name(component_name: &str) -> [Option<char>; FUNCTION_NAME_SIZE] {
    let mut padded_name = [None; FUNCTION_NAME_SIZE];
    for (index, character) in component_name.chars().take(FUNCTION_NAME_SIZE).enumerate() {
        padded_name[index] = Some(character);
    }
    padded_name
}

fn decode_component_name(padded_name: &[Option<char>; FUNCTION_NAME_SIZE]) -> String {
    padded_name.iter()
    .filter_map(|&character| character)
    .collect()
}

lazy_static! {
    static ref COMPONENT_TO_NUM_MAP: HashMap<[Option<char>; FUNCTION_NAME_SIZE], u64> = {
        let mut component_map = HashMap::new();
        component_map.insert(pad_component_name("give_velocity"), 0);
        component_map
    };
}

lazy_static! {
    static ref TEXT_TO_OPCODE_MAP: HashMap<[Option<char>; FUNCTION_NAME_SIZE], u64> = {
        let mut component_map = HashMap::new();
        component_map.insert(pad_component_name("give_velocity"), 0);
        component_map
    };
}

pub fn get_component_num(component_name: &str) -> Option<u64> {
    COMPONENT_TO_NUM_MAP.get(&pad_component_name(component_name)).cloned()
}

fn parse_component_string(component_call: &str) -> Result<(String, Vec<Parameter>), &'static str> {
    if component_call.chars().last() != Some(')') {
        return Err("Invalid component: Must end with close bracket");
    }

    let mut component_name = String::new();
    let mut character_count = 0;
    let mut found_opening_bracket = false;

    // Looping through component_call to get component_name
    for character in component_call.chars() {
        if character == '(' {
            if character == ' '{
                continue;
            } else if character == ',' {
                return Err("Invalid component: Must begin with letters")
            }
            found_opening_bracket = true;
            break;
            // Checking if character is alphabetic if not an open bracket.
        } else if !character.is_alphabetic() && character != '_' {
            return Err("Invalid component: Name must be made up of letters")
        }

        character_count += 1;
        component_name.push(character);
    }

    // There needs to be an opening bracket, if there is none, returns error
    if found_opening_bracket == false {
        return Err("Invalid component: Must have opening bracket")
    }

    // This line gets the parameters as a string and puts it into the variable parameters_string
    if let Some(parameters_string) = component_call.get(character_count + 1..component_call.len() - 1) {
        let parameters = collect_parameters(parameters_string, &component_name)?;
        return Ok((component_name, parameters))
    } else {
        return Err("Invalid component: Parameters not valid")
    }
}

enum Parameter {
    Integer(u64),
    Float(f64),
    Boolean(bool)
}

impl Parameter {
    fn to_bits(&self) -> u64 {
        match *self {
            Parameter::Integer(int) => int,
            Parameter::Float(float) => float.to_bits(),
            Parameter::Boolean(boolean) => match boolean {
                true => 100,
                false => 101
            }
        }
    }
}

fn collect_parameters(parameters_string: &str, component_name: &str) -> Result<Vec<Parameter>, &'static str> {
    let mut parameter = String::new();
    let mut parameters: Vec<Parameter> = vec![];

    // ToDo: Use COMPONENT_TO_FUNCTION_MAP to find expected parameter type and attempt to convert it

    let mut index = 0;

    if let Some((_, encoded_types)) = COMPONENT_TO_FUNCTION_MAP.get(&get_component_num(component_name).expect("Expected component")) {
        let encoded_types: &[u64] = encoded_types;
        for character in parameters_string.chars() {
            if character == ',' {
                if parameter.is_empty() {
                    return Err("Invalid parameters: Must have value before bracket")
                }

                if index >= encoded_types.len() {
                    return Err("Invalid parameters: More parameters than expected types");
                }

                // Adding parameter to parameters vector
                parameters.push(parse_parameter(&parameter, encoded_types[index])?);
                index += 1;

                // Clear parameter string so next one can be recorded
                parameter.clear()

            } else {
                parameter.push(character)
            }
        }

        // Adding last parameter
        if !parameter.is_empty() {
            if index >= encoded_types.len() {
                return Err("Invalid parameters: More parameters than expected types");
            }
            parameters.push(parse_parameter(&parameter, encoded_types[index])?);
        }
    } else {
        panic!("Expected component mapping")
    }

    return Ok(parameters)
}

fn parse_parameter(parameter_string: &str, parameter_type: u64) -> Result<Parameter, &'static str> {
    let trimmed_parameter_string = parameter_string.trim();
    match parameter_type {
        0 => Ok(Parameter::Integer(trimmed_parameter_string.parse::<u64>().expect("Couldn't parse parameter: should be integer"))),
        1 => Ok(Parameter::Float(trimmed_parameter_string.parse::<f64>().expect("Couldn't parse parameter: should be float"))),
        2 => Ok(Parameter::Boolean(trimmed_parameter_string.parse::<bool>().expect("Couldn't parse parameter: should be boolean"))),
        _ => Err("Invalid parameters: parameter doesn't match expected type")
    }
}
