use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    id: u64,
    math_mark: u64,
    english_mark: u64,
    science_mark: u64
}

fn main() {
    let mut student_data = get_student_data();
    
    loop {
        println!("Enter Your choice: ");
        println!("1) Sort By Name");
        println!("2) Sort By Total");
        println!("3) Exit");

        let mut sort_by = String::new();
        io::stdin()
            .read_line(&mut sort_by)
            .expect("Failed to get user choice");
        let sort_by: u8 = sort_by.trim().parse().expect("Expect a number");
        
        if sort_by == 1 {
            student_data.sort_by_key(|s| s.name.clone());
        } else if sort_by == 2 {
            student_data.sort_by_key(|s| s.science_mark + s.math_mark + s.english_mark);
        } else {
            break;
        }
        
        for student in &student_data {
             println!("Result: {:?}", student);
        }
    }
    
}

fn get_student_data() -> Vec<Student> {
    
    let mut students = Vec::new();
    println!("Enter Number of Student: ");
    let mut no_of_student = String::new();
    io::stdin()
        .read_line(&mut no_of_student)
        .expect("failed to get student count");
    let no_of_student:u64 = no_of_student.trim().parse().expect("Type a Number");
    for _number in 0..no_of_student {
        println!("Enter Student Id:");
        let mut id = String::new();
       
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to get ID");
    
        let id: u64 = id.trim().parse().expect("Type a number");

        println!("Enter Student Name: ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("failed to get Name");
        
        println!("Enter Maths marks: ");
        let mut math_mark = String::new();
        io::stdin()
            .read_line(&mut math_mark)
            .expect("Failed to get mark");
        let math_mark: u64 = math_mark.trim().parse().expect("Type a number");


        println!("Enter Science marks: ");
        let mut science_mark = String::new();
        io::stdin()
            .read_line(&mut science_mark)
            .expect("Failed to get mark");
        let science_mark: u64 = science_mark.trim().parse().expect("Type a number");

        println!("Enter English marks: ");
        let mut english_mark = String::new();
        io::stdin()
            .read_line(&mut english_mark)
            .expect("Failed to get mark");
        let english_mark: u64 = english_mark.trim().parse().expect("Type a number");

        let student = Student {
            id: id,
            name: name,
            math_mark: math_mark,
            english_mark: english_mark,
            science_mark: science_mark
        };

        students.push(student);
    }
    students

}

