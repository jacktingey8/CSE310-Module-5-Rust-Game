use csv::Reader;
use rand::seq::SliceRandom; 
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut my_word = Word::new(String::new());


    my_word.set_word()?;

    // 3. Clear screen
    clear_console();

    println!("Selected Word: '{}' (Length: {})", my_word.word, my_word.length);

   
    let mut progress: Vec<char> = Vec::new();

    let turns = my_word.length * 2;

    for i in 0 .. turns{

    let input = get_user_letter();

    

    get_word_progress(&my_word.letters, input, &mut progress);


    }

   
    
    println!("\n"); 
    Ok(())
}

struct Word {
    word: String,
    length: usize,
    letters: Vec<char>,
}

impl Word {
  
    fn new(word: String) -> Self {
        let length = word.len();
        let letters: Vec<char> = word.chars().collect();
        Self { word, length, letters }
    }

 
    fn set_word(&mut self) -> Result<(), Box<dyn Error>> {
        let file_path = "words.csv";
        let file = File::open(file_path)?;
        let mut rdr = Reader::from_reader(file);

      
        let records: Vec<csv::StringRecord> = rdr.records().collect::<Result<_, _>>()?;

   
        let mut rng = rand::thread_rng();
        if let Some(random_record) = records.choose(&mut rng) {
            if let Some(picked_str) = random_record.get(0) {
                
                self.word = picked_str.to_string();
                self.length = self.word.len(); 
                self.letters = self.word.chars().collect(); 
            }
        } else {
            println!("The CSV file was empty.");
        }

        Ok(())
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_user_letter() -> char {
    let mut input = String::new();

    loop {
        print!("  -  Guess a letter: ");
        io::stdout().flush().unwrap();

        input.clear(); 
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim();
        
        if let Some(ch) = trimmed.chars().next() {
            if ch.is_alphabetic() {
                return ch.to_ascii_lowercase(); 
            }
        }
        
        println!("Invalid input. Please type a single letter.");
    }
}

fn get_word_progress(letters:&Vec<char>, input:char,  progress: &mut Vec<char>) {


    if letters == progress {
        println!("YOU GUESSED IT!")

    }
    for &letter in letters {
        if(progress.contains(&letter)){
            print!("{}",letter);
        }
        else if(input == letter){
                print!("{}",letter);
                progress.push(letter);
        }
        else {print!("_")}
    }

    let won = letters.iter().all(|ch| progress.contains(ch));

    if won {
        println!("\nYOU GUESSED IT!");
    }


}