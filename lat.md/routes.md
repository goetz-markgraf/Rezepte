# Routes

Alle HTTP-Endpunkte der Anwendung. Registriert in [[src/routes/mod.rs#create_router]].

## Rezept-Routen

CRUD-Endpunkte für Rezepte inkl. Duplikat- und Merge-Funktionen.

| Method | Path                         | Handler                        | Beschreibung                        |
|--------|------------------------------|--------------------------------|-------------------------------------|
| GET    | `/`                          | `recipes::index`               | Rezeptliste mit Filtern             |
| GET    | `/recipes/new`               | `recipes::new_recipe_form`     | Formular: neues Rezept              |
| POST   | `/recipes`                   | `recipes::create_recipe_handler` | Rezept erstellen                  |
| GET    | `/recipes/:id`               | `recipes::show_recipe`         | Rezept-Detailansicht                |
| POST   | `/recipes/:id`               | `recipes::update_recipe_handler` | Rezept aktualisieren              |
| GET    | `/recipes/:id/edit`          | `recipes::edit_recipe_form`    | Formular: Rezept bearbeiten         |
| GET    | `/recipes/:id/confirm-delete`| `recipes::confirm_delete`      | Löschbestätigung                    |
| POST   | `/recipes/:id/delete`        | `recipes::delete_recipe_handler` | Rezept löschen                    |
| GET    | `/recipes/check-duplicate`   | `recipes::check_duplicate`     | Live-Duplikat-Check (HTMX)          |
| GET    | `/recipes/duplicates`        | `recipes::duplicates_handler`  | Duplikat-Übersicht                  |
| GET    | `/recipes/merge`             | `recipes::merge_form_handler`  | Merge-Formular                      |
| POST   | `/recipes/merge`             | `recipes::merge_handler`       | Merge ausführen                     |

## Filter-Routen

Endpunkte zum Verwalten gespeicherter Filter.

| Method | Path                        | Handler                              | Beschreibung                  |
|--------|-----------------------------|--------------------------------------|-------------------------------|
| POST   | `/saved-filters`            | `recipes::create_saved_filter_handler` | Filter speichern            |
| POST   | `/saved-filters/:id/delete` | `recipes::delete_saved_filter_handler` | Gespeicherten Filter löschen |

## Seiten-Routen

Toplevels Seiten: Wochenvorschau, Heute und Health-Check.

| Method | Path              | Handler                          | Beschreibung              |
|--------|-------------------|----------------------------------|---------------------------|
| GET    | `/wochenvorschau` | `wochenvorschau::wochenvorschau_handler` | Wochenübersicht   |
| GET    | `/heute`          | `heute::heute_handler`           | Heutige Rezepte           |
| GET    | `/health`         | `health_check`                   | Health-Check-Endpunkt      |

## Test-API-Routen

Nur für E2E-Tests. Erlauben das Zurücksetzen und Befüllen der Datenbank aus dem Testprozess.

| Method | Path                        | Beschreibung              |
|--------|-----------------------------|---------------------------|
| POST   | `/api/test/clear-recipes`   | Alle Rezepte löschen      |
| POST   | `/api/test/seed-recipe`     | Test-Rezept anlegen       |

## URL-Design

Filterzustand (Kategorien, Suche, Spezialfilter, eingeklappter Filter) wird vollständig in URL-Query-Parametern gehalten. Dadurch sind alle Ansichten direkt verlinkbar und über den Browser-Verlauf navigierbar.
