// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            match self.level {
                10.. => Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                }),
                _ => Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                }),
            }
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(amount) => {
                if amount >= mana_cost {
                    self.mana = Some(amount - mana_cost);
                    2 * mana_cost
                } else {
                    0
                }
            }
            None => {
                self.health = self.health - min(self.health, mana_cost);
                0
            }
        }
    }
}
