use crate::models::FlashCardData;

const PATH_TO_CARDS: &str = "./data/cards.json";

pub fn load_cards() -> anyhow::Result<FlashCardData> {
    let content = std::fs::read_to_string(PATH_TO_CARDS)?;
    let data: FlashCardData = serde_json::from_str(&content)?;
    Ok(data)
}
