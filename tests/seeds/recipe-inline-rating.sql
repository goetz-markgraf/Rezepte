-- Seed-Daten für Story 17: Inline-Bewertung
-- Enthält Rezepte für verschiedene Inline-Rating-Szenarien

INSERT INTO recipes (title, categories, rating) VALUES
    ('Testrezept ohne Bewertung', '["Mittagessen"]', NULL),
    ('Testrezept mit 3 Sternen', '["Kuchen"]', 3),
    ('Testrezept mit 4 Sternen', '["Party"]', 4);
