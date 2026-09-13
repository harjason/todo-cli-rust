use std::fs::File;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::io::Result;
use std::io::BufReader;

pub fn execute(operation: &String, args: &Vec<&String>){
    if operation == "create-list"{
        if args.len() == 1{ let _list =  create_list(args[0]);}
        else {
            println!("Invalid command");
            println!("Try: todo create-list [listname]");
        }
    }
    else if operation == "lists"{
        if args.len() == 0{
           let _ = show_all_lists();
        }
        else{
            println!("Invalid command");
            println!("Try: todo lists");
        }
    }
    else if operation == "list"{
        if args.len() == 0
        {
            let _ = show_selected_list();
        }
        else{
            println!("Invalid command");
            println!("Try: todo list");
        }
    }
    else if operation == "set"{
        if args.len() == 1{
           select_list(args[0]) 
        }
        else{
            println!("Invalid command");
            println!("Try: todo set [list name]");
        }
    }
    else if operation == "add"{
        if args.len() == 0{
            println!("Invalid command");
            println!("Try: todo add \"todo1\" \"todo2\"");
        }
        else{
            let _ = add(args);
        }
    }
    else if operation == "show"{
        if args.len() == 0 {
            show_todos_of_selected_list();
        }
        else{
            println!("Invalid command");
            println!("Try: todo show");
        }
    }
    else if operation == "complete"{
        if args.len() == 0{
            let _ = complete_todos();
        }
        else{
            println!("Invalid command");
            println!("Try: todo complete");
        }
    }
    else if operation == "delete-list"{
        if args.len() == 0 {
            let _ = delete_todo_list();
        }
        else{
             println!("Invalid command");
            println!("Try: todo delete-list");
        }
    }
    else if operation == "clear-list"{
        if args.len() == 0 {
            let _ = clear_list();
        }
        else{
             println!("Invalid command");
            println!("Try: todo clear-list");
        }
    }
    else if operation == "--help"{
        help();
    }
    else{
        println!("Invalid command");
        println!("Try: todo help");
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
    let current = fs::read_to_string(".current").unwrap();
    let mut total_lists = 0;
    let mut index =1;
    for path in paths{
        total_lists += 1;
        let reff =  &path.unwrap().file_name().into_string().unwrap();
        if &format!("{}.md", current) == reff{
            println!("* - {}", reff);
        }
        else{
            println!("{} - {}", index, reff);
        }
        index += 1;
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
            println!("todos/{}.md", selected_list);
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
        println!("Make sure you didnt include .md file extension in the command.")
    }
}


/// Add a todo(s) to selected todo list
pub fn add(todos: &Vec<&String>) -> std::io::Result<()> {
    let list_file_name = fs::read_to_string(".current").unwrap();
    let path = format!("todos/{}.md", list_file_name);
    let mut list = File::options().append(true).open(&path)?;
    for todo in todos{
        let t = format!("[ ] {}", todo);
        writeln!(&mut list, "{}",t)?;
    }
    show_todos_of_selected_list();
    Ok(())
}


/// Show entire selected todo list
pub fn show_todos_of_selected_list(){
    let current_list_name = fs::read_to_string(".current").unwrap();
    if current_list_name == ""{
        println!("No selected todo list")
    }
    else{
    let path = format!("todos/{}.md", current_list_name);
    println!("list: {}", path);
    let s = fs::read_to_string(path).unwrap();
    println!("{}",s);
    }
}


/// delete a todo list
pub fn delete_todo_list() -> std::io::Result<()>{
    let paths = fs::read_dir("todos").unwrap();
    let paths1 = fs::read_dir("todos").unwrap();

    let mut total_lists = 0;
    let mut number = 1;
    let mut lists = Vec::new();
    for path in paths{
        total_lists += 1;
        println!("{} - {}", number ,&path.unwrap().file_name().display());
        number +=1;
    }
    for path in paths1{
        lists.push(path.unwrap().file_name());
    }
    

    if total_lists == 0{
        println!("No list exists");
    }
    else{
        println!("0 - delete all");
    
     print!("Enter index of list: ");
     io::stdout().flush()?;


     let mut i: usize = usize::MAX;
    //make sure number entered is a non negative number and not a string
     while i == usize::MAX || !(0..total_lists+1).contains(&i) {
        let mut index = String::new();
        io::stdin().read_line(&mut index)?;
        i = index.trim().parse::<usize>().unwrap_or(usize::MAX);
        if i == usize::MAX || !(0..total_lists+1).contains(&i) {
            print!("Enter a valid number: ");
        }
        io::stdout().flush()?;
    }


    let selected_list = format!("{}.md", fs::read_to_string(".current")?);
      if i == 0{
        for  list in   lists{
            let path = format!("todos/{}", list.clone().into_string().unwrap());
            if selected_list == list.into_string().unwrap(){
                fs::write(".current", "")?;
            }
           println!("{} deleted !", path);
           fs::remove_file(path)?;
        }
    }
    // mark individual todo as complete
    else {
        i = i-1;
        let listt = lists.remove(i);
         let path = format!("todos/{}", listt.clone().into_string().unwrap());
         if selected_list == listt.into_string().unwrap(){
                fs::write(".current", "")?;
        }
        println!("{} deleted!", path);
         fs::remove_file(path)?
    }

    }
    Ok(())
}


/// delete all todos of a todo list
pub fn clear_list() ->std::io::Result<()>{
    let paths = fs::read_dir("todos").unwrap();
    let paths1 = fs::read_dir("todos").unwrap();

    let mut total_lists = 0;
    let mut number = 1;
    let mut lists = Vec::new();
    for path in paths{
        total_lists += 1;
        println!("{} - {}", number ,&path.unwrap().file_name().display());
        number +=1;
    }
    for path in paths1{
        lists.push(path.unwrap().file_name());
    }
    

    if total_lists == 0{
        println!("No list exists");
    }
    else{
        println!("0 - clear all");
    
     print!("Enter index of list: ");
     io::stdout().flush()?;


     let mut i: usize = usize::MAX;
    //make sure number entered is a non negative number and not a string
     while i == usize::MAX || !(0..total_lists+1).contains(&i) {
        let mut index = String::new();
        io::stdin().read_line(&mut index)?;
        i = index.trim().parse::<usize>().unwrap_or(usize::MAX);
        if i == usize::MAX || !(0..total_lists+1).contains(&i) {
            print!("Enter a valid number: ");
        }
        io::stdout().flush()?;
    }


    if i == 0 {
        for  list in   lists{
            let path = format!("todos/{}", list.clone().into_string().unwrap());
            println!("{} cleared !", path);
            fs::write(path, "")?;
        }
    }else{
        i = i-1;
        let listt = lists.remove(i);
        let path = format!("todos/{}", listt.clone().into_string().unwrap());
        println!("{} cleared !", path);
        fs::write(path, "")?;
    }
    }
    Ok(())
}


//help
pub fn help(){
    println!("todo - A minimal markdown based todolist for arch linux");
    println!("Commands");
    println!("\tcreate-list               Create a todo list");
    println!("\tlists                     Show all todo lists");
    println!("\tlist                      Show selected todo list");
    println!("\tshow                      Show all todos in selected list");
    println!("\tset                       Select a todo list");
    println!("\tadd                       Add todo(s) to selected list");
    println!("\tcomplete                  Complete todo(s) in selected list");
    println!("\tdelete-list               Delete todo list(s)");
    println!("\tclear-list                Clear todo list(s)");
}

/// mark todos as complete
pub fn complete_todos() -> std::io::Result<()>{
    let current_list_name = fs::read_to_string(".current").unwrap();
    let path = format!("todos/{}.md", current_list_name);

    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    let mut todos: Vec<String> = Vec::new();
    let mut number:i32 = 1;
    for line in reader.lines(){
        let todo = line?;
        println!("{} - {}" , number, todo);
        todos.push(todo);
        number += 1;
    }
    println!("0 - complete all");
    print!("Enter index of todo: ");
    io::stdout().flush()?;

    let mut i: usize = usize::MAX;
    //make sure number entered is a non negative number and not a string
    while i == usize::MAX || !(0..todos.len()+1).contains(&i) {
        let mut index = String::new();
        io::stdin().read_line(&mut index)?;
        i = index.trim().parse::<usize>().unwrap_or(usize::MAX);
        if i == usize::MAX || !(0..todos.len()+1).contains(&i) {
            print!("Enter a valid number: ");
        }
        io::stdout().flush()?;
    }

    // mark all todos as complete
    if i == 0{
        for  todo in &mut  todos{
            todo.remove(1);
           todo.insert(1,'x');
            
            println!("{}", todo);
        }
    }
    // mark individual todo as complete
    else {
        i = i - 1;
        let  todo = &mut todos[i];
        todo.remove(1);
        todo.insert(1,'x');
        for  todo in &mut todos{
            println!("{}", todo);
        }
    }


   let _ = fs::write(&path, "");
    
    let mut list = File::options().append(true).open(&path)?;
    for todo in todos{
        writeln!(&mut list, "{}",todo)?;
    }
    

    Ok(())
}