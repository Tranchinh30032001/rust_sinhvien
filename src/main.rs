use std::{io, vec};

#[derive(Debug)]
enum ACTION {
    ADD,
    VIEW, 
    EDIT,
    DELETE
}
#[derive(Debug)]
enum SEX {
    MALE,
    FEMALE
}
#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    school: String,
    sex: SEX
}
impl Student {
    fn create_student() -> Self {
        let mut name: String = String::new();
        let mut age: String = String::new();
        let mut school: String = String::new();
        let mut input_sex: String =  String::new();
        let mut sex: SEX = SEX::FEMALE;
        println!("Vui long nhap ten sinh vien: ");
        while io::stdin().read_line(&mut name).is_err() {
            println!("🧨Vui long nhap lai tren sinh vien: ");
        }
        println!("Vui long nhap tuoi sinh vien: ");
        while io::stdin().read_line(&mut age).is_err() {
            println!("Vui long nhap lai tuoi sinh vien: ");
        }
        println!("Vui long nhap ten truong: ");
        while io::stdin().read_line(&mut school).is_err(){
            println!("Vui long nhap lai ten truong: ");
        }
        println!("Chon gioi tinh: ");
        println!("1. MALE");
        println!("2. FEMALE");
        io::stdin().read_line(&mut input_sex).is_ok();
            let index = input_sex.trim();
            match index {
                "1" => sex = SEX::MALE,
                "2" => sex = SEX::FEMALE,
                _ => println!("Vui long chon lai")
            }
        
        let age = age.trim().parse().unwrap();
        let name = name.trim().to_owned();
        let school = school.trim().to_owned();
        return Student { name: name, age: age, school: school, sex: sex }
    }
    fn delete_student(vector: &mut Vec<Student>) {
        let mut name:String = String::new();
        let mut choose: String = String::new();
        println!("Nhap ten sinh vien cần xóa: ");
        io::stdin().read_line(&mut name).unwrap();
        for (index, item) in vector.iter().enumerate() {
            if item.name == name.trim() {
                println!("Bạn muốn xóa sinh viên: {:?}", item);
                println!("1. Yes");
                println!("2. No");
                io::stdin().read_line(&mut choose).is_ok();
                let luachon = choose.trim();
                match luachon {
                    "1" => {let x:Student = vector.remove(index);},
                    _ => break
                }
            }
        }
    }
}
impl ACTION {
    fn show() {
        println!("Choose your option: ");
        println!("1. Add Student: ");
        println!("2. View Student: ");
        println!("3: Edit Student: ");
        println!("4: Delete Student: ");
        println!("5: Exit");
    }
    fn choose_action(input: &str) -> Option<ACTION> {
        match input {
            "1" => return Some(ACTION::ADD),
            "2" => return Some(ACTION::VIEW),
            "3" => return Some(ACTION::EDIT),
            "4" => return Some(ACTION::DELETE),
            _ => None
        }
    }
    fn excute_action(input: &Option<ACTION>, vector: &mut Vec<Student>) {
        match input {
            Some(ACTION::ADD) => {add_student(vector)},
            Some(ACTION::VIEW) => view_list_student(vector),
            Some(ACTION::EDIT) => println!("Chinh sua sinh vien: "),
            Some(ACTION::DELETE) => println!("Xoa sinh vien: "),
            _ => println!("Exit")
        }
    }
}

fn get_input() -> Option<String> {
    let mut buff = String::new();
    while io::stdin().read_line(&mut buff).is_err() {
        println!("Please try again");
    }
    let input = buff.trim().to_owned();
    if input == "" {
        None
    }
    else {
        Some(input)
    }
}
fn add_student(vector: &mut Vec<Student>){
    let user1 = Student::create_student();
    vector.push(user1);
}
fn view_list_student(vector: &Vec<Student>){
    for i in vector {
        println!("list: {:?}", i);
    }
}

fn main() {
    let mut arrStudent: Vec<Student> = Vec::new();
    loop {
        ACTION::show();
        let input = get_input().unwrap();
        let option = ACTION::choose_action(&input);
        ACTION::excute_action(&option, &mut arrStudent)
    }
}