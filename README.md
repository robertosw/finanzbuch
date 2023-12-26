# Finanzbuch
Ein Tool zur statistischen Analyse und allgemeinen Verwaltung deiner Finanzen und Investitionen.

Wenn du bei der Entwicklung dieses Programms helfen möchtest, lies dir bitte [CONTRIBUTING.md](/CONTRIBUTING.md) durch.

## Projektstruktur
Die [Bibliothek](/finanzbuch_lib) und der [UI-Code](/tauri) sind einzelne Cargo-Projekte.
Im [docker](/docker) Ordner liegen alle notwendigen Dateien um dieses Projekt in einem Container zu starten und zu entwickeln.

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
![Bildschirmfoto vom 2023-12-21 12-28-50](https://github.com/robertosw/finanzbuch/assets/47303535/5344f357-347f-49f6-a6da-dd83566624f0)
Die angezeigte Genauigkeit der Werte in einer Spalte und einem Jahr richtet sich nach dem genauesten Wert in dieser Spalte und diesem Jahr.

<br>

#### Depot-Übersicht mit Diagrammen
![Bildschirmfoto vom 2023-12-21 12-28-59](https://github.com/robertosw/finanzbuch/assets/47303535/95df72f1-7925-4f9c-a575-623a443d0107)
Vergleich mit idealem Wachstum wird später hinzugefügt
