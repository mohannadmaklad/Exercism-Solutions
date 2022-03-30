// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Self {
            level: level,
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                0
            }

            Some(val) => {
                if (val < mana_cost) {
                    0
                } else {
                    self.mana = Some(val - mana_cost);
                    2 * mana_cost
                }
            }
        }
    }
}
