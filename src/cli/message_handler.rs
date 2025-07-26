use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

pub fn ask_for_status() {
    println!("Add the status for this task:");
    println!("  (1) In Progress");
    println!("  (2) Done");
    println!("  (3) Cancelled");
    println!("  (4) Blocked");
    println!();
    print_dashed_line();
}

fn print_dashed_line() {
    println!("--------------------------------------------");
}

pub fn clear_and_reset() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All), // clear entire screen
        MoveTo(0, 0)           // move cursor to top-left corner
    )
    .unwrap();
}

pub fn present_commands_prompt() {
    println!("Please enter the command initial letter to execute:");
    println!();
    println!("  [A]dd     - Add a new task with status and description");
    println!("  [D]elete  - Delete an existing task");
    println!("  [U]pdate  - Update a task");
    println!("  [L]ist    - List all tasks");
    println!();
    print!("Your choice: ");
    print_dashed_line();
}
