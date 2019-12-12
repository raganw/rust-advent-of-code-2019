use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();
    let target = 19690720;
    let mut noun = -1;
    let mut verb = -1;
    for n in 0..99 {
        for v in 0..99 {
            if run(contents.to_owned(), n, v) == target {
                noun = n;
                verb = v;
            }
        }
    }
    let result = 100 * noun + verb;
    println!("Result = {}", result);
}

fn parse_program(program: String) -> Vec<i32> {
    program.split(",").map(|s| s.parse().unwrap()).collect()
}

fn print_program(program: Vec<i32>) -> String {
    program.iter().map(i32::to_string).collect::<Vec<String>>().join(",")
}

fn run(program: String, noun: i32, verb: i32) -> i32 {
    let mut codes = parse_program(program);
    codes.splice(1..3, vec![noun, verb]);
    let result = eval_op(codes, 0);
    result.first().unwrap().to_owned()
}

fn eval_op(program: Vec<i32>, pointer: i32) -> Vec<i32> {
    let operation = program.get(pointer as usize).expect("Unable to get operation");
    if 99.eq(operation) {
        return program;
    }
    let param_1 = program.get((pointer + 1) as usize).expect("Unable to get param 1");
    let param_2 = program.get((pointer + 2) as usize).expect("Unable to get param 2");
    let save_location = program.get((pointer + 3) as usize).expect("Unable to get save location");

    let new_value = match operation {
        1 => program.get(*param_1 as usize).expect("") + program.get(*param_2 as usize).expect(""),
        2 => program.get(*param_1 as usize).expect("") * program.get(*param_2 as usize).expect(""),
        _ => panic!("Encountered invalid opcode {}", operation)
    };
    let range = *save_location as usize..((save_location + 1) as usize);
    let mut return_program = program.to_vec();
    return_program.splice(range, vec![new_value]);
    eval_op(return_program, pointer + 4)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_1() {
        let expected = "3500,9,10,70,2,3,11,0,99,30,40,50".to_string();
        let parsed_program = parse_program("1,9,10,3,2,3,11,0,99,30,40,50".to_string());
        let actual = print_program(eval_op(parsed_program, 0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let expected = "2,0,0,0,99".to_string();
        let parsed_program = parse_program("1,0,0,0,99".to_string());
        let actual = print_program(eval_op(parsed_program, 0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_3() {
        let expected = "2,3,0,6,99".to_string();
        let parsed_program = parse_program("2,3,0,3,99".to_string());
        let actual = print_program(eval_op(parsed_program, 0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_4() {
        let expected = "2,4,4,5,99,9801".to_string();
        let parsed_program = parse_program("2,4,4,5,99,0".to_string());
        let actual = print_program(eval_op(parsed_program, 0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_5() {
        let expected = "30,1,1,4,2,5,6,0,99".to_string();
        let parsed_program = parse_program("1,1,1,4,99,5,6,0,99".to_string());
        let actual = print_program(eval_op(parsed_program, 0));
        assert_eq!(actual, expected);
    }
}
