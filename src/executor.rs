use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::io::Result;

pub fn execute(operation: &String, args: &Vec<&String>){
    //create a todo list
    if operation == "create-list"{
        if args.len() == 1{ let _list =  create_list(args[0]);}
        else {
            println!("Wrong command");
            println!("Try \"todo create-list [listname]\" ");
        }
    }
    //show all lists 
    else if operation == "lists"{
        if args.len() == 0{
           let _ = show_all_lists();
        }
        else{
            println!("Invalid command");
            println!("Did you mean \"todo lists\" ?");
        }
    }
    //show selected todo list
    else if operation == "list"{
        if args.len() == 0
        {
            let _ = show_selected_list();
        }
        else{
            println!("Invalid command");
            println!("Did you mean \"todo list\" ?");
        }
    }
    //select a todo list
    else if operation == "set"{
        if args.len() == 1{
           select_list(args[0]) 
        }
        else{
            println!("Invalid command");
            println!("Did you mean \"todo set [list name]\" ?");
        }
    }
    else if operation == "add"{
        let _ = add(args);
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
    let mut _file = File::create(&path)?;
    let _ = fs::write(".current", file_name);
    println!("Todo list \"{}\" created!", file_name);
    }
    Ok(())
}


/// Show all existing todo lists
pub fn show_all_lists(){
    let paths = fs::read_dir("todos").unwrap();
    let mut total_lists = 0;
    for path in paths{
        total_lists += 1;
        println!("{}", path.unwrap().file_name().display());
    }
    if total_lists == 0{
        println!("No list exists");
        println!("Create a list using \"todo create-list [listname]\"");
    }
    else{
        println!("Total lists: {}" ,total_lists); 
    }
}

/// Show selected list 
pub fn show_selected_list(){
    if let Ok(selected_list) = fs::read_to_string(".current"){
        if selected_list == ""{
            println!("No list selected");
        }
        else{
            println!("{}", selected_list);
        }
    }
}

///Select a todo
///the name of the selected todo list is stored in .current 
pub fn select_list(list_name: &String) {
    let mut does_exist = false;
    let paths = fs::read_dir("todos").unwrap();
    let list = format!("{}.md", list_name);
    for path in paths{
        if *list == *path.unwrap().file_name(){
            does_exist = true;
            break;
        }
    } 

    if does_exist {
        let _ = fs::write(".current", list_name);
        println!("Selected todo list: {}", list_name);
    }
    else{
        println!("Todolist \"{}\" does not exist.", list_name);
    }
}


/// Add a todo
pub fn add(todos: &Vec<&String>) -> std::io::Result<()> {
    let list_file_name = fs::read_to_string(".current").unwrap();
    let path = format!("todos/{}.md", list_file_name);
    let mut list = File::options().append(true).open(&path)?;
    for todo in todos{
        let t = format!("[ ] {}", todo);
        writeln!(&mut list, "{}",t)?;
    }
    Ok(())
}