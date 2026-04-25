-- Migration 003: Entfernt die rating-Spalte aus der recipes-Tabelle
-- Hinweis: Bestehende Bewertungsdaten gehen verloren, da das Feature komplett entfernt wird.

ALTER TABLE recipes DROP COLUMN rating;
