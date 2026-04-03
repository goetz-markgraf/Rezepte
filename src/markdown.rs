/// Rendert Markdown-Text zu HTML und sanitiert das Ergebnis gegen XSS.
///
/// Unterstützte Erweiterungen: Tabellen, Durchgestrichen, Aufgabenlisten (Checkboxen).
use pulldown_cmark::{html, Options, Parser};

/// Wandelt Markdown-Rohtext in einen HTML-String um.
///
/// # Beispiel
/// ```
/// let html = rezepte::markdown::render_markdown("**fett**");
/// assert!(html.contains("<strong>"));
/// ```
pub fn render_markdown(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Rendert Markdown und sanitiert das HTML gegen XSS.
///
/// - `None` → `None` (leeres Feld bleibt leer, Sektion wird ausgeblendet)
/// - Leerstring / nur Whitespace → `None`
/// - Sonst: Markdown rendern, dann mit ammonia sanitisieren
pub fn render_and_sanitize(input: Option<&str>) -> Option<String> {
    let text = input?.trim();
    if text.is_empty() {
        return None;
    }
    let rendered = render_markdown(text);
    let sanitized = ammonia::Builder::default()
        .add_tags(&["input"])
        .add_tag_attributes("input", &["type", "checked", "disabled"])
        .clean(&rendered)
        .to_string();
    Some(sanitized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aufzaehlung_wird_zu_ul_li() {
        // Given: Markdown-Text mit Aufzählungsliste
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <ul> und <li>
        let html = render_markdown("- Item A\n- Item B");
        assert!(html.contains("<ul>"), "Erwartet <ul> im Output");
        assert!(html.contains("<li>"), "Erwartet <li> im Output");
    }

    #[test]
    fn nummerierte_liste_wird_zu_ol_li() {
        // Given: Markdown mit nummerierter Liste
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <ol> und <li>
        let html = render_markdown("1. Schritt eins\n2. Schritt zwei");
        assert!(html.contains("<ol>"), "Erwartet <ol> im Output");
        assert!(html.contains("<li>"), "Erwartet <li> im Output");
    }

    #[test]
    fn fettschrift_wird_zu_strong() {
        // Given: Markdown mit **fett**
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <strong>
        let html = render_markdown("**fett**");
        assert!(html.contains("<strong>"), "Erwartet <strong> im Output");
    }

    #[test]
    fn kursiv_wird_zu_em() {
        // Given: Markdown mit *kursiv*
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <em>
        let html = render_markdown("*kursiv*");
        assert!(html.contains("<em>"), "Erwartet <em> im Output");
    }

    #[test]
    fn checkbox_leer_enthaelt_input_checkbox() {
        // Given: Markdown mit offener Checkbox `- [ ] Aufgabe`
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <input type="checkbox"
        let html = render_markdown("- [ ] Aufgabe");
        assert!(
            html.contains(r#"type="checkbox""#),
            "Erwartet checkbox input im Output, war: {html}"
        );
    }

    #[test]
    fn checkbox_angehakt_enthaelt_checked() {
        // Given: Markdown mit angehakter Checkbox `- [x] Erledigt`
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält checked-Attribut
        let html = render_markdown("- [x] Erledigt");
        assert!(
            html.contains("checked"),
            "Erwartet checked-Attribut im Output, war: {html}"
        );
    }

    #[test]
    fn ueberschrift_wird_zu_h1() {
        // Given: Markdown mit # Überschrift
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <h1>
        let html = render_markdown("# Überschrift");
        assert!(html.contains("<h1>"), "Erwartet <h1> im Output");
    }

    #[test]
    fn horizontale_linie_wird_zu_hr() {
        // Given: Markdown mit --- (horizontale Linie)
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <hr
        let html = render_markdown("---");
        assert!(html.contains("<hr"), "Erwartet <hr im Output");
    }

    #[test]
    fn inline_code_wird_zu_code() {
        // Given: Markdown mit `code`
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält <code>
        let html = render_markdown("`code`");
        assert!(html.contains("<code>"), "Erwartet <code> im Output");
    }

    #[test]
    fn fliesstext_wird_zu_p() {
        // Given: Reiner Fließtext ohne Markdown-Syntax
        // When: render_markdown aufgerufen wird
        // Then: HTML enthält den Text in einem <p>-Element
        let html = render_markdown("Einfacher Text ohne Formatierung");
        assert!(html.contains("<p>"), "Erwartet <p> im Output");
        assert!(
            html.contains("Einfacher Text"),
            "Erwartet den Text im Output"
        );
    }

    #[test]
    fn none_input_gibt_none_zurueck() {
        // Given: None als Input
        // When: render_and_sanitize aufgerufen wird
        // Then: Ergebnis ist None
        assert_eq!(render_and_sanitize(None), None);
    }

    #[test]
    fn leerstring_input_gibt_none_zurueck() {
        // Given: Leerer String als Input
        // When: render_and_sanitize aufgerufen wird
        // Then: Ergebnis ist None
        assert_eq!(render_and_sanitize(Some("")), None);
    }

    #[test]
    fn nur_whitespace_gibt_none_zurueck() {
        // Given: String aus nur Leerzeichen und Leerzeilen
        // When: render_and_sanitize aufgerufen wird
        // Then: Ergebnis ist None
        assert_eq!(render_and_sanitize(Some("   \n\n  ")), None);
    }

    #[test]
    fn xss_script_tag_wird_entfernt() {
        // Given: HTML-Script-Tag als Input (XSS-Versuch)
        // When: render_and_sanitize aufgerufen wird
        // Then: <script>-Tag fehlt im Output (sanitiert)
        let result = render_and_sanitize(Some("<script>alert(1)</script>")).unwrap_or_default();
        assert!(
            !result.contains("<script>"),
            "XSS: <script>-Tag darf nicht im Output sein, war: {result}"
        );
    }
}
