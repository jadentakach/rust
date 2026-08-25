#[derive(Debug)]
enum Course {
    CulinaryArts,
    AnatomyPhysiology,
    BusinessOfSports,
    EnglishWriting,
    Psychology,
    Civics,
    Baking,
    Precalculus
}

struct HomeworkEntry {
    title: String,
    course: Course,
    description: String,
    due_date: String
}

fn new_entry(title: String, course: Course, description: String, due_date: String) -> HomeworkEntry {
    HomeworkEntry {
        title,
        course,
        description,
        due_date
    }
}

fn get_input(output: &str) -> String {
    let mut input: String = String::new();
    
    println!("{}", output);
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}

fn add_homework(entries: &mut Vec<HomeworkEntry>) {
    let title: String = get_input("Enter the title of the homework:");
    let course_input: String = get_input("Enter the course by shorthand:\nCulinary Arts: CA\nAnatomy Physiology: AP\nBusiness of Sports: BS\nEnglish Writing: EW\nPsychology: PS\nCivics: CV\nBaking: BK\nPrecalculus: PC");
    let description: String = get_input("Enter the description of the homework:");
    let due_date: String = get_input("Enter the due date of the homework:");

    let course: Course = match course_input.as_str() {
        "CA" => Course::CulinaryArts,
        "AP" => Course::AnatomyPhysiology,
        "BS" => Course::BusinessOfSports,
        "EW" => Course::EnglishWriting,
        "PS" => Course::Psychology,
        "CV" => Course::Civics,
        "BK" => Course::Baking,
        "PC" => Course::Precalculus,

        _ => {
            println!("Invalid course entered. Defaulting to CulinaryArts.");
            Course::CulinaryArts
        }
    };

    let entry: HomeworkEntry = new_entry(title, course, description, due_date);
    entries.push(entry);
}

fn view_homework(entries: &Vec<HomeworkEntry>) {
    for entry in entries {
        println!("Title: {}, Course: {:?}, Description: {}, Due Date: {}", entry.title, entry.course, entry.description, entry.due_date);
    }
}

fn exit_loop() {
    println!("Exiting...");
}

fn main() {
    let mut homework_entries: Vec<HomeworkEntry> = Vec::new();

    loop {
        let input: String = get_input("Do you want to..\n1. Add a new homework entry\n2. View all homework entries\n3. Exit");

        match input.as_str() {
            "1" => add_homework(&mut homework_entries),
            "2" => view_homework(&homework_entries),
            "3" => { exit_loop(); break; },
            _ => println!("Invalid option selected.")
        }
    }
}