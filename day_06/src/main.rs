use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, Read};
use std::error::Error;

struct FileCharIterator {
    reader: BufReader<File>
}

impl FileCharIterator {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(FileCharIterator { reader })
    }
}

impl Iterator for FileCharIterator {
    type Item = io::Result<char>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(Ok(buf[0] as char)),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum CellState {
    Unvisited,
    Visited,
    Obstacle,
}

struct State {
    pub position: (usize, usize),
    pub direction: Direction,
    pub map: Vec<Vec<CellState>>,
}

impl State {
    pub fn new() -> State {
        State {
            position: (0, 0),
            direction: Direction::Up,
            map: Vec::new(),
        }
    }
}

fn parse_input(chars: FileCharIterator) -> Result<State, Box<dyn Error>> {
    let mut state = State::new();
    let mut row = Vec::new();
    let mut i = 0;
    let mut j = 0;
    for char in chars.flatten() {
        match char {
            '.' => {
                row.push(CellState::Unvisited);
                j += 1;
            },
            '#' => {
                row.push(CellState::Obstacle);
                j += 1;
            },
            '^' => {
                row.push(CellState::Visited);
                state.position = (i, j);
                j += 1;
            },
            '\n' => {
                state.map.push(row);
                row = Vec::new();
                i += 1;
                j = 0;
            },
            _ => {
                return Err("invalid character".into());
            },
        }
    }
    Ok(state)
}

fn check_bounds(state: &State) -> bool {
    let (i, j) = state.position;
    let height = state.map.len();
    if height == 0 {
        return false;
    }
    let width = state.map[0].len();
    if width == 0 {
        return false;
    }
    match state.direction {
        Direction::Up => i != 0,
        Direction::Down => i < height - 1,
        Direction::Left => j != 0,
        Direction::Right => j < width - 1,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let chars = FileCharIterator::new("./data/map.txt")?;
    let mut state = parse_input(chars)?;
    let mut num_positions = 1;
    loop {
        let (i, j) = state.position;
        match state.direction {
            Direction::Up => {
                if !check_bounds(&state) {
                    break;
                }
                match state.map[i-1][j] {
                    CellState::Obstacle => {
                        state.direction = Direction::Right;
                    },
                    CellState::Unvisited => {
                        state.map[i-1][j] = CellState::Visited;
                        state.position = (i-1, j);
                        num_positions += 1;
                    },
                    CellState::Visited => {
                        state.position = (i-1, j);
                    },
                }
            },
            Direction::Down => {
                if !check_bounds(&state) {
                    break;
                }
                match state.map[i+1][j] {
                    CellState::Obstacle => {
                        state.direction = Direction::Left;
                    },
                    CellState::Unvisited => {
                        state.map[i+1][j] = CellState::Visited;
                        state.position = (i+1, j);
                        num_positions += 1;
                    },
                    CellState::Visited => {
                        state.position = (i+1, j);
                    },
                }
            },
            Direction::Left => {
                if !check_bounds(&state) {
                    break;
                }
                match state.map[i][j-1] {
                    CellState::Obstacle => {
                        state.direction = Direction::Up;
                    },
                    CellState::Unvisited => {
                        state.map[i][j-1] = CellState::Visited;
                        state.position = (i, j-1);
                        num_positions += 1;
                    },
                    CellState::Visited => {
                        state.position = (i, j-1);
                    },
                }
            },
            Direction::Right => {
                if !check_bounds(&state) {
                    break;
                }
                match state.map[i][j+1] {
                    CellState::Obstacle => {
                        state.direction = Direction::Down;
                    },
                    CellState::Unvisited => {
                        state.map[i][j+1] = CellState::Visited;
                        state.position = (i, j+1);
                        num_positions += 1;
                    },
                    CellState::Visited => {
                        state.position = (i, j+1);
                    },
                }
            },
        }
    }
    println!("The guard visits {} position(s) on the map.", num_positions);
    Ok(())
}
