use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
//    let args: Vec<String> = env::args().collect();
//    let filename = &args[1];
//    let contents = fs::read_to_string(filename).unwrap().trim().to_string();
    println!("Hello, world!");
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinate(i32, i32);

#[derive(Hash, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction_from_string(input: String) -> Direction {
    match input.as_ref() {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Not a valid direction")
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Operation {
    direction: Direction,
    length: i32,
}

fn parse_operation(op_str: Command) -> Operation {
    match op_str.split_at(1) {
        (x, y) => Operation {
            direction: direction_from_string(x.to_string()),
            length: y.parse().unwrap(),
        }
    }
}

type Command = String;
type CommandLine = Vec<Command>;
type ParsedInput = Vec<CommandLine>;

fn distance_to_closest_intersection(input: String) -> i32 {
    let lines: ParsedInput = parse_input(input);
    let line_1: CommandLine = lines.get(0).unwrap().to_owned();
    let wired_coordinates_1: HashSet<Coordinate> = get_wired_coordinates(line_1);
    let line_2: CommandLine = lines.get(1).unwrap().to_owned();
    0
}

fn parse_input(input: String) -> ParsedInput {
    input.lines().map(|s| s.split(',').map(str::to_string).collect()).collect()
}

fn get_wired_coordinates(input: Vec<String>) -> HashSet<Coordinate> {
    for operation in input {}
    let mut coordinates: HashSet<Coordinate> = HashSet::new();
    coordinates.insert(Coordinate(0, 0));
    coordinates
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_parses_operations() {
        let input = "R75".to_string();
        let expected = Operation { direction: Direction::Right, length: 75 };
        let actual = parse_operation(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_parses_input() {
        let expected = vec![vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]];
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
        let actual = parse_input(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_gets_the_correct_solution_for_example_1() {
        let expected = 6;
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
        let actual = distance_to_closest_intersection(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_gets_the_correct_solution_for_example_2() {
        let expected = 159;
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83".to_string();
        let actual = distance_to_closest_intersection(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_gets_the_correct_solution_for_example_3() {
        let expected = 135;
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();
        let actual = distance_to_closest_intersection(input);
        assert_eq!(actual, expected);
    }
}
