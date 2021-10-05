// Structs
#[derive(Debug)]
struct Learner {
    who: String,
    what: String,
}

impl Learner {
    fn change_what(&mut self, new: String) {
        self.what = new;
    }

    fn new(who: String, what: String) -> Learner {
        Learner { who, what }
    }
}

// Tuple Structs

#[derive(Debug)]
struct Subject(String);

#[derive(Debug)]
struct Type(String);

pub fn creation_and_mutation() {
    // Create a mutable struct
    let mut learner = Learner::new(String::from("arjun"), String::from("rust"));
    learner.who = String::from("arjun puri");
    learner.what = String::from("Rust");

    let mut learner2 = Learner {
        who: String::from("aditi"),
        ..learner
    };
    println!("name of learner 2: {}", learner2.who);

    let subject = Subject(String::from("rust"));
    let subject_type = Type(String::from("language"));
    println!("Subject name: {:#?}", subject);
    println!("Subject type: {:#?}", subject_type);

    learner2.change_what(String::from("Ownership"));
    println!("Learner 2: {:?}", learner2);
}

// Enums
#[derive(Debug)]
enum PersonalityType {
    Introvert(i64, i64),
    Extrovert(String),
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
enum Person {
    Cool(String, PersonalityType, Gender),
    NotCool(PersonalityType, Gender),
}

impl Person {
    fn say_hi(&self) -> String {
        match self {
            Person::Cool(name, personality_type, _) => format!(
                "hi my name is {} and I'm an {:?}",
                name, personality_type,
            ),
            Person::NotCool(_, _) => String::from("I'm not cool"),
        }
    }
}

pub fn enums() {
    let introvert = PersonalityType::Introvert(0, 10);
    let extrovert = PersonalityType::Extrovert(String::from("low"));

    let person1 = Person::Cool(String::from("aditi"), extrovert, Gender::Female);
    let person2 = Person::NotCool(introvert, Gender::Male);

    println!("{}", person1.say_hi());
    person2.say_hi();

    let a = Some(5);
    let b: Option<i32> = None;

    match a {
        Some(_) => println!("found a value for a!"),
        None => println!("no value for a"),
    }

    match b {
        Some(_) => println!("found a value for b!"),
        None => println!("no value for b"),
    }
}

