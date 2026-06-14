# Story 47: Rezept aus Foto erstellen

**Epic:** Epic 1: Rezept-Verwaltung (Grundlegendes CRUD)
**Status:** Abgeschlossen

---

## 1. Story-Satz

Als **Nutzerin** möchte ich **ein Foto eines Rezepts hochladen oder direkt mit der Kamera aufnehmen**, damit ich **Rezepte schnell digitalisieren kann, ohne alle Felder manuell eintippen zu müssen**.

---

## 2. Weiterer Kontext

Viele Rezepte liegen als Fotos vor (Kochbücher, ausgeschnittene Zeitschriften, handgeschriebene Zettel). Das manuelle Abtippen ist zeitaufwendig und fehleranfällig. Die App soll ein Vision-fähiges KI-Modell nutzen, um den Text und die Struktur aus dem Foto zu extrahieren und das Neue-Rezept-Formular vorzubefüllen. Das Speichern liegt weiterhin beim User – er kann die Felder vor dem Speichern prüfen und korrigieren.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

**Foto-Upload-Seite:**

```gherkin
Angenommen die Nutzerin ist auf der Rezeptliste
Wenn sie auf „Neues Rezept" klickt
Dann sieht sie zwei Optionen: „Manuell eingeben" und „Aus Foto erstellen"

Angenommen die Nutzerin wählt „Aus Foto erstellen"
Dann landet sie auf der Upload-Seite /recipes/from-photo
Und sieht ein Datei-Upload-Feld (Bilder: JPEG, PNG, WEBP, HEIC)
Und sieht auf Geräten mit Kamera einen „Foto aufnehmen"-Button (capture=camera)
Und sieht einen „Analysieren"-Button

Angenommen die Nutzerin wählt eine Datei und klickt „Analysieren"
Wenn der Server das Foto erfolgreich ans Vision-Modell schickt
Dann landet sie auf dem Neue-Rezept-Formular (/recipes/new)
Und das Formular ist mit Titel, Zutaten, Anleitung und Kategorie vorausgefüllt
Und sie kann alle Felder noch bearbeiten
Und sie kann das Rezept wie gewohnt speichern
```

**Fehlerbehandlung:**

```gherkin
Angenommen die Nutzerin schickt das Formular ohne Datei
Dann bleibt sie auf der Upload-Seite
Und sieht eine Fehlermeldung „Bitte ein Foto auswählen"

Angenommen die API antwortet mit einem Fehler (Timeout, kein API-Key, etc.)
Dann bleibt die Nutzerin auf der Upload-Seite
Und sieht eine Fehlermeldung „Foto konnte nicht analysiert werden. Bitte erneut versuchen oder das Rezept manuell eingeben."

Angenommen kein VISION_API_KEY und keine VISION_API_URL sind konfiguriert
Dann ist der Menüpunkt „Aus Foto erstellen" nicht sichtbar
```

### Nicht-funktionale Kriterien

- Dateigröße: max. 10 MB
- Timeout: 30 Sekunden für die API-Anfrage
- Kein Speichern der hochgeladenen Fotos (nur als Bytes im Speicher, kein Temp-File)
- API-Konfiguration via Umgebungsvariablen: `VISION_API_URL`, `VISION_API_KEY`, `VISION_MODEL` (optional, Default: `gpt-4o`)
- Das Feature ist optional: fehlt die Konfiguration, funktioniert der Rest der App normal

---

## Zusatzinformationen

Man soll ein neues Rezept aus einem Foto nehmen können.

Dazu soll ein Foto entweder hochgeladen oder, wenn Kamera verfügbar, z. B. auf einem Handy, eines aufgenommen werden. Das
Foto wird dann über eine OpenAI-Kompatible Schnittstelle an ein Model mit Vision-Fähigkeit geleitet. Dieses Modell soll aus
dem Foto vier notwendige Angaben herausziehen:

- Name
- Zutaten
- Beschreibung der Zubereitung
- Zu welcher der Kategorien es gehört.

Als Ergebnis soll ein Formular wie bei "neues Rezept" gefüllt werden. Das Speichern übernimmt dann der User.
