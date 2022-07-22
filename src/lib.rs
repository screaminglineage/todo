use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

#[derive(PartialEq, Debug)]
struct Task {
    description: String,
    is_complete: bool,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            description: name,
            is_complete: false,
        }
    }

    fn from_string(task: &str, seperator: char) -> Task {
        let mut tasks = task.split(seperator);
        let description = match tasks.next() {
            Some(n) => n.to_string(),
            None => panic!("Failed to create Task struct: Couldnt parse name from string"),
        };
        let is_complete = match tasks.next() {
            Some("true") => true,
            Some("false") => false,
            _ => panic!("Failed to create Task struct: Couldnt parse is_complete from string"),
        };

        Task {
            description,
            is_complete,
        }
    }

    fn view(&self) -> String {
        let checkbox: &str;
        if self.is_complete {
            checkbox = "[x]"
        } else {
            checkbox = "[ ]"
        }
        format!("{} {}", checkbox, self.description)
    }

    fn set_complete(&mut self) {
        self.is_complete = true;
    }

    fn write_to_file(&self, file: &mut std::fs::File, separator: char) -> io::Result<()> {
        writeln!(
            file,
            "{}{}{}",
            self.description, separator, self.is_complete
        )?;
        Ok(())
    }
}



// Displays a prompt to the user and returns their input
pub fn take_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush buffer");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}


pub fn add_task(task_name: String, filepath: &str, separator: char) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)?;

    let task = Task::new(task_name);
    task.write_to_file(&mut file, separator)?;
    Ok(())
}

pub fn display_tasks(filepath: &str, separator: char) -> io::Result<()> {
    let tasks_data = fs::read_to_string(filepath)?;

    let mut i = 1;
    for line in tasks_data.lines() {
        let task = Task::from_string(line, separator);
        println!("{i}. {}", task.view());
        i += 1;
    }
    Ok(())
}

pub fn mark_as_done(task_nums: Vec<u32>, filepath: &str, separator: char) -> io::Result<()> {
    let task_data = fs::read_to_string(filepath)?;
    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("temp.txt")?;

    let mut i = 1;
    for line in task_data.lines() {
        if task_nums.contains(&i) {
            let mut task = Task::from_string(line, separator);
            task.set_complete();
            task.write_to_file(&mut temp_file, separator)?;
        } else {
            writeln!(temp_file, "{line}")?;
        }
        i += 1
    }
    fs::remove_file(filepath)?;
    fs::rename("temp.txt", filepath)?;

    Ok(())
}


pub fn remove_all(filepath: &str) -> io::Result<()>{
    let _ = File::create(filepath)?;
    Ok(())
}


// Parses a pattern from "1-6,13,7-9" to [1,2,3,4,5,6,13,7,8,9]
pub fn parse_pattern(pattern: String) -> Vec<u32> {
    let mut nums = Vec::new();
    for num in pattern.split(",") {
        let mut n = num
                .split("-")
                .map(|s| s.parse::<u32>().expect("Invalid User Input"));
        
        let lower = match n.next() {
            Some(num) => num,
            None => panic!("FUUUUUCK") 
        };
        let upper = match n.next() {
            Some(num) => num,
            None => lower
        };

        // println!("n -> {lower} {upper}");
        
        for i in lower..upper+1 {
            nums.push(i);
        }
    }
    nums
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_from_string() {
        assert_eq!(
            Task::from_string("Theres another one`false".into(), '`'),
            Task {
                description: "Theres another one".into(),
                is_complete: false
            }
        );
    }

    #[test]
    fn pattern_parser() {
        assert_eq!(
            parse_pattern("1,5,7".into()),
            [1,5,7]
        );
    }

    #[test]
    fn pattern_parser_2() {
        assert_eq!(
            parse_pattern("1-6,13,7-9".into()),
            [1,2,3,4,5,6,13,7,8,9]
        );
    }

}
