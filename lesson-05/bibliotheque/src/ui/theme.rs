use ratatui::style::Color;

// Couleurs du thème de l'application
pub struct AppTheme;

impl AppTheme {
    // Couleurs principales
    pub const PRIMARY: Color = Color::Cyan;
    pub const SECONDARY: Color = Color::Blue;
    pub const ACCENT: Color = Color::LightCyan;

    // Couleurs de statut
    pub const SUCCESS: Color = Color::Green;
    pub const ERROR: Color = Color::Red;
    pub const WARNING: Color = Color::Yellow;

    // Couleurs neutres
    pub const BG: Color = Color::Reset;
    pub const FG: Color = Color::White;
    pub const DIM: Color = Color::DarkGray;

    // Couleurs sémantiques pour les livres
    pub const AVAILABLE: Color = Color::Green;
    pub const BORROWED: Color = Color::Red;

    // Couleurs pour les bordures et titres
    pub const BORDER_ACTIVE: Color = Color::Cyan;
    pub const BORDER_INACTIVE: Color = Color::Blue;
    pub const TITLE: Color = Color::Cyan;
}

// Constantes pour les symboles
pub mod symbols {
    pub const BULLET_EMPTY: &str = "○";
    pub const BULLET_FILLED: &str = "●";
    pub const TREE_BRANCH: &str = "└─";
    pub const SELECTOR: &str = ">";
}
