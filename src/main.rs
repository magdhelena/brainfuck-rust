use std::{
  fs,
  io::{self, Read, Write},
  num::Wrapping,
};

fn main() {
  let arguments: Vec<_> = std::env::args().collect();
  let brainfuck_string = if arguments.get(1).expect("Must provide at least 1 argument") == "-f" {
    let file_path = arguments.get(2).expect("Must provide a file path");
    fs::read_to_string(file_path).expect("Should have been able to read the file")
  } else {
    arguments.get(1).expect("No brainfuck given").clone()
  };
  let brainfuck_bytes = brainfuck_string.as_bytes();
  let mut state = State {
    data_pointer: 0,
    memory: [Wrapping(0); 30000],
    instruction_pointer: 0,
    brackets: vec![0usize; 0],
  };

  while state.instruction_pointer < brainfuck_bytes.len() {
    if let Err(err) = execute_instruction(brainfuck_bytes, &mut state) {
      println!("{err}");
      debug(
        &state.instruction_pointer,
        &state.memory,
        &state.data_pointer,
        &state.brackets,
      );
      break;
    }
  }
}
struct State {
  instruction_pointer: usize,
  memory: [Wrapping<u8>; 30000],
  data_pointer: usize,
  brackets: Vec<usize>,
}

fn execute_instruction(brainfuck_bytes: &[u8], state: &mut State) -> Result<(), &'static str> {
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
        return Err("Memory range exceeded");
      }
    }
    b'<' => *data_pointer = data_pointer.checked_sub(1).ok_or("Memory range exceeded")?,
    b'+' => *cell += 1,
    b'-' => *cell -= 1,
    b'.' => {
      io::stdout().write(&[cell.0]).map_err(|_| "Output error")?;
    }
    b',' => {
      io::stdout().flush().map_err(|_| "Flush error")?;
      let mut buffer = [0u8; 1];
      io::stdin()
        .read_exact(&mut buffer)
        .map_err(|_| "Input error")?;
      cell.0 = buffer[0]
    }
    b'[' => {
      if cell.0 == 0 {
        jump_to_matching_bracket(brainfuck_bytes, instruction_pointer);
      } else {
        brackets.push(*instruction_pointer);
      }
    }
    b']' => {
      if cell.0 == 0 {
        brackets.pop();
      } else {
        *instruction_pointer = *brackets.last().ok_or("Syntax error")?
      }
    }
    b'#' => debug(instruction_pointer, memory, data_pointer, brackets),
    _ => {}
  }
  *instruction_pointer += 1;
  Ok(())
}

fn debug(
  instruction_pointer: &usize,
  memory: &[Wrapping<u8>; 30000],
  data_pointer: &usize,
  brackets: &Vec<usize>,
) {
  eprintln!(
    "\nState:\ninstruction_pointer: {}\nmemory: {:?}\ndata_pointer: {}\nbrackets: {:?}\n",
    instruction_pointer,
    &memory[..=memory
      .iter()
      .enumerate()
      .rev()
      .find(|(_i, value)| value.0 != 0)
      .map_or(0, |(i, _)| i)],
    data_pointer,
    brackets
  )
}

fn jump_to_matching_bracket(brainfuck_bytes: &[u8], instruction_pointer: &mut usize) {
  let mut accum = 0;
  for (i, value) in brainfuck_bytes[*instruction_pointer..].iter().enumerate() {
    if *value == b']' {
      accum -= 1;
    };
    if *value == b'[' {
      accum += 1;
    }
    if accum == 0 {
      *instruction_pointer += i;
      return;
    }
  }
  panic!("Syntax error");
}
