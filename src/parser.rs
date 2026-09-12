pub fn parse<'a>(input: &'a Vec<String>, operation: &mut String, args_vec: &mut Vec<&'a String>) { 
    let len = input.len();

    operation.push_str(&input[1]);
    // index[0] - value not needed, index [1] - commands, index[2] - args
    if len > 2{ for i in 2..len{ args_vec.push(&input[i]); } }
}
