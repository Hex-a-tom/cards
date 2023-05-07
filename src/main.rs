use std::io;

use card::EMPTY_CARD;
use crossterm::event::KeyEventKind;
use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    style::Color,
};
use effect::EffectType;

use crate::card::{Card, CARD_HEIGHT, CARD_WIDTH};

mod card;
mod effect;
mod icons;

const HORIZONTAL_CARD_MARGIN: u16 = 0;
const VERTICAL_CARD_MARGIN: u16 = 1;
const CENTER_BOARD_MARGIN: u16 = 6;
const BOARD_COLUMNS: usize = 3;
const BOARD_ROWS: usize = 3;
const BOARD_WIDTH: u16 = BOARD_COLUMNS as u16 * CARD_WIDTH + (BOARD_COLUMNS as u16 -1) * HORIZONTAL_CARD_MARGIN;

const MIN_X: u16 = BOARD_WIDTH * 2 + CENTER_BOARD_MARGIN;
const MIN_Y: u16 = CARD_HEIGHT * BOARD_ROWS as u16 + (BOARD_ROWS as u16 - 1) * VERTICAL_CARD_MARGIN + CARD_HEIGHT/2;

type CardsOnBoard = [[Option<Card>; BOARD_ROWS]; BOARD_COLUMNS];

#[derive(Default)]
struct GameState {
    player_cards: Vec<Card>,

    player_board: CardsOnBoard,
    enemy_board: CardsOnBoard,

    dirty: bool,
}

fn draw_board<W>(board: &CardsOnBoard, w: &mut W, x: u16, y: u16) -> io::Result<()>
where
W: io::Write 
{
    for (r, row) in board.iter().enumerate()  {
        for (c, i) in row.iter().enumerate() {
            let xc = x + (CARD_WIDTH + HORIZONTAL_CARD_MARGIN) * c as u16;
            let yc = y + (CARD_HEIGHT + VERTICAL_CARD_MARGIN) * r as u16;
            if let Some(card) = i {
                card.draw(w, xc, yc)?
            } else {
                queue! {
                    w,
                    cursor::MoveTo(xc, yc),
                    style::SetForegroundColor(Color::DarkGrey),
                }?;
                for line in EMPTY_CARD.split('\n') {
                    queue! {
                        w,
                        style::Print(line),
                        cursor::MoveDown(1),
                        cursor::MoveToColumn(xc),
                    }?
                }
                queue! {
                    w,
                    style::ResetColor,
                }?;
            }
        }
    }
    io::Result::Ok(())
}

impl GameState {
    fn draw_player_board<W>(&self, w: &mut W, x: u16, y: u16) -> io::Result<()>
    where
        W: io::Write
        {
            draw_board(&self.player_board, w, x, y)
        }

    fn draw_enemy_board<W>(&self, w: &mut W, x: u16, y: u16) -> io::Result<()>
    where
        W: io::Write
        {
            draw_board(&self.enemy_board, w, x, y)
        }

    /// Ankor: bottom-center
    fn draw_hand<W>(&self, w: &mut W, x: u16, y: u16) -> io::Result<()>
    where
        W: io::Write
        {
            let xc = x - (self.player_cards.len() as u16 * CARD_WIDTH) /2;
            let yc = y - CARD_HEIGHT;
            for (i, card) in self.player_cards.iter().enumerate() {
                card.draw(w, xc + CARD_WIDTH * i as u16, yc)?;
                queue!(
                    w,
                    cursor::MoveTo(xc + CARD_WIDTH * i as u16 + CARD_WIDTH / 2, yc -1),
                    style::Print(i)
                    )?;
            }

            io::Result::Ok(())
        }

}


fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    let mut state = GameState::default();

    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let mut card = Card::new("Squirrel", 1, 1, "");
    card.apply_effect(EffectType::Poison, 2);
    card.apply_effect(EffectType::Curse, 2);
    let card2 = Card::new("Joe", 3, 2, "Joe");

    // state.player_board[1][1] = Some(card);
    // state.player_board[0][2] = Some(card2);
    state.player_cards.push(card);
    state.player_cards.push(card2);

    state.dirty = true;

    let mut size = terminal::size().unwrap();

    loop {

        if state.dirty {
            queue!(
                w,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::Hide,
                cursor::MoveTo(0, 0)
                )?;


            if size.0 < MIN_X || size.1 < MIN_Y {
                let text = format!("{}x{}", size.0, size.1);
                let required = format!("Required size: {}x{}", MIN_X, MIN_Y);
                queue!(
                    w,
                    cursor::MoveTo(size.0/2 - text.len() as u16/2, size.1/2 - 1),
                    style::Print(text),
                    cursor::MoveTo(size.0/2 - 10, size.1/2),
                    style::Print("Terminal is too small"),
                    cursor::MoveTo(size.0/2 - required.len() as u16 / 2, size.1/2 + 1),
                    style::Print(required),
                    )?;
                w.flush()?;
            } else {
                state.draw_player_board(w, size.0 / 2 - BOARD_WIDTH - CENTER_BOARD_MARGIN /2, 0)?;
                state.draw_enemy_board(w, size.0 / 2 + CENTER_BOARD_MARGIN /2, 0)?;
                state.draw_hand(w, size.0 / 2, size.1)?;

                w.flush()?;
            }

             state.dirty = false
        }
        let event = read()?;

        match event {
            Event::Key(key) => {
               match key.code {
                    KeyCode::Char('q') => {
                         execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                         break;
                    },
                    KeyCode::Char('0') => {
                         unimplemented!();
                    },
                    KeyCode::Char('n') => {
                        icons::next_style();
                        state.dirty = true;
                    },
                    _ => {}
               }
            }, 
            Event::Resize(x, y) => {
                size = (x, y);
                state.dirty = true;
            },
            _ => {}
        }

        //if event == Event::Key(KeyCode::Char('q').into()) {
        //    execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
        //    break;
        //}

    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> io::Result<(u16, u16)> {
    terminal::size()
}

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
