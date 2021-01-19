// use std::io;

// struct Student{
//  id: String,
//  name: String,
//  math_marks: i64,
//  english_marks: i64,
//  science_marks: i64
// }

// fn main() {
//  println!("Enter Number of Students");
    
//     let mut no_of_student = String::new();
//     io::stdin().read_line(&mut no_of_student).expect("Failed to get Student Count");
//     println!("NO of Student {}", no_of_student);

//     println!("Please Enter following student Data");

// }

// fn take_student_data{
//  println!("Enter Student ID");
//  let mut id = String::new()::io::stdin().read_line().expect("Failed to get ID")
// }


use std::io;

#[derive(Debug)]
struct Student {
    id: String,
  name: String,
  math_marks: i64,
  english_marks: i64,
  science_marks: i64
}

fn main() {
    println!("Enter Number of Student");
    let mut number_of_student = String::new();
    
    
    io::stdin().read_line(&mut number_of_student).expect("Failed to get student count");
    let number_of_student: usize = number_of_student.trim().parse::<usize>().unwrap();
    println!("number of students {}", number_of_student);


    let mut students = Vec::new();

    for _number in 1..number_of_student {
        let mut id = String::new();
        let mut name = String::new();
        let mut science_marks = String::new();
        let mut math_marks = String::new();
        let mut english_marks = String::new();

     
        println!("Enter student id");
        io::stdin().read_line(&mut id).expect("unable to read id");

        println!("Enter student name");
        io::stdin().read_line(&mut name).expect("unable to read name");

        println!("Ender student science marks");
        io::stdin().read_line(&mut science_marks).expect("unable to read physics marks");
        let science_marks: i64 = science_marks.trim().parse::<i64>().unwrap();

        println!("Ender student math marks");
        io::stdin().read_line(&mut math_marks).expect("unable to read math marks");
        let math_marks: i64 = math_marks.trim().parse::<i64>().unwrap();

        println!("Ender student physics marks");
        io::stdin().read_line(&mut english_marks).expect("unable to read english marks");
        let english_marks: i64 = english_marks.trim().parse::<i64>().unwrap();

        let student = Student{
            id: id,
            name: name,
            science_marks: science_marks,
            math_marks: math_marks,
            english_marks: english_marks,
        };

        students.push(student)
    }

    loop {
        println!("Enter your choice");
        println!("1) Sort by Name");
        println!("2) Sort by Total");
        println!("3) Exit");

        let mut user_option = String::new();
        io::stdin().read_line(&mut user_option).expect("Failed to get choice");
        let user_option: usize = user_option.trim().parse::<usize>().unwrap();
        
        if user_option == 1 {
            students.sort_by_key(|student| student.name.clone());
        } else if user_option == 2 {
            students.sort_by_key(|student| {student.science_marks + student.math_marks + student.english_marks});
        } else {
            break;
        }
        
        println!("================= Result ==================");
        for student in students.iter() {
            println!("{:?}", student);
        }
    }
}



fn test_demo {
    println!("Hello from ")
}
