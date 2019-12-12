enum ParameterType {
  Position = 0,
  Immediate = 1,
}

impl ParameterType {
  fn from(digit: u8) -> ParameterType {
    match digit {
      1 => ParameterType::Immediate,
      _ => ParameterType::Position,
    }
  }
}

struct Instruction {
  opcode: u8,
  param1: ParameterType,
  param2: ParameterType,
  param3: ParameterType,
}

fn lookup(memory: &[i64], address: usize, parameter_type: ParameterType) -> i64 {
  match parameter_type {
    ParameterType::Position => {
      let lvalue = memory[address] as usize;
      memory[lvalue]
    }
    ParameterType::Immediate => memory[address],
  }
}

fn set(memory: &mut [i64], address: usize, value: i64) {
  let lvalue = memory[address] as usize;
  memory[lvalue] = value;
}

fn get_digits(n: i64) -> [u8; 4] {
  [
    (n / 10000 % 10) as u8,
    (n / 1000 % 10) as u8,
    (n / 100 % 10) as u8,
    (n / 1 % 100) as u8, // opcode uses two digits
  ]
}

fn parse_instruction(value: i64) -> Instruction {
  let [param3, param2, param1, opcode] = get_digits(value);

  Instruction {
    opcode: opcode as u8,
    param1: ParameterType::from(param1),
    param2: ParameterType::from(param2),
    param3: ParameterType::from(param3),
  }
}

pub fn interpret(memory: &mut [i64], answers: &[i64]) -> Vec<i64> {
  let mut ip: usize = 0; // instruction pointer
  let mut input = answers.into_iter();
  let mut output = Vec::<i64>::new();
  loop {
    let Instruction {
      opcode,
      param1,
      param2,
      param3: _param3, // Not yet used, but teased in the description
    } = parse_instruction(memory[ip]);
    match opcode {
      // add
      1 => {
        set(
          memory,
          ip + 3,
          lookup(memory, ip + 1, param1) + lookup(memory, ip + 2, param2),
        );
        ip += 4;
      }
      // multiply
      2 => {
        set(
          memory,
          ip + 3,
          lookup(memory, ip + 1, param1) * lookup(memory, ip + 2, param2),
        );
        ip += 4;
      }
      // read input
      3 => {
        let value = *input.next().expect("Not enough input provided");
        set(memory, ip + 1, value);
        ip += 2;
      }
      // output
      4 => {
        let value = lookup(memory, ip + 1, param1);
        println!("{}", value);
        output.push(value);
        ip += 2;
      }
      // jump-if-true
      5 => {
        if lookup(memory, ip + 1, param1) != 0 {
          ip = lookup(memory, ip + 2, param2) as usize;
        } else {
          ip += 3
        }
      }
      // jump-if-false
      6 => {
        if lookup(memory, ip + 1, param1) == 0 {
          ip = lookup(memory, ip + 2, param2) as usize;
        } else {
          ip += 3
        }
      }
      // less-than
      7 => {
        let lt = lookup(memory, ip + 1, param1) < lookup(memory, ip + 2, param2);
        set(memory, ip + 3, if lt { 1 } else { 0 });
        ip += 4
      }
      // equals
      8 => {
        let eq = lookup(memory, ip + 1, param1) == lookup(memory, ip + 2, param2);
        set(memory, ip + 3, if eq { 1 } else { 0 });
        ip += 4
      }
      // exit
      99 => {
        return output;
        // ip += 1;
      }
      _ => panic!("Unknown opcode {} at address {}", opcode, ip),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exit() {
    let mut memory: &mut [i64] = &mut [99];
    interpret(&mut memory, &[1]);
    assert_eq!(memory, [99]);
  }

  #[test]
  fn test_addition() {
    let mut memory: &mut [i64] = &mut [1, 0, 0, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn test_multiplication() {
    let mut memory: &mut [i64] = &mut [2, 3, 0, 3, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [2, 3, 0, 6, 99]);
  }

  #[test]
  fn test_combination() {
    let mut memory: &mut [i64] = &mut [1, 1, 1, 4, 99, 5, 6, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }

  #[test]
  fn test_read_input() {
    let mut memory: &mut [i64] = &mut [3, 0, 99];
    interpret(&mut memory, &[99]);
    assert_eq!(memory, [99, 0, 99]);
  }

  #[test]
  fn test_print_output() {
    let mut memory: &mut [i64] = &mut [4, 2, 99];
    let output = interpret(&mut memory, &[]);
    assert_eq!(memory, [4, 2, 99]);
    assert_eq!(output, [99]);
  }

  #[test]
  fn test_jump_if_true() {
    let mut memory: &mut [i64] = &mut [1005, 2, 7, 2, 3, 0, 3, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [1005, 2, 7, 2, 3, 0, 3, 99]);
  }

  #[test]
  fn test_jump_if_false() {
    let mut memory: &mut [i64] = &mut [1006, 5, 7, 2, 3, 0, 3, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [1006, 5, 7, 2, 3, 0, 3, 99]);
  }

  #[test]
  fn test_lt_true() {
    let mut memory: &mut [i64] = &mut [7, 1, 2, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [1, 1, 2, 0, 99]);
  }
  #[test]
  fn test_lt_false() {
    let mut memory: &mut [i64] = &mut [7, 2, 2, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [0, 2, 2, 0, 99]);
  }

  #[test]
  fn test_eq_true() {
    let mut memory: &mut [i64] = &mut [1108, 2, 2, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [1, 2, 2, 0, 99]);
  }

  #[test]
  fn test_eq_false() {
    let mut memory: &mut [i64] = &mut [1108, 2, 3, 0, 99];
    interpret(&mut memory, &[]);
    assert_eq!(memory, [0, 2, 3, 0, 99]);
  }

  const COMPARE_8: [i64; 47] = [
    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
    1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
    1, 46, 98, 99,
  ];

  #[test]
  fn test_new_instructions() {
    let mut memory: &mut [i64] = &mut COMPARE_8.clone();
    let output = interpret(&mut memory, &[3]);
    assert_eq!(output, [999]);

    let mut memory: &mut [i64] = &mut COMPARE_8.clone();
    let output = interpret(&mut memory, &[8]);
    assert_eq!(output, [1000]);

    let mut memory: &mut [i64] = &mut COMPARE_8.clone();
    let output = interpret(&mut memory, &[100]);
    assert_eq!(output, [1001]);
  }
}
