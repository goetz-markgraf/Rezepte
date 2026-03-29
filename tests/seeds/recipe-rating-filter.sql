-- Seed-Daten für Story 11: Filter nach Bewertung
INSERT INTO recipes (title, categories, rating, planned_date) VALUES
    ('Fünf-Sterne-Gericht', '["Mittagessen"]', 5, NULL),
    ('Vier-Sterne-Gericht', '["Mittagessen"]', 4, '2025-01-01'),
    ('Drei-Sterne-Gericht', '["Brot"]', 3, NULL),
    ('Zwei-Sterne-Gericht', '["Snacks"]', 2, '2024-06-01'),
    ('Ein-Stern-Gericht', '["Party"]', 1, NULL),
    ('Unbewertetes-Gericht', '["Kuchen"]', NULL, NULL);
