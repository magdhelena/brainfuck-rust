use std::{
  io::{self, Read, Write},
  num::Wrapping,
  ops::ControlFlow::{Break, Continue},
};

fn main() {
  let brainfuck_string = std::env::args().nth(1).expect("No brainfuck given");
  let brainfuck_bytes = brainfuck_string.as_bytes();
  let mut state = State {
    data_pointer: 0,
    memory: [Wrapping(0); 30000],
    instruction_pointer: 0,
    brackets: vec![0usize; 0],
  };

  while state.instruction_pointer < brainfuck_bytes.len() {
    execute_instruction(brainfuck_bytes, &mut state);
  }
}
struct State {
  instruction_pointer: usize,
  memory: [Wrapping<u8>; 30000],
  data_pointer: usize,
  brackets: Vec<usize>,
}

fn execute_instruction(brainfuck_bytes: &[u8], state: &mut State) {
  let State {
    instruction_pointer,
    memory,
    data_pointer,
    brackets,
  } = state;
  let char = brainfuck_bytes[*instruction_pointer];
  let cell = &mut memory[*data_pointer];

  match char {
    b'>' => {
      *data_pointer += 1;
      if *data_pointer >= memory.len().try_into().unwrap() {
        panic!("Memory range exceeded")
      }
    }
    b'<' => {
      *data_pointer = data_pointer.checked_sub(1).expect("Memory range exceeded");
    }
    b'+' => *cell += 1,
    b'-' => *cell -= 1,
    b'.' => {
      io::stdout().write(&[cell.0]).expect("Output error");
    }
    b',' => {
      let mut buffer = [0u8; 1];
      io::stdin().read_exact(&mut buffer).expect("Input error");
      cell.0 = buffer[0]
    }
    b'[' => {
      brackets.push(*instruction_pointer);
      brainfuck_bytes[*instruction_pointer..brainfuck_bytes.len()]
        .into_iter()
        .enumerate()
        .try_fold(0, |accum, (i, value)| {
          let mut next = accum;
          if *value == b']' {
            next -= 1;
          };
          if *value == b'[' {
            next += 1;
          }
          if next == 0 {
            if cell.0 == 0 {
              *instruction_pointer += i
            }
              Break(next)
          } else {
              Continue(next)
          }
      });
    }
    b']' => {
      if cell.0 == 0 {
        brackets.pop();
      } else {
        *instruction_pointer = *brackets.last().expect("Syntax error");
      }
    }
    _ => {}
  }
  *instruction_pointer += 1;
}
