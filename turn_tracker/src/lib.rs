use std::error::Error;


#[derive(Debug)]
pub struct Combat {
    pub round: u8,
    pub characters: Vec<Character>,
    pub environ: Vec<Effect>,
}

impl Combat {
    pub fn new() -> Combat {
        let round = 0;
        let characters: Vec<Character> = Vec::new();
        let environ: Vec<Effect> = Vec::new();

        Combat { round, characters, environ }
    }

    pub fn add_environ(&mut self, effect: Effect) {
        self.environ.push(effect);
    }

    pub fn add_char(&mut self, new_char: Character) {
        self.characters.push(new_char);
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct Character {
    pub name: String,
    pub initiative: u8,
    pub effects: Vec<Effect>,
}

impl Character {
    pub fn new(name: &str) -> Result<Character, &'static str> {
        let name: String = name.to_string();
        let initiative = 0;
        let effects: Vec<Effect> = Vec::new();

        Ok(Character { name, initiative, effects })
    }

    pub fn change_init(&mut self, new_init: u8) {
        self.initiative = new_init;
    }

    pub fn add_new_effect(&mut self, effect_vec: &Vec<String>) -> Result<(), String> {
        let new_effect = Effect::new(effect_vec.iter())?;

        self.effects.push(new_effect);

        Ok(())
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Effect {
    pub description: String,
    pub modifier: String,
    pub duration: u8,
}


impl Effect {
    pub fn new(mut effect_iter: std::slice::Iter<String>) -> Result<Effect, &'static str> {
        let description: String = match effect_iter.next() {
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get description string")
        };
        let modifier = match effect_iter.next() {
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get modifier string")
        };
        let duration = match effect_iter.next() {
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get duration string")
        };

        let duration: u8 = match duration.parse::<u8>() {
            Ok(dur) => dur,
            Err(_e) => return Err("Duration must be a positive integer.")
        };

        Ok(Effect { description, modifier, duration })
    }

    pub fn decrement_duration(&mut self) {
        self.duration = self.duration - 1;
    }
}


pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");

    let mut combat = Combat::new();

    let mut char_bob = Character::new("Bob")?;

    let mut char_jim = Character::new("Jim")?;

    let test_effect = Effect::new(
        vec![
            String::from("Blinded"),
            String::from("Perception -2"),
            String::from("2")
        ].iter()
    )?;

    println!("{:?},\n{:?} {:?},\n{:?}", combat, char_bob, char_jim, test_effect);

    println!("{:?}", combat);

    combat.add_char(char_bob);
    char_jim.add_effect(test_effect);
    combat.add_char(char_jim);

    println!("{:?}", combat);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_effect() -> Result<(), String> {
        let description: String = String::from("Blinded");
        let modifier: String = String::from("-2 Perception");
        let duration: String = String::from("3");
        let effect_vec = vec![description, modifier, duration];

        let test_effect = Effect::new(effect_vec.iter())?;

        assert_eq!(test_effect.description, effect_vec[0]);
        assert_eq!(test_effect.modifier, effect_vec[1]);
        assert_eq!(test_effect.duration, effect_vec[2].parse::<u8>().unwrap());

        Ok(())
    }

    
    #[test]
    #[should_panic]
    fn test_effect_inv_duration() {
        let description: String = String::from("Blinded");
        let modifier: String = String::from("-2 Perception");
        let duration: String = String::from("-3");
        let effect_vec = vec![description, modifier, duration];

        Effect::new(effect_vec.iter())
            .expect("Effect creation failed");
    }

    #[test]
    #[should_panic]
    fn test_effect_no_description() {
        let effect_vec: Vec<String> = Vec::new();

        Effect::new(effect_vec.iter())
            .expect("Effect creation failed");
    }


    #[test]
    #[should_panic]
    fn test_effect_no_modifier() {
        let description: String = String::from("Blinded");
        let effect_vec: Vec<String> = vec![description];

        Effect::new(effect_vec.iter())
            .expect("Effect creation failed");
    }


    #[test]
    #[should_panic]
    fn test_effect_no_duration() {
        let description: String = String::from("Blinded");
        let modifier: String = String::from("-2 Perception");
        let effect_vec: Vec<String> = vec![description, modifier];

        Effect::new(effect_vec.iter())
            .expect("Effect creation failed");
    }


    #[test]
    fn test_effect_decrement_duration() -> Result<(), String> {
        let description: String = String::from("Blinded");
        let modifier: String = String::from("-2 Perception");
        let duration: String = String::from("3");
        let effect_vec = vec![description, modifier, duration];

        let mut test_effect = Effect::new(effect_vec.iter())?;

        assert_eq!(test_effect.duration, 3);
        test_effect.decrement_duration();
        assert_eq!(test_effect.duration, 2);

        Ok(())
    }


    #[test]
    fn test_new_character() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");

        let new_char = Character::new(&name[..])?;

        assert_eq!(new_char.name, name);

        Ok(())
    }


    #[test]
    fn test_character_change_init() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");
        let mut character = Character::new(&name[..])?;

        assert_eq!(character.initiative, 0);
        character.change_init(16);
        assert_eq!(character.initiative, 16);

        Ok(())
    }


    #[test]
    fn test_character_add_new_effect() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");
        let mut character = Character::new(&name[..])?;
        let effect_vec = vec![String::from("Blinded"), String::from("-2 Perception"), String::from("3")];
        let test1_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");
        let test2_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");

        assert_eq!(character.effects, Vec::new());
        character.add_new_effect(&effect_vec).expect("Effect creation failed.");
        assert_eq!(character.effects, vec![test1_effect]);
        assert_eq!(character.effects[0], test2_effect);

        Ok(())
    }
    

    #[test]
    fn test_character_add_effect() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");
        let mut character = Character::new(&name[..])?;
        let effect_vec = vec![String::from("Blinded"), String::from("-2 Perception"), String::from("3")];
        let new_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");
        let test1_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");
        let test2_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");

        assert_eq!(character.effects, Vec::new());
        character.add_effect(new_effect);
        assert_eq!(character.effects, vec![test1_effect]);
        assert_eq!(character.effects[0], test2_effect);

        Ok(())
    }
     
    
    #[test]
    fn test_new_combat() -> Result<(), String> {
        let new_combat = Combat::new();

        assert_eq!(new_combat.round, 0);
        assert_eq!(new_combat.characters, Vec::new());
        assert_eq!(new_combat.environ, Vec::new());

        Ok(())
    }

    
    #[test]
    fn test_combat_add_effect_environ() -> Result<(), String> {
        let mut new_combat = Combat::new();
        let description: String = String::from("Blinded");
        let modifier: String = String::from("-2 Perception");
        let duration: String = String::from("3");
        let effect_vec = vec![description, modifier, duration];
        let test_effect = Effect::new(effect_vec.iter())?;

        new_combat.add_environ(test_effect);

        assert_eq!(new_combat.environ, vec![ Effect::new(effect_vec.iter())? ]);

        Ok(())
    }

    
    #[test]
    fn test_combat_add_character() -> Result<(), String> {
        let mut new_combat = Combat::new();
        let test_char = Character::new("TEST")?;

        new_combat.add_char(test_char);

        assert_eq!(new_combat.characters, vec![Character::new("TEST")?]);

        Ok(())
    }   
}

