use cursive::theme::{BaseColor, Color, PaletteColor, Theme};

pub fn setup_theme() -> Theme {
    let mut theme = Theme::default();

    // Couleurs plus douces et professionnelles
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Cyan);
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Blue);
    theme.palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Cyan);
    theme.palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::Blue);
    theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Cyan);
    theme.palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Blue);

    theme
}
