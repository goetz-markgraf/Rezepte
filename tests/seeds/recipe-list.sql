-- Seed-Daten für Story 05: Rezept-Liste alphabetisch sortiert
-- Enthält Rezepte mit verschiedenen Anfangsbuchstaben inkl. Umlauten

INSERT INTO recipes (title, categories, ingredients, instructions) VALUES
    ('Apfelkuchen', '["Kuchen"]', 'Äpfel, Mehl, Zucker', 'Äpfel schälen und backen.'),
    ('Bolognese', '["Mittagessen"]', 'Hackfleisch, Tomaten, Nudeln', 'Sauce zubereiten.'),
    ('Zupfbrot', '["Brot"]', 'Mehl, Hefe, Salz', 'Teig kneten und backen.');
