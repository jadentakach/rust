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

fn main() {
    let mut homework_entries: Vec<HomeworkEntry> = Vec::new();
    let mut input: String = String::new();

    println!("Do you want to..\n1. Add a new homework entry\n2. View all homework entries\n3. Exit");
    std::io::stdin().read_line(&mut input).expect("Failed to read input"); 

    match input.as_str().trim() {
        "1" => {
            let mut title = String::new();
            let mut course_input = String::new();
            let mut description = String::new();
            let mut due_date = String::new();

            println!("Enter the title of the homework:");
            std::io::stdin().read_line(&mut title).expect("Failed to read input");

            println!("Enter the course by shorthand:\nCulinary Arts: CA\nAnatomy Physiology: AP\nBusiness of Sports: BS\nEnglish Writing: EW\nPsychology: PS\nCivics: CV\nBaking: BK\nPrecalculus: PC");
            std::io::stdin().read_line(&mut course_input).expect("Failed to read input");

            println!("Enter the description of the homework:");
            std::io::stdin().read_line(&mut description).expect("Failed to read input");

            println!("Enter the due date of the homework:");
            std::io::stdin().read_line(&mut due_date).expect("Failed to read input");

            let course = match course_input.trim() {
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

            let entry = new_entry(title.trim().to_string(), course, description.trim().to_string(), due_date.trim().to_string());
            homework_entries.push(entry);
        },
        "2" => {
            for entry in &homework_entries {
                println!("Title: {}, Course: {:?}, Description: {}, Due Date: {}", entry.title, entry.course, entry.description, entry.due_date);
            }
        },
        "3" => {
            println!("Exiting...");
        },
        _ => {
            println!("Invalid option selected.");
        }
    }
}