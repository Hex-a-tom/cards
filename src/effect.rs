use crossterm::{queue, style, style::Color};

use crate::Card;

use crate::text;

use std::io;

#[derive(Debug, PartialEq, Clone)]
pub enum EffectType {
    Poison,
    Curse,
}

#[derive(Debug, Clone)]
pub struct Effect {
    pub stack: i32,
    pub effect_type: EffectType,
}

impl Effect {
    pub fn new(stack: i32, effect_type: EffectType) -> Self {
        Effect { stack, effect_type }
    }

    pub fn on_turn_end(&mut self, card: &mut Card) {
        match self.effect_type {
            EffectType::Poison => { 
                card.take_damage(self.stack, true);
                self.stack -= 1;
            },
            _ => (),
        }
    }

    pub fn on_damage_taken(&mut self, damage: &mut i32, from_effect: bool) {
        match self.effect_type {
            EffectType::Curse if !from_effect => {
                self.stack -= 1;
                *damage *= 2;
            },
            _ => ()
        }
    }

    pub fn draw<W>(&self, w: &mut W ) -> io::Result<()>
    where 
        W: io::Write,
    {
        match self.effect_type {
            EffectType::Poison => {
                let t = text::get_poison_text();
                queue!(
                    w,
                    style::Print(self.stack),
                    style::SetForegroundColor(t.0),
                    style::Print(t.1),
                    style::ResetColor,
                    )
            },
            EffectType::Curse => {
                let t = text::get_curse_text();
                queue!(
                    w,
                    style::Print(self.stack),
                    style::SetForegroundColor(t.0),
                    style::Print(t.1),
                    style::ResetColor,
                    )
            }
        }
    }
}
