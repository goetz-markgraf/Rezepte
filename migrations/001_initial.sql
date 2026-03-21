CREATE TABLE recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    categories TEXT,
    ingredients TEXT,
    instructions TEXT,
    rating INTEGER CHECK (rating BETWEEN 1 AND 5),
    planned_date DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_recipes_title ON recipes(title);
CREATE INDEX idx_recipes_planned_date ON recipes(planned_date);