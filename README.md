# Mob Control – Rust Edition

Eine performante 3D/2.5D-Nachbildung des bekannten Mobile-Hits **Mob Control** in **Rust** mit der **Bevy Engine (0.15)**.

---

## 🎮 Features

- **Hyper-Casual Swarm Action**: Hunderte bis Tausende von Mobs strömen als organische Welle über die Runway.
- **Custom Spatial-Hashing Physik**: Hochperformante Kollisionsabfrage & Schwarm-Separation (Boids-Verhalten) für flüssige 60+ FPS ohne Rigid-Body-Instabilitäten.
- **Multiplikator-Tore**: Dynamische und statische Tore (`x2`, `x3`, `x4`, `+5`, `+8`, `+15`), die sich im Takt hin- und herbewegen.
- **Kanone & Barrels**:
  - Zielsteuerung per Maus-Drag oder Tastatur (`A`/`D` bzw. Pfeiltasten).
  - Automatisches Dauerfeuer.
  - Upgradebar auf Doppel- und Dreifach-Lauf.
  - Rückstoß- und visuelles Feedback.
- **Riesen-Champions**:
  - Jeder abgeschossene Mob füllt die Champion-Leiste.
  - Entfessle gigantische Champions (`Leertaste` oder `C`), die feindliche Horden niederwalzen.
  - Spätere Level fordern dich mit feindlichen Riesen heraus!
- **Level-Kampagne & Endlos-Modus**:
  - Level 1: Grundlagen & Einführung in Multiplikatoren.
  - Level 2: Schwingende Tore & additiver Boost.
  - Level 3: Geteilte Pfade mit synchronen Sinus-Toren.
  - Level 4: Schwere feindliche Wellen & Champion-Duelle.
  - Level 5+: Skalierender prozeduraler Endlos-Modus mit eskalierenden Basen und Tor-Kombos.
- **Meta-Progression & Armory Shop**:
  - Verdiene Münzen durch das Ausschalten feindlicher Mobs und das Zerstören von Basen.
  - Schalte Upgrades frei: Feuerrate, Mob-Geschwindigkeit, Mehrfachläufe und Champion-HP.
- **Zero-Dependency Prozedurales Audio**:
  - Synthetisierte Soundeffekte direkt im Arbeitsspeicher (Schuss, Tor-Chime, Mob-Pop, Champion-Stomp, Sieg & Niederlage).
  - Keine externen Audio-Dateien notwendig!
- **Juice & Polish**:
  - Dynamischer Screenshake bei Einschlägen und Explosionen.
  - 3D-Weltraum-Text und leuchtende PBR-Materialien.

---

## 🕹️ Steuerung

| Aktion | Eingabe |
| :--- | :--- |
| **Kanone zielen / bewegen** | Maus-Drag (linke Maustaste halten & ziehen) oder `A` / `D` / Pfeiltasten |
| **Feuern** | Automatisches Dauerfeuer |
| **Champion / Riese rufen** | `Leertaste` oder `C` (sobald 100% aufgeladen) |
| **Menü-Navigation** | Mausklick auf die Buttons |

---

## 🚀 Spiel starten

```bash
cargo run
```

Für maximale Performance:

```bash
cargo run --release
```
