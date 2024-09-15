use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    read_from_vector();
    enum_and_vec();
    string_practice();
    hashmap();

    
}

fn create_and_update_vector() -> Vec<i32>{
    let mut vector = vec![1,2,3];
    vector.push(2);
    println!("{:?}", vector);
    vector

}

fn read_from_vector(){
    let vector=create_and_update_vector();
    let vector_value=&vector.get(10);
    println!("{:?}", vector_value);

}

fn enum_and_vec(){
    #[derive(Debug)]
    enum Spreadsheet{
        Number(i32), Word(String), Float(f64)
    }

    let enum_vector= vec![Spreadsheet::Number(1),
    Spreadsheet::Float(2.0),
    Spreadsheet::Word(String::from("Hello"))];
    println!("{:?}", enum_vector);
}

fn string_practice(){
    let mut s= String::from("Hello");
    let s2=String::from("world");
    let s3= s.push_str(&s2);
    println!("{:?}", s3);
}

fn hashmap(){

    let mut scores= HashMap::new();
    scores.insert("Team A".to_string(), 10);
    scores.insert(String::from("Team B"), 20);
    //println!("{:?}", scores.get("Team B"));

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }
    
}