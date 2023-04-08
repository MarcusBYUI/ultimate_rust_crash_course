pub fn inspect(string: &String){
    println!("{}", if string.len() > 1 {"String is plural"} else {"String is singular"})
}

pub fn change(string: &mut String){
    if !string.ends_with("s") {string.push_str("s")}
}

pub fn eat(string: String) -> bool {
    if string.contains("a") && string.starts_with("b"){
        return true
    }else{
        return false
    }
}

pub fn bedazzle(string: &mut String) {
    *string = String::from("sparkly");
}