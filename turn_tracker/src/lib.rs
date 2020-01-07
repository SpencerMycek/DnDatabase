use std::error::Error;

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
}


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
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");

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
    #[test]
    fn test_new_character() -> Result<(), String> {
        let name: String = String::from("TEST_NAME");

        let new_char = Character::new(&name[..])?;

        assert_eq!(new_char.name, name);

        Ok(())
    }
        
}

