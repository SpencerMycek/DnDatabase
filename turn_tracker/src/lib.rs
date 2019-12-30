use std::error::Error;

pub struct Effect {
    pub description: String,
    pub modifier: String,
    pub duration: u8,
}

impl Effect {
    pub fn new(desc: &String, modi: &String, dur: &String) -> Result<Effect, &'static str> {
        let description = desc.clone();
        let modifier = modi.clone();
        let duration = dur.parse::<u8>();

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

        let test_effect = Effect::new(&description, &modifier, &duration)?;

        assert_eq!(test_effect.description, description);
        assert_eq!(test_effect.modifier, modifier);
        assert_eq!(test_effect.duration, duration.parse::<u8>().unwrap());

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_effect_inv_duration() {
        let description: String = String::from("Blinded");
        let modifier: String = String::from("-2 Perception");
        let duration: String = String::from("-3");

        Effect::new(&description, &modifier, &duration)
            .expect("Effect creation failed");
    }
        
}

