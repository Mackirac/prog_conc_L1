use std::sync::{ Mutex, Condvar, Arc };
use std::fs::File;
use std::io::Write;
use std::convert::TryInto;

struct Board([u8; 8]);

impl std::ops::Deref for Board {
    type Target = [u8; 8];
    fn deref(&self) -> &Self::Target { &self.0 }
}

use std::fmt::{ self, Debug, Formatter };
impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

struct SharedState {
    buffer: Vec<Board>,
    generating: bool
}

struct SharedMemo {
    state: Mutex<SharedState>,
    notifier: Condvar
}

fn generate_permutations(
    current_solution: Vec<u8>,
    remaining_options: Vec<u8>,
    shared_memory: Arc<SharedMemo>
) {
    if remaining_options.is_empty() {
        let board = Board(current_solution.as_slice().try_into().unwrap());
        { /* CRITICAL SESSION BEGIN */
            let mut lock = shared_memory.state.lock().unwrap();
            lock.buffer.push(board);
        /* CRITICAL SESSION END */ }
        shared_memory.notifier.notify_all();
    }
    else {
        for idx in 0..remaining_options.len() {
            let mut remaining_options = remaining_options.clone();
            let mut current_solution = current_solution.clone();
            current_solution.push(remaining_options.remove(idx));
            generate_permutations(current_solution, remaining_options, shared_memory.clone());
        }
    }
}

fn producer(shared_memory: Arc<SharedMemo>) {
    generate_permutations(vec![], vec![7, 6, 5, 4, 3, 2, 1, 0], shared_memory.clone());
    { /* CRITICAL SESSION BEGIN */
        let mut lock = shared_memory.state.lock().unwrap();
        lock.generating = false;
    /* CRITICAL SESSION END */ }
    shared_memory.notifier.notify_all();
}

fn test_placement(board: &Board) -> bool {
    let mut main_diagonals = [false; 15];
    let mut secd_diagonals = [false; 15];

    for col in 0..8 {
        let mut diagonal = col + board[col] as usize;
        if main_diagonals[diagonal] { return false; }
        else { main_diagonals[diagonal] = true; }

        diagonal = col + (board[col] as i16 - 7).abs() as usize;
        if secd_diagonals[diagonal] { return false; }
        else { secd_diagonals[diagonal] = true; }
    }

    true
}

fn consumer(shared_memory: Arc<SharedMemo>) -> Vec<Board> {
    let mut solutions = vec![];
    loop {
        // CRITICAL SESSION BEGIN
        let board = {
            let lock = shared_memory.state.lock().unwrap();

            let mut shared_memory = shared_memory.notifier.wait_while(lock,
                |state: &mut SharedState| -> bool {
                    state.buffer.is_empty() &&
                    state.generating
                }
            ).unwrap();

            if shared_memory.buffer.is_empty() && !shared_memory.generating { break; }
            else { shared_memory.buffer.pop().unwrap() }
        };
        // CRITICAL SESSION END

        if test_placement(&board) { solutions.push(board); }
    }
    solutions
}

pub fn q34() {
    let shared_memory = Arc::new(SharedMemo {
        state: Mutex::new(SharedState {
            buffer: vec![],
            generating: true
        }),
        notifier: Condvar::new()
    });

    let p1 = {
        let shared_memory = shared_memory.clone();
        std::thread::spawn(move || {
            producer(shared_memory);
        })
    };

    let p2 = {
        std::thread::spawn(move || {
            consumer(shared_memory)
        })
    };

    p1.join().unwrap();
    let solutions = p2.join().unwrap();

    let mut output = format!("Number of solutions: {}\n\n", solutions.len());
    solutions.iter().enumerate().for_each(|(idx, line)| {
        output.push_str(&format!("Solução {:2}: {:?}\n", idx+1, line));
    });
    let mut file = File::create("q32.txt").unwrap();
    file.write_all(&output.into_bytes()).unwrap();
}
