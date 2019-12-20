use rand::Rng;

#[derive(Debug)]
struct Student 
{
    name: String,
    sub1: i32,
    sub2: i32
}

//struct Color(i32, i32, i32);

fn main() {

    let name = String::from("Ali");
    let sub1 = 88;
    let sub2 = 77;

    println!("Normal printing variable name {} sub1 {} sub2 {}", name, sub1, sub2);

    let stud_tuple = (String::from("Zaheer"), 91,81);

    println!("Tuple printing name {}, sub1 {} sub2 {}", stud_tuple.0, stud_tuple.1, stud_tuple.2);


    let student1 = Student {name: String::from("Zaheer"), sub1: 91, sub2: 81};

    println!("Student 1 instance name {} sub1 {} sub2 {}", student1.name,student1.sub1, student1.sub2);

    let mut student3 = Student {name: String::from("Major Sb"), sub1: 50, sub2: 81};

    println!("Student 3 instance name {} sub1 {} sub2 {}", student3.name,student3.sub1, student3.sub2);

    student3.sub1 = 91;

    println!("Student 3 aganin instance {:#?}", student3); //either use {:?} or for pretty printing use {:?}

    let student4 = Student {name: String::from("Waleed"), sub1: student3.sub1, sub2: 66}; // use other instance value

    println!("Student 4 instance {:#?}", student4); //either use {:?} or for pretty printing use {:?}

    let student5 = Student {name: String::from("Zahid"), ..student4}; // use other instance value using struct update syntex

    println!("Student 5 instance {:#?}", student5);

    let student6 = student3;

    println!("Student 6 instance {:#?}", student6);
    //println!("Student 3 instance {:#?}", student3);// Its been borrowed so error will come here

    let returned_student = one(name, sub1, sub2);

    println!("Returned student from function one {:#?}", returned_student);

    two(stud_tuple);

    let mut student7  = Student {name: String::from("Fakhir"), sub1:50, sub2:70};
    
    let returned_from_three = three(&mut student7);

    println!("Returned value from function three {:#?}", returned_from_three);

    //let paint = Color(12,34,44);

    //println!("Struct type tuple {:?}", paint);

    let returned_value = multiply(10,20);

    println!("returned valeu {}", returned_value.0);

    random();
    match_practice();
}

fn one(the_name: String, sub1: i32, sub2: i32) -> Student
{
    println!("Passed value to function one are: name = {}, sub1= {}, sub2 = {}", the_name, sub1, sub2);

    let student3 = Student {
        name: the_name, 
        sub1, 
        sub2
    }; // sub1 & sub2 as short hand

    println!("Student3 inside function three {:#?}", student3);

    return student3;
}

fn two(stud_tuple: (String, u32, u32))
{
    println!("Student tuple value {:?}", stud_tuple);
}

fn three(student7: &mut Student) -> &mut Student
{
    println!("The value of struct is {:?}", student7);

    student7.name = String::from("Name changed in function");
    student7.sub1 = student7.sub1 + student7.sub2;
    student7
}

fn multiply(num1: u32, num2: u32) -> (String, String, u32,Student)//function defination with parameters
{
    let new_num = num1 * num2;

    let name1 = String::from("M Ali");
    let name2 = String::from("Mufassar");

    let student = Student {name: String::from("Salman"), sub1: 12, sub2: 30};

    return (name1, name2, new_num, student);
}


//Random, Github, Match
fn random()
{
    let mut rand_number = rand::thread_rng();

    let n1: u8 = rand_number.gen();
    let n2: u16 = rand_number.gen();
    
    println!("8 bytes random no: {} & 16 bytes random no:{}", n1, n2);

    let range = rand_number.gen_range(100,1000);

    println!("Some random number between 100 & 1000 range is: {}", range);
}

fn match_practice()
{
    let some = 30;

    match some
    {
        10 => println!("No Match"),
        30 => println!("Perfect Match"),
        _=>println!("No occurences")
    }
}
