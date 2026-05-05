/// Bestimmt ein Emoji für ein Rezept basierend auf Titel, Zutaten und Anleitung.
///
/// Gibt `None` zurück wenn Zutaten und Anleitung beide leer oder nur Whitespace sind.
/// Gibt ein passendes Emoji zurück basierend auf Schlüsselwörtern oder Fallback.
/// Der Titel dient nur als Keywords, aber nicht als "hat Inhalt"-Check.
pub fn recipe_emoji(
    title: &str,
    ingredients: Option<&str>,
    instructions: Option<&str>,
) -> Option<&'static str> {
    // Prüfen ob Zutaten UND Anleitung beide leer/whitespace sind
    let ingredients_trimmed = ingredients.map(|s| s.trim()).filter(|s| !s.is_empty());
    let instructions_trimmed = instructions.map(|s| s.trim()).filter(|s| !s.is_empty());

    if ingredients_trimmed.is_none() && instructions_trimmed.is_none() {
        return None;
    }

    // Zutaten und Anleitung zusammenführen für Keyword-Matching
    let all_text = [
        ingredients_trimmed.unwrap_or(""),
        instructions_trimmed.unwrap_or(""),
        title.trim(),
    ]
    .join(" ");
    let lower = all_text.to_lowercase();

    // Keywords -> Emoji Mapping (spezifische -> allgemeine Reihenfolge)
    let keywords: &[(&str, &str)] = &[
        ("nudel", "🍝"),
        ("pasta", "🍝"),
        ("spaghetti", "🍝"),
        ("bolognese", "🍝"),
        ("carbonara", "🍝"),
        ("pizza", "🍕"),
        ("brot", "🍞"),
        ("brötchen", "🥖"),
        ("baguette", "🥖"),
        ("kuchen", "🎂"),
        ("torte", "🎂"),
        ("brownie", "🍫"),
        ("schokolad", "🍫"),
        ("chokolade", "🍫"),
        ("salat", "🥗"),
        ("suppe", "🍲"),
        ("gulasch", "🍲"),
        ("eintopf", "🍲"),
        ("curry", "🍛"),
        ("reis", "🍚"),
        ("gemüse", "🥬"),
        ("fleisch", "🍖"),
        ("hähnchen", "🍗"),
        ("huhn", "🍗"),
        ("fisch", "🐟"),
        ("lachs", "🐟"),
        ("party", "🎉"),
        ("feier", "🎉"),
        ("bratwurst", "🌭"),
        ("imbiss", "🌭"),
        ("kaffee", "☕"),
        ("tee", "🍵"),
        ("apfel", "🍎"),
        ("äpfel", "🍎"),
        ("banane", "🍌"),
        ("beere", "🫐"),
        ("erdbeer", "🍓"),
        ("schnecken", "🥐"),
        ("croissant", "🥐"),
        ("weihnacht", "🍪"),
        ("plätzchen", "🍪"),
        ("kekse", "🍪"),
        ("eis", "🍦"),
        ("frühstück", "🍳"),
        ("morgenessen", "🍳"),
    ];

    for (keyword, emoji) in keywords {
        if lower.contains(keyword) {
            return Some(emoji);
        }
    }

    // Fallback: 🍽️ für allgemeine Rezepte mit Inhalt
    Some("🍽️")
}

/// Bestimmt ein Kategorie-gebundenes Emoji (für Rezepte ohne Inhalt, aber mit Kategorie).
/// Gibt `None` wenn keine Kategorie passt.
#[allow(dead_code)]
pub fn emoji_from_category(category: &str) -> Option<&'static str> {
    match category {
        "Mittagessen" => Some("🥘"),
        "Brot" => Some("🍞"),
        "Kuchen" => Some("🎂"),
        "Party" => Some("🎉"),
        "Snacks" => Some("🍿"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_emoji_returns_none_for_empty_content() {
        // Given: Keine Zutaten, keine Anleitung
        let result = recipe_emoji("Pizza", None, None);
        assert!(result.is_none());
    }

    #[test]
    fn recipe_emoji_returns_none_for_whitespace_only() {
        let result = recipe_emoji("Brot", Some("  \n  "), Some("  "));
        assert!(result.is_none());
    }

    #[test]
    fn recipe_emoji_matches_pasta_keyword() {
        let result = recipe_emoji("Selbstgemachte Nudeln", Some("Mehl, Eier"), None);
        assert_eq!(result, Some("🍝"));
    }

    #[test]
    fn recipe_emoji_matches_pizza_keyword() {
        let result = recipe_emoji("Hausgemachte Pizza", Some("Mehl, Tomaten"), None);
        assert_eq!(result, Some("🍕"));
    }

    #[test]
    fn recipe_emoji_matches_bread_keyword() {
        let result = recipe_emoji("Selbstgebackenes Brot", Some("Mehl, Hefe"), None);
        assert_eq!(result, Some("🍞"));
    }

    #[test]
    fn recipe_emoji_matches_kuchen_keyword() {
        let result = recipe_emoji("Schokoladenkuchen", Some("Schokolade, Mehl"), None);
        assert_eq!(result, Some("🎂"));
    }

    #[test]
    fn recipe_emoji_matches_in_ingredients() {
        let result = recipe_emoji("Gemüsepfanne", Some("Paprika, Brokkoli, Gemüse"), None);
        assert_eq!(result, Some("🥬"));
    }

    #[test]
    fn recipe_emoji_matches_apfel_in_instructions() {
        let result = recipe_emoji("Leckerbissen", None, Some("Äpfel schälen"));
        assert_eq!(result, Some("🍎"));
    }

    #[test]
    fn recipe_emoji_falls_back_for_content_with_no_keyword() {
        let result = recipe_emoji(
            "Hübsche Leckerbissen",
            Some("Etwas Ungewöhnliches"),
            Some("Mischen"),
        );
        assert_eq!(result, Some("🍽️"));
    }

    #[test]
    fn recipe_emoji_case_insensitive() {
        let result = recipe_emoji("Gemüsepfanne", Some("SUPPE vom Feld"), None);
        assert_eq!(result, Some("🍲"));
    }

    #[test]
    fn emoji_from_category_returns_correct_emoji() {
        assert_eq!(emoji_from_category("Brot"), Some("🍞"));
        assert_eq!(emoji_from_category("Kuchen"), Some("🎂"));
        assert_eq!(emoji_from_category("Mittagessen"), Some("🥘"));
        assert_eq!(emoji_from_category("Party"), Some("🎉"));
        assert_eq!(emoji_from_category("Snacks"), Some("🍿"));
    }

    #[test]
    fn emoji_from_category_returns_none_for_unknown() {
        assert!(emoji_from_category("Unbekannt").is_none());
    }

    #[test]
    fn recipe_emoji_priority_keyword_over_fallback() {
        let result = recipe_emoji("Brot mit Käse", Some("Weizenmehl"), Some("Kneten"));
        assert_eq!(result, Some("🍞"));
    }
}
