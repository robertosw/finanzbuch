# Finanzbuch
Ein Tool zur statistischen Analyse und allgemeinen Verwaltung deiner Finanzen und Investitionen.

Wenn du bei der Entwicklung dieses Programms helfen möchtest, lies dir bitte [CONTRIBUTING.md](/CONTRIBUTING.md) durch.

## Projektstruktur
- Die [Bibliothek](/finanzbuch_lib) und der [UI-Code](/tauri) sind einzelne Cargo-Projekte.
- Im [docker](/docker) Ordner liegen alle notwendigen Dateien um dieses Projekt in einem Container zu starten und zu entwickeln.
- Warum sind diese Übersicht und das Program in Deutsch und nicht in Englisch?
  - Bisher nutze ich das Program alleine, warum sollte ich es dann auf Englisch entwickeln
  - Ich kenne meist nicht die korrekten Fachbegriffe in Englisch und möchte vermeiden, dass es dadurch unprofessionell wirkt
  - Ich hatte zu beginn alles auf Englisch, deshalb gibt es den Branch `main-en`. Außerdem ist auch auf dem `main-de` Branch der gesamte Code auf Englisch

## Feedback
Für Verbesserungsvorschläge und Ideen bitte eine [Diskussion](https://github.com/robertosw/finanzbuch/discussions/categories/ideas-feedback) in der Kategorie Ideen & Feedback starten.

## Features / Roadmap
Alle Daten werden lokal gespeichert.
- [ ] Verschlüsselung

### Persönliche Finanzen / Buchhaltung

- Monatliche Einnahmen, Ausgaben und Sparziele im Auge behalten.
- Regelmäßige Einnahmen und Ausgaben speichern und einsehen, um vorausschauend planen zu können.

<br>

- [ ] Einnahmen und Ausgaben pro Monat speichern
  - [ ] Monatliche Notiz möglich
- [ ] Ziel setzen, wieviel % der Einnahmen maximal ausgegeben werden sollen
- [ ] Jahresübersicht mit monatlich berechneten Daten: Einnahmen, Ausgaben, Differenz, % der Einnahmen ausgegeben und ob Ziel erreicht wurde
  - [ ] Eine Tabelle pro Jahr mit:
    - Summe Einnahmen/Ausgaben, deren Differenz, Prozentsatz ausgegeben und ob Ziel erreicht
    - Median für entsprechende Felder
  - [ ] Wiederkehrende Einnahmen und Ausgaben
- [ ] Diagramme zur Übersicht
- [ ] Speichern und Bearbeiten von wiederkehrenden Einnahmen und Ausgaben
- [ ] Import einer CSV-Datei mit Transaktionsdaten für einen Monat

<br>

### Investing
- Tabellarische Anzeige der monatlichen Entwicklung des Porfolios und der einzelnen Positionen
- Statistiken und Diagramme zur Zusammensetzung und Entwicklung des Depots

<br>

- [ ] Depoteinträge
  - [x] Erstellen
  - [x] Löschen
  - [x] Daten verändern
  - [x] Vergangene Jahre hinzufügen
  - [x] Automatisch aktuelles Jahr hinzufügen
  - [ ] Daten aus CSV Datei importieren
- [ ] Sparpläne erstellen und ändern (Start- und Enddatum, Interval und Sparrate)
  - [ ] Bearbeiten für jeweils ein Depoteintrag:
    - [ ] Erstellen
    - [ ] Löschen
  - [ ] Bearbeiten für mehrere Depoteinträge gleichzeitig
    - [ ] Erstellen
    - [ ] Löschen
- [ ] Übersicht (Diagramme)
  - [ ] Inflationsbereinigte Anzeige
  - [ ] TER (Laufkosten) einberechnen
  - [ ] Vergleich mit idealem Wachstum

### Dezeitiger Stand
Alles was in der Navigationsleiste ausgegraut ist, wurde noch nicht umgesetzt.

#### Tabelle zur Anzeige der Daten eines Depoteintrags
![Bildschirmfoto vom 2024-01-02 22-52-57](https://github.com/robertosw/finanzbuch/assets/47303535/5bfcc4c9-41de-418b-a6b4-50959e1e88f5)
Die angezeigte Genauigkeit der Werte in einer Spalte und einem Jahr richtet sich nach dem genauesten Wert in dieser Spalte und diesem Jahr.

<br>

#### Depot-Übersicht mit Diagrammen
![Bildschirmfoto vom 2024-01-02 22-52-46](https://github.com/robertosw/finanzbuch/assets/47303535/6455e669-7282-45ec-b840-5852821dc733)
