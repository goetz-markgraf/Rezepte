-- Seed-Daten für Story 36: Markdown-Rendering in der Rezept-Detailansicht
-- Rezept 1: Zutaten als Aufzählungsliste (K1)
INSERT INTO recipes (title, categories, ingredients, instructions) VALUES
    (
        'Pfannkuchen',
        '["Kuchen"]',
        '- 500g Mehl' || char(10) || '- 1 Ei' || char(10) || '- 250ml Milch',
        '1. Teig zubereiten' || char(10) || '2. In der Pfanne backen'
    );

-- Rezept 2: Zutaten mit Checkboxen (K4)
INSERT INTO recipes (title, categories, ingredients, instructions) VALUES
    (
        'Einkaufsliste Kuchen',
        '["Kuchen"]',
        '- [ ] Mehl' || char(10) || '- [x] Eier',
        'Backen nach Rezept.'
    );

-- Rezept 3: Zubereitung mit Fettschrift (K3)
INSERT INTO recipes (title, categories, ingredients, instructions) VALUES
    (
        'Brot mit Hinweis',
        '["Brot"]',
        'Mehl, Hefe, Salz',
        '**Wichtig:** Ofen vorheizen auf 180°C'
    );

-- Rezept 4: Nur Titel, leere Felder (K9)
INSERT INTO recipes (title, categories, ingredients, instructions) VALUES
    (
        'Minimal Rezept',
        '["Snacks"]',
        NULL,
        NULL
    );

-- Rezept 5: Fließtext ohne Markdown-Syntax (K8)
INSERT INTO recipes (title, categories, ingredients, instructions) VALUES
    (
        'Einfache Suppe',
        '["Mittagessen"]',
        'Gemüse, Brühe, Salz und Pfeffer',
        'Gemüse schneiden und in der Brühe weichkochen. Abschmecken und servieren.'
    );
