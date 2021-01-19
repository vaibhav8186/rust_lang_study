use std::io;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    take_student_data();

}

fn take_student_data() {

    println!("Please Enter Number Of Student");
    let mut no_of_student = String::new();

    io::stdin().read_line(&mut no_of_student).expect("Failed to get No of Student");

    let no_of_student = no_of_student.trim().parse::<i64>().unwrap();

    let mut students = Vec::new();
    let mut student_data = HashMap::new();


    for number in 1..no_of_student {
        let mut id = String::new();
        let mut name = String::new();
        let mut english_marks = String::new();
        let mut science_marks = String::new();
        let mut math_marks = String::new();

        println!("================== Enter Data of {} Student ===============", number);

        println!("Enter ID");
        
        io::stdin().read_line(&mut id).expect("Failed to get student id");
        
        println!("Enter Student Name");

        io::stdin().read_line(&mut name).expect("Failed to get student name");

        println!("Enter english masrks:");
        io::stdin().read_line(&mut english_marks).expect("Failed to read english marks");
        let english_marks: i64 = english_marks.trim().parse::<i64>().unwrap();

        println!("Enter science marks");
        io::stdin().read_line(&mut science_marks).expect("Failed to read science marks");
        let science_marks: i64 = science_marks.trim().parse::<i64>().unwrap();

        println!("Enter maths marks");
        io::stdin().read_line(&mut math_marks).expect("Failed to read maths marks");
        let math_marks: i64 = math_marks.trim().parse::<i64>().unwrap();

        student_data.insert(String::from("id"), id);
        student_data.insert(String::from("name"), name);
        student_data.insert(String::from("english"), english_marks.to_string());
        student_data.insert(String::from("science"), science_marks.to_string());
        student_data.insert(String::from("math"), math_marks.to_string());

        students = Vec::from_iter(student_data.iter());

    }

    for student in &students {
        let vec3 = Vec::from_iter(student.values());
        println!("{:?}", vec3);
    }
}
