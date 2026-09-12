use std::fs::File;
use std::path::PathBuf;
use std::io::Result;

pub fn execute(operation: &String, args: &Vec<&String>){
    println!("created todo!");
    //create a todo list
    if operation == "create-list"{
        if args.len() == 1{ let mut list =  create_list(args[0]);}
        else {
            println!("Wrong command");
            println!("Try \"todo create-list [listname]\" ");
        }
    }
    //show all lists 
    else if operation == "lists"{
        if args.len() == 0{
            show_all_lists();
        }
        else{
            println!("Invalid command");
            println!("Did you mean \"todo lists\" ?");
        }
    }
    else{
        println!("Invalid command");
        println!("Try: todo --help");
    }
}

/// Create todo lists
pub fn create_list(file_name: &String) -> Result<()>{
    let list: String = format!("{}.md", file_name);
    let mut path = PathBuf::from("todos");
    path.push(list);

    if path.exists(){
        println!("Todo list \"{}\" already exists", file_name);
    }
    else{
    let mut file = File::create(&path)?;
    println!("Todo list \"{}\" created!", file_name);
    }
    Ok(())
}


pub fn show_all_lists(){
    println!("showing all lists here");
}
