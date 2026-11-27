use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, FocusPane};

pub fn render(frame: &mut Frame, app: &App) {
    render_cards(frame, app);
    render_decks(frame, app);
}

pub fn render_cards(frame: &mut Frame, app: &App) {
    let [card_area, _] = get_areas(frame);

    let total = app.visible_cards().len();
    let side = if app.showing_back { "Back" } else { "Front" };
    let card_title = if total == 0 {
        format!(" Card 1/{total} [{side}] ")
    } else {
        format!(" Card {}/{total} [{side}] ", app.current_card + 1)
    };

    let border_style = pane_border_style(app.pane_is_active(FocusPane::Cards));
    let card_box = Block::bordered()
        .title(card_title)
        .border_style(border_style);

    let inner_card = card_box.inner(card_area);
    let [body_area, shortcut_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(inner_card);

    let body_text = match app.current_card() {
        Some(card) => {
            if app.showing_back {
                card.prompt.as_str()
            } else {
                card.text.as_str()
            }
        }
        None => "No cards available.",
    };

    let line_count = body_text.lines().count().max(1) as u16;
    let text_height = line_count.min(body_area.height);

    let [_top, centered_body_area, _bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(text_height),
        Constraint::Fill(1),
    ])
    .areas(body_area);

    let body_line = Line::from(body_text).alignment(Alignment::Center).style(
        Style::default()
            .fg(if app.showing_back {
                Color::Cyan
            } else {
                Color::LightMagenta
            })
            .add_modifier(Modifier::BOLD),
    );

    let card_widget = Paragraph::new(body_line).wrap(Wrap { trim: false });
    let shortcuts_text = Line::from("f: flip · ← prev card · → next card")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    let shortcuts_widget = Paragraph::new(shortcuts_text);

    frame.render_widget(card_box, card_area);
    frame.render_widget(card_widget, centered_body_area);
    frame.render_widget(shortcuts_widget, shortcut_area);
}

pub fn render_decks(frame: &mut Frame, app: &App) {
    let [_, deck_area] = get_areas(frame);

    let active_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let mut items = vec![if app.selected_category.is_none() {
        ListItem::new(">All Cards").style(active_style)
    } else {
        ListItem::new(" All Cards")
    }];

    for (idx, category) in app.data.categories.iter().enumerate() {
        let is_selected = Some(idx as u8) == app.selected_category;
        let item = if is_selected {
            ListItem::new(format!(">{}", category.title)).style(active_style)
        } else {
            ListItem::new(format!(" {}", category.title))
        };

        items.push(item)
    }

    let deck_title = format!(
        " Decks {}/{}",
        app.selected_category.map_or(0, |cat| cat + 1),
        app.data.categories.len()
    );

    let border_style = pane_border_style(app.pane_is_active(FocusPane::Decks));
    let deck_block = Block::bordered()
        .title(deck_title)
        .border_style(border_style);

    let inner_deck = deck_block.inner(deck_area);
    let deck_list = List::new(items);

    let shortcuts_text = Line::from("↑ up ↓ down")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    let [list_area, _space, shortcut_area] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner_deck);
    frame.render_widget(deck_block, deck_area);
    frame.render_widget(deck_list, list_area);
    frame.render_widget(shortcuts_text, shortcut_area);
}

pub fn get_areas(frame: &Frame) -> [Rect; 2] {
    let [app_area, _bottom_space] =
        Layout::vertical([Constraint::Max(23), Constraint::Fill(1)]).areas(frame.area());

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .areas(app_area)
}

fn pane_border_style(is_active: bool) -> Style {
    if is_active {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}
