# Story 26: Nutzung von Icons

**Epic:** Epic 2: Rezept-Übersicht & Navigation
**Priorität:** Nice-to-have (UX-Verbesserung)
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **Icons bei Aktionen und Navigationselementen sehen**, damit ich **die Benutzeroberfläche schneller erfasse und häufig genutzte Funktionen auf einen Blick erkenne**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die Rezepte-App wird auf verschiedenen Geräten genutzt (Handy, Tablet, Laptop), insbesondere auch mobil beim Eintippen neuer Rezepte oder bei der Wochenplanung. Derzeit bestehen alle interaktiven Elemente aus reinen Textbeschriftungen. Icons verbessern die Orientierung, verkürzen die kognitive Erfassungszeit und erleichtern vor allem die mobile Nutzung, wo wenig Platz für lange Beschriftungen ist.

Auf Buttons außerhalb von Dialogen (z.B. Bearbeiten, Löschen, Zurück) können Icons die Textbeschriftung ergänzen oder bei eindeutig verständlichen Icons die alleinige visuelle Kommunikation übernehmen. In Dialogen und Formularen bleiben Beschriftungen erhalten und werden durch Icons ergänzt. Die Icons werden als SVG direkt inline in die Askama-Templates eingebettet oder über eine zentrale Makro-Datei wiederverwendet – keine CDN-Abhängigkeit.

Als Icon-Bibliothek wird **Lucide** verwendet, da sie im Architecture Document explizit genannt ist, quelloffen, SVG-basiert und gut lesbar ist.

### Nutzergruppe

Beide Partner des Haushalts – auf Desktop, Tablet und Handy. Besonders profitieren mobile Nutzer vom reduzierten Platzbedarf durch Icon-gestützte Buttons.

### Business-Value

- Schnelleres Erfassen von Aktionen auf Listenebene (z.B. Bearbeiten- und Löschen-Button direkt in der Rezeptliste)
- Bessere mobile Nutzbarkeit durch kompaktere Darstellung
- Konsistente visuelle Sprache für alle Aktionstypen in der App
- Orientierung für Nutzer, die sich noch nicht alle Menüpunkte gemerkt haben

### Edge Cases

- **Icons ohne Beschriftung:** Wenn ein Icon ohne sichtbaren Text verwendet wird (z.B. nur Löschen-Icon), muss ein `aria-label` oder `title`-Attribut am Button vorhanden sein, damit Screenreader und Tastaturnutzer die Aktion verstehen.
- **Nicht geladene SVGs (Inline):** Da SVGs inline eingebettet werden, gibt es keinen Ladeausfall – das SVG ist immer vorhanden, wenn das HTML gerendert wird.
- **Unbekannte Aktionen:** Aktionen, für die kein etabliertes Icon-Konzept existiert, erhalten weiterhin reine Textbeschriftung. Kein Icon ist besser als ein unklares Icon.
- **Verkleinerte Darstellung auf kleinen Bildschirmen:** Icons müssen auch bei 16px noch erkennbar sein. Richtwert: 20×20px als Mindestgröße.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Lucide-Icons als SVG-Inline eingebunden**
  - Ausgewählte Icons aus der Lucide-Bibliothek sind als SVG direkt inline in den Askama-Templates vorhanden
  - Keine externe CDN-Abhängigkeit, keine separaten Icon-Font-Dateien

- [ ] **K2: Icons auf Aktions-Buttons**
  - Der "Bearbeiten"-Button zeigt ein Stift-Icon (z.B. `pencil`)
  - Der "Löschen"-Button zeigt ein Papierkorb-Icon (z.B. `trash-2`)
  - Der "Neues Rezept"-Button zeigt ein Plus-Icon (z.B. `plus`)
  - Der "Zurück zur Liste"-Link zeigt ein Pfeil-zurück-Icon (z.B. `arrow-left`)
  - Der Suche-Button / Suche-Eingabefeld zeigt ein Lupen-Icon (z.B. `search`)

- [ ] **K3: Icons in der Navigation**
  - Navigationselemente erhalten passende Icons (z.B. Haus-Icon für Startseite, falls vorhanden)

- [ ] **K4: Icons in Dialogen mit Textbeschriftung**
  - Buttons in Bestätigungs-Dialogen (z.B. Lösch-Sicherheitsabfrage) zeigen Icon + Beschriftung
  - Beispiel: Trash-Icon + "Wirklich löschen", X-Icon + "Abbrechen"

- [ ] **K5: Bewertungssterne als Icons**
  - Die Sterne-Bewertung (1–5 Sterne) verwendet SVG-Stern-Icons (z.B. `star` gefüllt / ungefüllt)
  - Ausgefüllte Sterne signalisieren die aktuelle Bewertung

- [ ] **K6: Kategorie-Filter-Buttons mit optionalem Icon**
  - Kategorie-Filter-Buttons (Mittagessen, Brot, etc.) können ein passendes Icon erhalten, falls ein sinnvolles existiert

### Nicht-funktionale Kriterien

- [ ] **K7: Accessibility – Icons ohne sichtbaren Text**
  - Jeder Button/Link, der nur ein Icon ohne sichtbaren Text zeigt, hat ein `aria-label`-Attribut mit der Aktionsbeschreibung (z.B. `aria-label="Rezept bearbeiten"`)
  - Alternativ: ein `<title>`-Element im SVG

- [ ] **K8: Accessibility – Fokus-Sichtbarkeit**
  - Icon-Buttons sind per Tastatur fokussierbar und haben einen sichtbaren Fokus-Indikator (WCAG 2.1 Level A)

- [ ] **K9: Performance**
  - Inline-SVGs verursachen keinen zusätzlichen HTTP-Request
  - Seite lädt weiterhin in < 500ms (kein messbarer Overhead durch Icon-SVGs)

- [ ] **K10: Mindestgröße der Klick-/Touch-Fläche**
  - Icon-Buttons haben eine Touch-Fläche von mindestens 44×44px (WCAG 2.5.5 empfohlen) auf mobilen Geräten

---

## 4. Technische Planung

### Datenmodell

Keine Datenbankänderungen erforderlich. Icons sind rein ein UI-Thema.

### UI/UX-Spezifikation

**Icon-Einbindung via Askama-Makro:**

Icons werden als Askama-Makros (`templates/components/icons.html`) zentralisiert, sodass jedes Icon nur einmal gepflegt werden muss und per `{% call icon_pencil() %}` in Templates wiederverwendet werden kann.

**Icon-Zuordnung (Lucide):**

| Aktion / Element       | Lucide-Icon       | Anzeige                       |
|------------------------|-------------------|-------------------------------|
| Bearbeiten             | `pencil`          | Icon allein + `aria-label`    |
| Löschen                | `trash-2`         | Icon allein + `aria-label`    |
| Neues Rezept           | `plus`            | Icon + Text "Neues Rezept"    |
| Suche                  | `search`          | Icon im Eingabefeld           |
| Zurück                 | `arrow-left`      | Icon + Text oder allein       |
| Bestätigen / Speichern | `check`           | Icon + Text in Dialogen       |
| Abbrechen              | `x`               | Icon + Text in Dialogen       |
| Bewertungsstern        | `star`            | Gefüllt/ungefüllt per CSS     |
| Startseite / Home      | `home`            | Icon + Text in Navigation     |

**Größen:**
- Standard-Icons: 20×20px (`width="20" height="20"`)
- In Text eingebettet (z.B. Button): `1em` Größe mit `vertical-align: middle`
- Sterne: 16×16px oder 20×20px je nach Kontext

**CSS-Klassen:**
- `.icon` – Basis-Klasse für alle Icons (display: inline-block, vertical-align: middle)
- `.btn-icon` – Button mit nur Icon (kein sichtbarer Text, ausreichende Touch-Fläche)
- `.btn-icon-text` – Button mit Icon + sichtbarem Text

**Technische Umsetzung:**

Da Askama zur Compile-Zeit rendert, werden die SVG-Inhalte direkt in den Templates hinterlegt. Eine dedizierte Makro-Datei (`templates/components/icons.html`) enthält alle verwendeten Icon-SVGs. Jedes Template importiert diese Makros nach Bedarf.

Alternativ kann eine Rust-Hilfsfunktion oder ein Askama-Filter die SVG-Strings liefern. Die konkrete Umsetzung entscheidet der Entwickler im Implementierungs-Plan.

---

## 5. Nicht-funktionale Anforderungen

### Performance

- Seite lädt ohne sichtbare Verzögerung (< 500ms) – Inline-SVGs haben keinen zusätzlichen HTTP-Overhead
- SVG-Markup pro Icon: typisch 100–300 Bytes – vernachlässigbar bei Seitengrößen im KB-Bereich

### Browser-Support

- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- SVG wird in allen modernen Browsern vollständig unterstützt

### Barrierefreiheit

- WCAG 2.1 Level A konform
- Icon-only-Buttons: `aria-label` Pflicht
- SVGs mit `aria-hidden="true"` wenn daneben sichtbarer Text vorhanden (verhindert doppelte Ansage durch Screenreader)
- Fokus-Indikatoren sichtbar für alle interaktiven Elemente

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Bearbeiten-Button hat Icon und Accessibility-Label**
```gherkin
Given die Rezeptliste ist geöffnet und enthält mindestens ein Rezept
When der Nutzer die Liste betrachtet
Then ist für jeden Eintrag ein Bearbeiten-Button sichtbar
And der Button enthält ein SVG-Element
And der Button hat ein aria-label="Rezept bearbeiten" oder vergleichbares Label
```

**Testfall 2: Löschen-Button mit Icon navigiert zur Sicherheitsabfrage**
```gherkin
Given die Rezeptliste enthält ein Rezept
When der Nutzer auf den Löschen-Icon-Button klickt
Then erscheint die Lösch-Bestätigungsabfrage
And die Bestätigungsabfrage enthält Buttons mit Icon + Text
```

**Testfall 3: Sterne-Icons zeigen korrekte Bewertung**
```gherkin
Given ein Rezept mit einer Bewertung von 4 Sternen existiert
When der Nutzer die Detailansicht öffnet
Then sind 4 ausgefüllte Stern-Icons und 1 ungefülltes Stern-Icon sichtbar
```

**Testfall 4: Suche-Icon im Suchfeld sichtbar**
```gherkin
Given die Startseite / Rezeptliste ist geöffnet
When der Nutzer das Suchfeld betrachtet
Then ist ein Lupen-Icon im oder neben dem Suchfeld sichtbar
```

**Testfall 5: Tastatur-Navigation auf Icon-Buttons**
```gherkin
Given die Rezeptliste enthält Rezepte mit Icon-Buttons
When der Nutzer mit der Tab-Taste navigiert
Then sind alle Icon-Buttons per Tab erreichbar
And der Fokus-Indikator ist sichtbar
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 01 (Rezept erstellen), Story 02 (Rezept bearbeiten), Story 03 (Rezept löschen), Story 04 (Detailansicht), Story 05 (Rezeptliste) müssen implementiert sein – Icons werden den bestehenden UI-Elementen hinzugefügt
- Keine neuen externen Bibliotheken zur Laufzeit – Lucide-SVG-Code wird statisch eingebettet

### Rahmenbedingungen

- Keine Authentifizierung erforderlich (LAN-only)
- Keine CDN-Abhängigkeit – alle Icons müssen im Repository vorhanden sein
- Icon-Set: Lucide (MIT-Lizenz) – SVG-Quellcode kann frei verwendet werden
- Progressive Enhancement: Die App muss auch ohne CSS-Klassen für Icons noch funktionieren (Buttons bleiben klickbar, nur optisch unformatiert)

---

## Offene Punkte / Fragen

- [ ] Werden Icons auf Kategorie-Filter-Buttons eingesetzt? Falls ja: welche Icons passen zu Mittagessen, Brot, Party, Kuchen, Snacks?
- [ ] Soll der Bearbeiten-Button in der Rezeptliste nur ein Icon sein (platzsparend) oder Icon + Text?
- [ ] Werden Bewertungssterne interaktiv (klickbar in der Liste) oder nur in der Detailansicht/Bearbeitungsansicht?

---

**Letzte Aktualisierung:** 2026-03-28
