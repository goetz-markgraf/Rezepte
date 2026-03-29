-- Seed-Daten für Story 14: Rezept-Bewertung
-- Enthält Rezepte mit verschiedenen Bewertungen für E2E-Tests

INSERT INTO recipes (title, categories, ingredients, instructions, rating) VALUES
    ('Fünf-Sterne-Rezept', '["Mittagessen"]', 'Zutaten A', 'Anleitung A', 5),
    ('Drei-Sterne-Rezept', '["Kuchen"]', 'Zutaten B', 'Anleitung B', 3),
    ('Unbewertetes Rezept', '["Brot"]', 'Zutaten C', 'Anleitung C', NULL);
