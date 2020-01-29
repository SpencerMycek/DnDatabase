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
        self.characters.sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }

    pub fn next_round(&mut self) {
        self.round += 1;
        for character in &mut self.characters {
            character.new_round();
        }
        for effect in &mut self.environ {
            effect.decrement_duration();
        }
        self.environ.retain(|x| x.duration != 0);
    }
}


#[derive(Debug, PartialEq)]
pub struct Character {
    pub name: String,
    pub initiative: u8,
    pub effects: Vec<Effect>,
}

impl Character {
    pub fn new(name: &str, init: u8) -> Result<Character, &'static str> {
        let name: String = name.to_string();
        let initiative = init;
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

    pub fn rm_old_effects(&mut self) {
        self.effects.retain(|x| x.duration != 0)

    }

    pub fn new_round(&mut self) {
        for effect in &mut self.effects {
            effect.decrement_duration()
        }
        self.rm_old_effects();
    }
}

#[derive(Debug, PartialEq)]
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

    let char_bob = Character::new("Bob", 11)?;

    let mut char_jim = Character::new("Jim", 12)?;

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

    combat.next_round();
    println!("{:?}", combat);
    combat.next_round();
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

        let new_char = Character::new(&name[..], 10)?;

        assert_eq!(new_char.name, name);
        assert_eq!(new_char.initiative, 10);

        Ok(())
    }


    #[test]
    fn test_character_change_init() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");
        let mut character = Character::new(&name[..], 10)?;

        assert_eq!(character.initiative, 10);
        character.change_init(16);
        assert_eq!(character.initiative, 16);

        Ok(())
    }


    #[test]
    fn test_character_add_new_effect() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");
        let mut character = Character::new(&name[..], 10)?;
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
        let mut character = Character::new(&name[..], 10)?;
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
    fn test_character_rm_old() -> Result<(), String> {
        let mut character = Character::new("TEST", 10)?;
        let effect_vec = vec![String::from("Blinded"), String::from("-2 Perception"), String::from("0")];
        let new_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");

        character.add_effect(new_effect);
        assert_ne!(character.effects, Vec::new());
        character.rm_old_effects();
        assert_eq!(character.effects, Vec::new());

        Ok(())
    }


    #[test]
    fn test_character_new_round() -> Result<(), String> {
        let mut character = Character::new("TEST", 10)?;
        let effect_vec = vec![String::from("Blinded"), String::from("-2 Perception"), String::from("2")];
        let new_effect = Effect::new(effect_vec.iter()).expect("Effect creation failed.");

        character.add_effect(new_effect);
        character.new_round();
        assert_ne!(character.effects, Vec::new());
        character.new_round();
        assert_eq!(character.effects, Vec::new());

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
        let test_char = Character::new("TEST", 10)?;

        new_combat.add_char(test_char);

        assert_eq!(new_combat.characters, vec![Character::new("TEST", 10)?]);

        Ok(())
    }


    #[test]
    fn test_combat_next_round() -> Result<(), String> {
        let mut new_combat = Combat::new();
        let mut test_char = Character::new("TEST", 10)?;
        let effect_vec = vec![String::from("Blinded"), String::from("-2 Perception"), String::from("1")];
        let effect1 = Effect::new(effect_vec.iter()).expect("Effect creation failed.");
        let effect2 = Effect::new(effect_vec.iter()).expect("Effect creation failed.");
        test_char.add_effect(effect1);
        
        new_combat.add_environ(effect2);
        new_combat.add_char(test_char);

        new_combat.next_round();
        for i in new_combat.characters {
            assert_eq!(i.effects, Vec::new());
        }
        assert_eq!(new_combat.environ, Vec::new());
        assert_eq!(new_combat.round, 1);

        Ok(())
    }
}

