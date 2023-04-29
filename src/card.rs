use std::io;

use crossterm::{
    queue,
    style,
    style::{Color, Attribute},
    cursor,
};

use crate::effect::{Effect, EffectType, StateEffect};
use crate::icons;

pub const CARD_WIDTH: u16 = 20;
pub const CARD_HEIGHT: u16 = 10;

const BORDER: &'static str = r#"╭──────────────────╮
│                  │
├──────────────────┤
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
╰──────────────────╯"#;

pub const EMPTY_CARD: &'static str = r#"╭ ─ ─ ─ ─ ─ ─ ─ ─ ─╮
╵                  ╷
╵                  ╷
╵                  ╷
╵                  ╷
╵                  ╷
╵                  ╷
╵                  ╷
╵                  ╷
╰─ ─ ─ ─ ─ ─ ─ ─ ─ ╯"#;

#[derive(Default, Clone)]
pub struct Card {
    name: &'static str,
    health: i32,
    health_overload: i32,
    max_health: i32,
    base_damage: i32,
    added_damage: i32,
    description: &'static str,

    effects: Vec<Effect>,

    state_effects: Vec<StateEffect>,

    attack_effects: Vec<i32>,
}

impl Card {
    pub fn new(name: &'static str, max_health: i32, damage: i32, description: &'static str) -> Self {
        Card {
            name,
            health: max_health,
            max_health,
            base_damage: damage,
            description,
            ..Default::default()
        }
    }

    pub fn recalculate_stats(&mut self) {
        self.health_overload = 0;
        self.added_damage = 0;

        for i in 0..self.state_effects.len() {
            self.state_effects[i].clone().apply(i, self)
        }
    }

    pub fn add_damage(&mut self, amount: i32) {
        self.added_damage += amount
    }

    pub fn add_health_overload(&mut self, amount: i32) {
        self.health_overload += amount
    }

    pub fn take_damage(&mut self, mut amount: i32, from_effect: bool) {
        for effect in &mut self.effects {
            effect.on_damage_taken(&mut amount, from_effect)
        }
        self.health -= amount
    }

    pub fn apply_effect(&mut self, effect_type: EffectType, amount: i32) {
        for effect in &mut self.effects {
            if effect.effect_type == effect_type {
                effect.stack += amount;
                return
            }
        }
        self.effects.push(Effect::new(amount, effect_type))
    }

    pub fn clean(&mut self) {
        self.effects.retain(|x| x.stack > 0)
    }

    pub fn draw<W>(&self, w: &mut W, x: u16, y: u16 ) -> io::Result<()>
    where 
        W: io::Write,
    {

        // Border
        queue! {
            w,
            cursor::MoveTo(x, y),
        }?;
        for line in BORDER.split('\n') {
            queue! {
                w,
                style::Print(line),
                cursor::MoveDown(1),
                cursor::MoveToColumn(x),
            }?
        }

        // Health
        let t = icons::hp();
        queue!
        (
            w,
            cursor::MoveTo(x + 1, y + 1),
            style::Print(self.health),
            style::Print('/'),
            style::Print(self.max_health),
            style::SetForegroundColor(t.0),
            style::Print(t.1),
            style::ResetColor,
        )?;

        let t = icons::dmg();
        // Damage
        queue!
        (
            w,
            cursor::MoveTo(x + 15, y + 1),
            style::Print(self.base_damage + self.added_damage),

            cursor::MoveTo(x + 17, y + 1),
            style::SetForegroundColor(t.0),
            style::Print(t.1),
            style::ResetColor,
        )?;

        // Name
        queue!
        (
            w,
            cursor::MoveTo( x + CARD_WIDTH/2 - self.name.len() as u16 /2, y),
            style::Print(self.name),
        )?;

        // Description
        queue!(
            w,
            cursor::MoveTo( x + 1, y + 3)
            )?;
        for line in textwrap::wrap(self.description, (CARD_WIDTH -2) as usize) {
            queue!(
                w,
                style::Print(line),
                cursor::MoveDown(1),
                cursor::MoveToColumn(x + 1),
                )?;
        }

        // Effects
        if !self.effects.is_empty() {
            queue!(
                w,
                cursor::MoveTo(x+1, y+CARD_HEIGHT-1),
                )?;
            for effect in &self.effects {
                queue!(
                    w,
                    style::Print(" ")
                    )?;
                effect.draw(w)?;
            }
            queue!(
                w,
                style::Print(" ")
                )?;
        }

        io::Result::Ok(())
    }

}
