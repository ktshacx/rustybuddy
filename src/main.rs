use clap::{Parser, Subcommand};
use crossterm::{cursor, execute, style::Print};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{stdout, Write},
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

#[derive(Parser)]
#[command(name = "RustyBuddy", about = "Your Rusty Pet or Buddy!")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[clap(short, long)]
        name: String,

        #[clap(short, long)]
        kind: String,
    },
    Stats,
    Feed,
    Play,
    Sleep,
    Save,
    Load,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pet {
    name: String,
    kind: String,
    hunger: i32,
    happiness: i32,
    energy: i32,
    health: i32,
    age: u32,
    last_update: SystemTime,
}

impl Pet {
    fn new(name: String, kind: String) -> Self {
        Pet {
            name,
            kind,
            hunger: 50,
            happiness: 50,
            energy: 50,
            health: 100,
            age: 0,
            last_update: SystemTime::now(),
        }
    }

    fn feed(&mut self) {
        self.hunger = (self.hunger - 10).max(0);
        println!("{} is happily munching on food!", self.name);
        self.show_animation("eating");
        self.check_mood();
    }

    fn play(&mut self) {
        if self.energy >= 10 {
            self.happiness = (self.happiness + 15).min(100);
            self.energy -= 10;
            println!("{} is having a great time playing!", self.name);
            self.show_animation("playing");
        } else {
            println!("{} is too tired to play. Try letting them rest.", self.name);
        }
        self.check_mood();
    }

    fn sleep(&mut self) {
        self.energy = (self.energy + 20).min(100);
        self.hunger = (self.hunger + 5).min(100); // Sleeping makes them a bit hungry.
        println!("{} is peacefully sleeping. Sweet dreams!", self.name);
        self.show_animation("sleeping");
        self.check_mood();
    }

    fn age_up(&mut self) {
        self.age += 1;
        if self.age % 5 == 0 {
            println!("🎉 {} has grown older and wiser! Age: {}", self.name, self.age);
        }
    }

    fn decay_stats(&mut self) {
        let elapsed = self.last_update.elapsed().unwrap_or(Duration::ZERO).as_secs() as i32;
        self.hunger = (self.hunger + elapsed / 60).min(100);
        self.happiness = (self.happiness - elapsed / 120).max(0);
        self.energy = (self.energy - elapsed / 90).max(0);
        self.health = if self.hunger >= 80 || self.happiness <= 20 {
            (self.health - elapsed / 120).max(0)
        } else {
            self.health
        };
        self.last_update = SystemTime::now();

        if elapsed > 3600 {
            self.age_up();
        }
    }

    fn show_stats(&self) {
        println!(
            "Pet: {}\nKind: {}\nAge: {}\nHunger: {}\nHappiness: {}\nEnergy: {}\nHealth: {}\nMood: {}\n",
            self.name,
            self.kind,
            self.age,
            self.hunger,
            self.happiness,
            self.energy,
            self.health,
            self.get_mood()
        );
    }

    fn get_mood(&self) -> &str {
        if self.happiness > 80 {
            "Happy 😊"
        } else if self.energy < 20 {
            "Tired 😴"
        } else if self.hunger > 80 {
            "Hungry 😋"
        } else {
            "Neutral 😐"
        }
    }

    fn check_mood(&self) {
        println!("Current mood: {}", self.get_mood());
    }

    fn show_animation(&self, action: &str) {
        let frames = match action {
            "eating" => vec!["😋", "🍗", "😋", "🍖", "😋"],
            "playing" => vec!["🐾", "🎾", "🐾", "🎾", "🐾"],
            "sleeping" => vec!["💤", "😴", "💤", "😴"],
            _ => vec!["❓"],
        };
        self.animate(frames);
    }

    fn animate(&self, frames: Vec<&str>) {
        let mut stdout = stdout();
        for frame in frames {
            execute!(stdout, cursor::MoveTo(0, 6), Print(frame)).unwrap();
            thread::sleep(Duration::from_millis(400));
        }
        execute!(stdout, cursor::MoveToNextLine(2)).unwrap();
    }
}

fn load_pet(file_path: &str) -> Option<Pet> {
    if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path).ok()?;
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

fn save_pet(pet: &Pet, file_path: &str) {
    if let Ok(data) = serde_json::to_string_pretty(pet) {
        fs::write(file_path, data).expect("Failed to save pet data!");
        println!("Pet data saved!");
    } else {
        println!("Failed to serialize pet data!");
    }
}

fn main() {
    let args = Cli::parse();
    let save_file = "pet.json";
    let mut pet = match &args.command {
        Commands::New { name, kind } => {
            println!("🎉 A new pet {} the {} has been created!", name, kind);
            let new_pet = Pet::new(name.clone(), kind.clone());
            save_pet(&new_pet, save_file); // Save the pet immediately after creation
            new_pet
        }
        Commands::Load => load_pet(save_file).unwrap_or_else(|| {
            println!("No saved pet found! Create a new one.");
            std::process::exit(1);
        }),
        _ => {
            load_pet(save_file).unwrap_or_else(|| {
                println!("No saved pet found! Create a new one.");
                std::process::exit(1);
            })
        }
    };

    pet.decay_stats();

    match args.command {
        Commands::Stats => pet.show_stats(),
        Commands::Feed => {
            pet.feed();
            save_pet(&pet, save_file); // Save after feeding
        }
        Commands::Play => {
            pet.play();
            save_pet(&pet, save_file); // Save after playing
        }
        Commands::Sleep => {
            pet.sleep();
            save_pet(&pet, save_file); // Save after sleeping
        }
        Commands::Save => save_pet(&pet, save_file),
        Commands::Load => pet.show_stats(),
        _ => {}
    }
}