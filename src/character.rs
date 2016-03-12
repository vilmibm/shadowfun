use std::cmp::max;
use dice;
use dice::RollResult;

#[derive(Debug)]
pub enum Race {
    Human,
    Ork,
    Elf,
    Dwarf,
    Troll,
}

#[derive(Debug)]
pub struct Character {
    name: &'static str,
    body: i32,
    race: Race,
    intelligence: i32,
    strength: i32,
    charisma: i32,
    willpower: i32,
    quickness: i32,

    stun_level: i32,
    phys_level: i32,
}

pub enum Damage {
    Stun,
    Physical,
}

impl Character {
    pub fn new(name: &'static str, race: Race) -> Character {
        // TODO this is obviously just for testing.
        Character {
            name: name,
            race: race,
            body: 1,
            intelligence: 2,
            strength: 3,
            charisma: 4,
            willpower: 5,
            quickness: 6,
            phys_level: 0,
            stun_level: 0,
        }
    }

    pub fn reaction(&self) -> i32 {
        (self.intelligence + self.quickness) / 2
    }

    pub fn injure(&mut self, kind: Damage, amount: i32) -> &Self {
        match kind {
            Damage::Stun => {
                if self.stun_level + amount >= 10 {
                    self.stun_level = 10;
                    self.phys_level += amount - (10 - self.stun_level);
                    println!("WARNING: {} has fallen unconscious.", self.name);
                } else {
                    self.stun_level += amount;
                }
            },
            Damage::Physical => {
                self.phys_level += amount;
                if self.phys_level > 10 {
                    println!("WARNING: {} has died.", self.name);
                }
            }
        };
        self
    }

    fn injury_to_mod(&self) -> i32 {
        match max(self.stun_level, self.phys_level) {
            0 => 0,
            1...2 => 1,
            3...5 => 2,
            _ => 3,
        }
    }

    pub fn roll(&self, die: i32, tn: i32) -> RollResult {
        if self.phys_level > 10 || self.stun_level > 10 {
            println!("WARNING rolling for dead or unconscious character");
        }
        let tn = self.injury_to_mod() + tn;
        return dice::roll(die, tn);
    }
}