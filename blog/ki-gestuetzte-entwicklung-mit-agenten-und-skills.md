# Von der Story zum Code: Mein Agent-basierter Entwicklungs-Workflow

Generative KI

Goetz Markgraf

**Kann man mit KI-Agenten nicht nur einzelne Code-Schnipsel erzeugen, sondern einen kompletten, strukturierten Entwicklungsprozess automatisieren?**

Vor ein paar Tagen habe ich damit begonnen, eine kleine Web-Anwendung für die Verwaltung von Familienrezepten zu entwickeln. Nichts Weltbewegendes – ein simples CRUD-Tool mit ein paar Filtern und einer Wochenplanungs-Funktion. Aber ich wollte experimentieren: Wie weit kann ich KI-Agenten gehen? Nicht nur beim Coden, sondern beim gesamten Entwicklungsprozess?

Das Ergebnis überrascht mich selbst. Ich habe ein System aus Agents, Skills und Commands aufgebaut, das Stories vom Backlog bis zur fertigen Implementierung durch alle Phasen führt – inklusive Refining, Planung, TDD-Implementierung und Review. Und das Ganze läuft mit einem KI-Modell, das ein Bruchteil dessen kostet, was die großen Player verlangen.

## Das Setup: Mehr als nur ein Chat-Interface

Wenn du heute über KI-gestützte Entwicklung sprichst, denken viele immer noch an ChatGPT, Claude oder GitHub Copilot. Das sind großartige Tools – aber sie bleiben auf Level 1 bis 3 meines [Fünf-Level-Modells](https://www.codecentric.de/wissens-hub/blog/die-fuenf-level-der-ki-gestuetzten-softwareentwicklung). Was ich gebaut habe, bewegt sich auf Level 4: Spec-Driven Development mit automatisierten Agenten -- und es kratzt an Level 5: Voll-Autonome Entwicklung eines Produktes.

Die Grundidee ist simpel: Statt jeden Prompt manuell einzugeben, definiere ich **Skills** – wiederverwendbare Anleitungen für spezifische Aufgaben. Diese Skills werden von **Agenten** geladen, die komplexe Workflows orchestrieren. Und über **Commands** kann ich das ganze System mit einem einzigen Befehl starten.

Hier ist die Struktur, die ich im Ordner `.opencode` aufgebaut habe:

```
.opencode/
├── agents/
│   └── run-story.md          # Orchestriert den kompletten Story-Flow
├── skills/
│   ├── refine-story/         # Story fachlich verfeinern
│   ├── plan-implementation/  # Technischen Plan erstellen
│   ├── implement/            # TDD-Implementierung
│   ├── review-implementation/# Code-Review
│   ├── tdd/                  # TDD-Methodik
│   └── ...
└── commands/
    └── add-story.md          # Neue Story zum Backlog hinzufügen
```

Jede Datei ist ein Markdown-Dokument mit strukturierten Instruktionen. Kein Code, keine JSON-Dateien – einfach lesbare Anweisungen, die ein KI-Modell interpretieren kann.

## Der Workflow: Von der Idee zur Implementierung

Lass mich zeigen, wie das in der Praxis funktioniert. Ich starte mit einer einfachen User-Story:

```
/add-story Rezept anlegen mit Titel, Kategorien und Zutaten
```

Der `add-story` Command ist ein Markdown-File mit klaren Instruktionen:

> *"Extrahiere Titel und Zusatzinformationen aus der User-Eingabe. Vergib die nächste freie Story-Nummer. Erstelle ein Verzeichnis `docs/XX-story-name/` mit einer rudimentären `story.md`. Aktualisiere `docs/stories_epics.md`."*

Das Ergebnis: Eine neue Story mit Nummer und Verzeichnis. Aber noch ohne Inhalt, nur das, was ich dem Command mitgegeben habe.

### Phase 1: Story Refining

Als nächstes starte ich den `run-story` Agenten:

```
Nächste Story
```

Dieser Agent orchestriert den gesamten Flow. Zuerst sucht er die nächste Story, die umgesetzt werden kann. Dabei werden Abhängigkeiten berücksichtigt. 

Danach lädt er den Skill `refine-story`. Die Instruktion dort lautet:

> *"Lies `docs/stories_epics.md` und ermittle Nummer und Verzeichnisname. Lies die Vorlage `docs/templates/story.md`. Verfeinere die Story fachlich – nur fachliche Aspekte, keine technischen Details. Erstelle klare, testbare Akzeptanzkriterien. Die Definition of Ready muss erfüllt sein."*

Der Agent analysiert jetzt nicht nur die rudimentäre Story-Beschreibung, sondern auch die Produkt-Dokumentation. In meinem `prd.md` stehen detaillierte User Journeys beschrieben – zum Beispiel diese über die Wochenplanung:

> *"Es ist Mittwochabend, 19:30 Uhr. Anna und Fritz sitzen auf dem Sofa. Die wöchentliche Frage steht im Raum: 'Was kochen wir nächste Woche?' Früher bedeutete das: Kochbücher vom Regal holen, 20 Minuten später frustriert dieselben 5 Gerichte wählen wie immer. Heute öffnen beide die Rezepte-App."*

Diese User Journey sind in der allerersten Phase mithilfe des BMAD-Frameworks entstanden (TODO: Link einfügen). Für diese Phase war das extrem wertvoll, aber nachher habe ich BMAD aus dem Projekt entfernt, da ich die Agenten und Skills so schlank wie möglich halten wollte.

Basierend auf diesen Journeys und dem fachlichen Kontext erstellt der Agent eine vollständige `story.md` mit Akzeptanzkriterien wie:

- Benutzer können ein neues Rezept mit Titel (Pflicht) und Kategorie (Pflicht) erstellen
- Kategorien sind: Mittagessen, Brot, Party, Kuchen, Snacks
- Zutaten und Anleitung werden als Markdown gespeichert

### Phase 2: Technische Planung

Der `run-story` Agent lädt nun den Skill `plan-implementation`. Dieser liest die `story.md`, die `architecture.md` und erstellt einen technischen Plan.

Aus meiner `architecture.md`:

> *"Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX. Server-Side Rendering, keine JSON-APIs für UI. HTMX für interaktive Elemente. Form-Posts + Redirects (funktioniert ohne JS). SQLite mit JSON-Array für Kategorien."*

Der Plan, der daraus entsteht, ist eine detaillierte Checkliste in `plan.md`:

```markdown
## Implementierungsschritte

- [ ] Datenbank-Migration für Tabelle `recipes`
- [ ] Rust Model `Recipe` mit sqlx-FromRow
- [ ] Route `POST /recipes` für Create
- [ ] Askama Template `recipes/form.html`
- [ ] E2E-Test mit Playwright
```

### Phase 3: TDD-Implementierung

Der Skill `implement` lädt zunächst den Skill `tdd`. Die dort definierte Methodik folgt dem BDD Dual-Loop:

> *"Outer Loop: Write one integration/acceptance test that describes the next observable behavior. Inner Loop: Red-Green-Refactor cycles with unit tests. Never skip the red step."*

Der Agent implementiert nun Feature für Feature:

1. **Outer Loop (Rot)**: Ein Playwright-Test, der versucht, ein Rezept anzulegen – und natürlich fehlschlägt
2. **Inner Loop**: Unit Tests für die Datenbank-Operation, dann Implementierung
3. **Grün**: Der Integrationstest läuft durch
4. **Refactor**: Code bereinigen, während alle Tests grün bleiben

Dieser Zyklus wiederholt sich für jeden Schritt im Plan. Der Agent committed nach jeder Phase:

```bash
git commit -m "story XX: implementation"
```

### Phase 4: Review

Der letzte Schritt im Flow ist der Skill `review-implementation`. Der Agent:

1. Führt alle Qualitätschecks durch: `cargo build`, `cargo clippy`, `cargo test`, `npm run test:e2e`
2. Prüft die Akzeptanzkriterien aus der `story.md`
3. Erstellt ein `review.md` mit Findings (Prio 1 = blocker, Prio 2 = nice to have)

Wenn Prio-1-Probleme gefunden werden, startet der Skill `rework` und behebt diese. Dann läuft der Review erneut.

Am Ende steht eine fertige Story – mit allen Commits, Dokumentation und Tests.

## Instruction Files als Gedächtnis

Wie kann ich verhindern, dass die KI halluziniert? Welche Sicherheits-Leitplanken habe ich eingefügt? Die Skills und Agenten-Beschreibungen sind sehr knapp gehalten. Aber zusätzlich habe ich eine Reihe von Instruction Files angelegt (zum Großteil ebenfalls mit Hilfe von KI). Das sind Markdown-Dateien wie `AGENTS.md`, `architecture.md` und `prd.md`, die im Projekt-Root liegen und von allen Agents gelesen werden.

Mein `AGENTS.md` beginnt so:

> *"Projekt Rezepte. Dieses Repo enthält den Code für eine einfache Rezepte-Verwaltung. Sprache: Deutsch. Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX. Wichtige Constraints: LAN-only Web-App, KEINE Authentifizierung, Single-User, Last-write-wins bei Konflikten."*

Jeder Agent liest diese Datei zu Beginn. Das bedeutet:

- Ich muss die Constraints nicht bei jedem Prompt wiederholen
- Der Agent weiß immer, in welchem Kontext er arbeitet
- Das System ist konsistent über alle Phasen hinweg

Das ist der entscheidende Unterschied zu herkömmlichen KI-Chat-Interfaces: **Kontinuität durch Kontext**. Ein Skill definiert nicht nur, WAS zu tun ist, sondern WIE es zu tun ist – basierend auf den projektspezifischen Konventionen.

## Es muss nicht immer Claude sein. Kimi K2.5 tut es auch.

Das gesamte System läuft mit Kimi-K2.5, einem Modell von Moonshot AI. Kein Claude, kein GPT-4 – ein relativ unbekannter Player auf dem Markt.

Warum?

**Kosten:** Kimi-K2.5 kostet etwa 1/10 dessen, was Claude Sonnet verlangt. Bei einem komplexen Agent-Workflow, der mehrere tausend Tokens verbraucht, macht das einen gewaltigen Unterschied.

**Qualität:** Ja, es ist langsamer als Claude oder GPT-4. Die Antworten dauern länger. Aber die Qualität ist erstaunlich gut – besonders wenn man die Instruktionen strukturiert und viele Beispiele gibt. Ich habe keine Benchmarks durchgeführt, aber nach meiner Erfahrung liefert das Modell Code etwa in der Qualität von Claude Sonnet 3.5 ab. Nicht vergleichbar mit Opus 3.5 oder Sonnet 3.6, aber für sehr viele Anwendungen reicht es. Insbesondere, da Fehler ja durch die E2E-Tests und die Überprüfung durch den `review`-Skill auffallen und dann korrigiert werden.

Meine Erfahrung: Bei gut strukturierten Prompts und klaren Instruktionen liefert Kimi-K2.5 Ergebnisse, die mit den großen Modellen mithalten können. Der Geschwindigkeitsnachteil wird dadurch ausgeglichen, dass ich nicht ständig interagieren muss – meine Agenten arbeiten autonom zusammen.

Als kleines Schmankerl schicke ich mir am Ende noch eine iMessage per AppleScript zu, was auf einem Mac sehr leicht geht.

## Praktische Erkenntnisse

Nach einigen Tagen mit diesem Setup habe ich eine lauffähige Anwendung, die wir in unserer Familie einsetzen und dazu noch einige Erkenntnisse gewonnen:

**Agents brauchen klare Exit-Kriterien.** Ein Skill sollte immer definieren, wann er fertig ist. "Implementiere bis alle Tests grün sind" funktioniert besser als "Implementiere das Feature".

**Review ist Pflicht.** Selbst ein guter Agent macht Fehler. Das zweite Paar Augen (auch wenn es ebenfalls ein Agent ist) findet Dinge, die der Implementierungs-Agent übersehen hat.

**TDD funktioniert auch mit KI.** Die Disziplin, erst einen roten Test zu schreiben, führt zu besserem Code. Auch wenn der Agent ihn schreibt. Besonders der Outer/Inner Loop aus dem TDD-Skill hilft, den Fokus zu behalten.

**Preiswerte Modelle sind gut genug.** Der Hype um die teuersten Modelle ist nicht immer gerechtfertigt. Mit gutem Prompting und strukturierten Workflows kann man auch mit günstigeren Alternativen arbeiten.

## Grenzen und Einschränkungen

Lassen Sie mich nichts beschönigen: Dieses System hat Grenzen.

**Komplexe Architekturentscheidungen** erfordern nach wie vor menschliche Intelligenz. Der Agent kann einen Plan ausführen, aber er versteht nicht immer, warum eine bestimmte Architektur gewählt wurde.

**Fachliche Unklarheiten** blockieren den Agenten. Wenn die Anforderungen widersprüchlich sind oder wichtige Informationen fehlen, kommt der Skill `refine-story` nicht weiter. Dann muss ich als Product Owner eingreifen.

**Kreative Lösungen** sind nicht die Stärke von Agents. Sie folgen Mustern, die sie in den Instruktionen und im bestehenden Code finden. Wirklich innovative Ansätze kommen (noch) vom Menschen. Das führt vor allem dazu, dass die UI meiner Anwendung -- sagen wir mal -- bestenfalls mittelmäßig ist.

**Setup-Aufwand** ist nicht zu unterschätzen. Das System, das ich beschrieben habe, hat einiges an Feinjustierung gekostet. Die Skills mussten iterativ verbessert werden, die Instruction Files erweitert.

TODO: Zwei Exkurse: 
- Ralph-Loop mit `/add-story` parallel
- Zwischendurch reine QS-Läufe, z. B. Qualität der E2e-Tests

## Fazit: Ein Blick in die Zukunft

Was ich gebaut habe, ist kein Produktions-System für Enterprise-Projekte. Es ist ein Experiment, ein Proof of Concept. Aber es zeigt, wohin die Reise geht.

Statt KI als erweiterte Autocomplete-Funktion zu nutzen, kann sie als Orchestrierungs-Engine eingesetzt werden. Die Agents führen einen definierten Prozess aus, der menschliche Software-Entwicklungs-Praktiken abbildet: Von der Story über den Plan zur Implementierung mit TDD und Review. Wenn man den Prozess im Auge behält und die Leitplanken justiert, kann man einen Development-Prozess errichten, die ohne Code-Erstellung oder -Prüfung auskommt.

Die wichtigste Erkenntnis: **Struktur schlägt Modell-Größe.** Ein gut durchdachter Workflow mit klaren Instruktionen und einem günstigen Modell (Kimi-K2.5) kann bessere Ergebnisse liefern als ein teures Modell ohne Prozess.

Für mich persönlich hat sich der Aufwand gelohnt. Die Rezepte-App ist mittlerweile produktiv im Einsatz – und wurde vollständig von Agents entwickelt. Ich habe die Stories geschrieben, die Architektur definiert und die Reviews geprüft. Den Rest hat das System übernommen. Und ich habe großes Vertrauen in die Qualität der Umsetzung.

Ob das die Zukunft der Softwareentwicklung ist? Ich bin optimistisch. Die Technik ist da, die Prozesse funktionieren. Es braucht noch etwas Experimentieren, bis solche Systeme in Teams skalieren, aber eines ist aber sicher: Die Art, wie wir Software entwickeln, wird sich grundlegend verändern. Vielleicht nicht in jedem Fall durch vollautomatisierte Code-Generierung, in jedem Fall aber durch die Kombination aus menschlicher Architektur und maschineller Ausführung.

