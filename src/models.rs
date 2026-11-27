use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Category {
    pub id: u8,
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct Card {
    pub category: u8,
    pub prompt: String,
    pub text: String,
}

#[derive(Deserialize, Clone)]
pub struct FlashCardData {
    pub categories: Vec<Category>,
    pub cards: Vec<Card>,
}

impl FlashCardData {
    pub fn cards_in_category(&self, id: u8) -> Vec<&Card> {
        self.cards
            .iter()
            .filter(|card| card.category == id)
            .collect()
    }

    pub fn category_by_id(&self, id: u8) -> Option<&Category> {
        self.categories.iter().find(|cat| cat.id == id)
    }
}
