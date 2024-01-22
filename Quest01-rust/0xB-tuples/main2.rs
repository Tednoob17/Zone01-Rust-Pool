pub struct Student(pub i32, pub String, pub String);

pub fn id(student: &Student) -> i32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.clone()
}

pub fn last_name(student: &Student) -> String {
    student.2.clone()
}

fn main() {
    let student = Student(1, String::from("John"), String::from("Doe"));

    println!("Student ID: {}", id(&student));
    println!("First Name: {}", first_name(&student));
    println!("Last Name: {}", last_name(&student));
}
