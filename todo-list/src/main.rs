use chrono::{DateTime, Local};
use std::io;

#[derive(Debug)]
enum Stage{
    Pending,
    Active,
    Complete
}
#[derive(Debug)]
enum Task{
    Task {title: String, stage: Stage, date_time: DateTime<Local>, end_time: DateTime<Local>},
}
#[derive(Debug)]
struct ToDoList{
    title: String,
    tasks: Vec<Task>,
}


fn create_list(title: String, task: String) -> ToDoList{
    ToDoList{title: title, tasks: vec![Task::Task {title: task, stage: Stage::Pending, date_time: Local::now(), end_time: Local::now()}]}
}
enum Mode{
    On,
    Off
}
fn list_mode() -> Mode{
    let mode = Mode::On;

    println!("What should we name the list sugar stick?");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    println!("What is our first task sugar stick?");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    let mut list = create_list(title.trim().to_string(), task.trim().to_string());

    println!("The list has been created! \n {:?}", list);

    loop{
        println!("\nWanna add some more sexy tasks? (y/n)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        match answer.trim() {
            "y" => {println!("Again, Rustybear!"); create_task(&mut list)},
            "n" => {println!("Well good luck then, RustyBear."); break},
            _ => println!("That is not an answer sweetie."),
        };
    }

    Mode::Off
}

fn create_task(list: &mut ToDoList){
    println!("\nWhat is the next task sugar stick?");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    list.tasks.push(Task::Task {title: title.trim().to_string(), stage: Stage::Active, date_time: Local::now(), end_time: Local::now()});
    println!("{:?}", list)
}

fn main() {
    println!("Hello, Rustybeer!");
    println!("Welcome to the Tasker's To Do List \n");

    println!("Press [ENTER] to continue.");
    let mut pause = String::new();
    let _ = io::stdin().read_line(&mut pause);

    println!("Hottie. Do you want to create a to do list? (y/n)");
    let mut answer = String::new();
    let _ = io::stdin().read_line(&mut answer).expect("That's doesn't work dumbass!");

    let _mode = match answer.trim() {
        "y" => {println!("Let's go then, Rustybear...");Mode::On},
        "n" => {
            println!("Well good luck then, RustyBear.");
            return;
        },
        _ => {
            println!("That is not an answer honey.");
            return;
        },
    };

    while let Mode::On = list_mode(){
        println!("--- Generating another iteration ---");
    }

    println!("Goodbye! Thanks for using me, RustyBeer. ilu < 3 ❤️.");
}
