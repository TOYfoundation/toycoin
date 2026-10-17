# TOYQUEST 3D (Godot 4.4)

Eine offene 3D-Welt aus 1001 Nacht als eigenes Spiel neben der TOYPAY-App. Es gibt sechs Karten zu je rund 400.000 m². Man kann klettern, springen, schwimmen und tauchen, reiten und Boot fahren. Dazu kommen eine Stadtwache mit Fahndung, Missionen, Kampf mit Säbel und Bogen sowie ein Online-Modus über den TOYPAY-Server.

## Starten
1. Godot 4.4 installieren (godotengine.org, Standard-Version).
2. Godot öffnen → **Importieren** → `godot3d/project.godot`. Der erste Import dauert 1–3 Minuten, weil die Texturen komprimiert werden.
3. Mit **F5** spielen.

Steuerung am PC:
- WASD laufen, Maus Kamera, Leertaste springen/klettern.
- Linke Maustaste Angriff, rechte Maustaste Bogen.
- C ausweichen, E Aktion, R Reittier, Q tauchen, H Trank, M Karte, Esc Menü.

Am Handy erscheinen Touch-Tasten. Ein Gamepad wird erkannt.

## Grafik
- **Renderer:** Am Desktop Forward+, auf Android Mobile. Fehlt Vulkan, schaltet Godot automatisch auf OpenGL (Kompatibilität) um. Erzwingen lässt sich das mit `--rendering-method gl_compatibility`.
- **Qualität:** Menü → Grafikqualität (Niedrig/Mittel/Hoch). Gras, Baum-Detailstufen und Schatten wirken voll ab der nächsten Karte, Bildeffekte sofort.
  - Niedrig: für Handys.
  - Mittel: Standard am PC. Im Renderer Forward+ zusätzlich Umgebungsverdeckung (SSAO) und Volumennebel.
  - Hoch: zusätzlich SSIL, Bildschirmspiegelungen, globale Beleuchtung (SDFGI) und TAA.
- **Materialien:** `shaders/props.gdshader` legt 15 Baustoffe dreiachsig auf alle Objekte. Die Materialnummer kommt aus dem Alpha der Vertexfarbe (`MeshKit.use(MeshKit.WOOD)` usw.).
- **Gelände:** `shaders/terrain.gdshader` mischt 12 Bodenlagen nach `worlds/m<i>_splat0..2.png`, mit Höhenmischung, Felshängen, Nässe, Pfützen und Kaustik.
- **Vegetation:**
  - `scripts/flora.gd` baut Bäume und Palmen in zwei Detailstufen.
  - `scripts/grass.gd` erzeugt Gras, Blumen, Weizen, Schilf und Kiesel in 10-m-Kacheln um den Helden.

## Antike Erfindungen
- 11 Lehrstätten mit Fachleuten (violette Punkte auf der Minikarte) zeigen 50 antike Maschinen als bewegte Modelle.
- An den Schautafeln (✋) gibt es Erklärung, Herkunft, Vorführung, Wissensfrage und Herausforderung.
- Bürger geben Hinweise auf die Lehrstätten.
- Menü → „Erfindungen“ zeigt den Fortschritt.
- Code: `scripts/machines.gd` (Logik), `scripts/machine_models.gd` (Modelle).
- Test: `godot --headless --path godot3d tests/machines_test.tscn`.

## Daten neu erzeugen
```
python3 tools/gen_world3d.py        # Höhen, Wasser, Farb- und Materialkarten aus server/world/m<i>.json
python3 tools/gen_textures3d.py     # Materialbibliothek (benötigt numpy, scipy, Pillow), ca. 2 Minuten
python3 tools/i18n/gen.py           # Texte (28 Sprachen), schreibt auch godot3d/i18n/
```

## Tests
```
godot --headless --path godot3d tests/move_test.tscn -- map=3     # Klettern, Schwimmen, Tauchen, Fallen
godot --headless --path godot3d tests/smoke_test.tscn             # Leute, Wache, Reiten, Boot, Missionen, Kampf
godot --path godot3d tests/play_shot.tscn -- map=3 spot=harbor out=/tmp   # Bildschirmfoto (spawn harbor town2 herd ride climb swim at)
godot --headless --path godot3d tests/net_test.tscn -- ticket=XXXX server=http://127.0.0.1:18080
```

## Exportieren
In Godot unter **Projekt → Exportieren** sind diese Vorlagen angelegt:
- Android (`app.toypay.quest3d`, arm64).
- Windows.
- Linux.

Die Export-Vorlagen von Godot müssen einmal installiert werden.
