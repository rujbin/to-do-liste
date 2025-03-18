# Todo GUI App

Eine grafische Aufgabenverwaltungsanwendung, erstellt mit Rust und Iced.

## Funktionen

- Aufgaben hinzufügen, bearbeiten und löschen
- Aufgaben als erledigt markieren
- Filterung von Aufgaben (Alle, Aktiv, Erledigt)
- Suchfunktion
- Datenpersistenz (Speichern in lokaler JSON-Datei)
- Moderne und responsive Benutzeroberfläche

## Voraussetzungen

- Rust und Cargo (neueste stabile Version empfohlen)
- Visual C++ Build Tools (für Windows) - [Download](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

## Ausführen der Anwendung

1. Klone dieses Repository
2. Navigiere zum Projektverzeichnis
3. Führe die Anwendung aus:

```bash
cargo run
```

## Projektstruktur

- `src/main.rs` - Einstiegspunkt und Anwendungssetup
- `src/lib.rs` - Hauptanwendungsstruktur und Nachrichtenverarbeitung
- `src/todo.rs` - Todo-Datenstrukturen und Zustandsverwaltung
- `src/ui/` - Benutzeroberflächen-Komponenten
  - `src/ui/view.rs` - UI-Layout und Komponenten
  - `src/ui/style.rs` - Benutzerdefinierte Stile für UI-Elemente

## Abhängigkeiten

- [Iced](https://github.com/iced-rs/iced) - GUI-Framework
- Serde - Serialisierung/Deserialisierung 
- Chrono - Datums-/Zeitbehandlung
- Dirs - Verwaltung von Dateipfaden

## Lizenz

MIT 