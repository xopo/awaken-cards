use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::models::{Card, FlashCardData};

#[derive(Clone, Copy, PartialEq)]
pub enum FocusPane {
    Cards,
    Decks,
}

pub struct App {
    // Zero position inside the collection
    pub current_card: usize,

    // Owns all imported categories and cards
    pub data: FlashCardData,

    // None - means All Cards from all decks
    // Some(id) means a specific card is selected
    pub selected_category: Option<u8>,

    // True - front, False - back
    pub showing_back: bool,

    // Which pane has the focus
    focus_pane: FocusPane,
}

impl App {
    pub fn new(data: FlashCardData) -> Self {
        Self {
            current_card: 0,
            data,
            focus_pane: FocusPane::Cards,
            selected_category: None,
            showing_back: false,
        }
    }

    pub fn pane_is_active(&self, pane: FocusPane) -> bool {
        self.focus_pane == pane
    }

    pub fn visible_cards(&self) -> Vec<&Card> {
        match self.selected_category {
            Some(id) => self
                .data
                .cards
                .iter()
                .filter(|card| card.category == id + 1)
                .collect(),
            None => self.data.cards.iter().collect(),
        }
    }

    pub fn next_card(&mut self) {
        let cards_len = self.visible_cards().len() - 1;
        self.current_card = if self.current_card < cards_len {
            self.current_card + 1
        } else {
            0
        };
        self.showing_back = false;
    }

    pub fn previous_card(&mut self) {
        let cards_len = self.visible_cards().len() - 1;
        self.current_card = if self.current_card > 1 {
            self.current_card - 1
        } else {
            cards_len
        };
        self.showing_back = false;
    }

    pub fn flip_card(&mut self) {
        self.showing_back = !self.showing_back
    }

    pub fn next_category(&mut self) {
        match self.selected_category {
            None => self.selected_category = Some(0),
            Some(cat) if cat < (self.data.categories.len() - 1) as u8 => {
                self.selected_category = Some(cat + 1)
            }
            _ => self.selected_category = None,
        }
        self.current_card = 0
    }

    pub fn swap_pane(&mut self) {
        match self.focus_pane {
            FocusPane::Cards => self.focus_pane = FocusPane::Decks,
            FocusPane::Decks => self.focus_pane = FocusPane::Cards,
        }
    }

    pub fn previous_category(&mut self) {
        match self.selected_category {
            None => self.selected_category = Some(self.data.categories.len() as u8 - 1),
            Some(cat) if cat > 0 => self.selected_category = Some(cat - 1),
            _ => self.selected_category = None,
        }
        self.current_card = 0
    }

    pub fn current_card(&self) -> Option<&Card> {
        self.visible_cards().get(self.current_card).copied()
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.kind != KeyEventKind::Press {
            return false;
        }

        if key.code == KeyCode::Char('q') {
            return true;
        }

        match key.code {
            KeyCode::Tab => {
                self.swap_pane();
                false
            }

            KeyCode::Char('f') => {
                self.flip_card();
                false
            }

            KeyCode::Char('l') | KeyCode::Right => {
                self.next_card();
                false
            }
            KeyCode::Char('h') | KeyCode::Left => {
                self.previous_card();
                false
            }
            _ => false,
        };

        match self.focus_pane {
            FocusPane::Cards => self.handle_card_key(key),
            FocusPane::Decks => self.handle_deck_key(key),
        }
    }

    pub fn handle_card_key(&mut self, _key: KeyEvent) -> bool {
        false
    }

    pub fn handle_deck_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous_category();
                return false;
            }

            KeyCode::Down | KeyCode::Char('j') => {
                self.next_category();
                return false;
            }
            _ => false,
        };
        false
    }
}
