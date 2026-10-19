# Whitepaper

Vorläufiges Whitepaper (Entwurf 0.3, 19.10.2026, **vor der Testphase**): `ToyChain-Whitepaper-v0.3.pdf`, 19 Seiten, Deutsch, mit 16 Abbildungen, rund 40 Web-Links (Stand Oktober 2026) und einer Einschätzung der Marktchancen (Abschnitt 16).

Entwurf 0.3 prüft Entwurf 0.2 auf Logik, Nachvollziehbarkeit und neue Erkenntnisse; alle Änderungen stehen in Anhang D.

Übersetzungen, alle auf Stand Entwurf 0.3 (die deutsche Fassung ist maßgeblich):

- Englisch: `ToyChain-Whitepaper-v0.3-en.pdf` (19 Seiten), `build_en.py`
- Französisch: `ToyChain-Whitepaper-v0.3-fr.pdf` (20 Seiten), `build_fr.py`
- Yoruba: `ToyChain-Whitepaper-v0.3-yo.pdf` (16 Seiten), `build_yo.py`. Fachbegriffe ohne klare Yoruba-Entsprechung bleiben englisch, alle Links führen auf englische Seiten. Maschinell übersetzt, vor einer Veröffentlichung von Muttersprachlern gegenlesen lassen.
- Abbildungen: `python3 figures_i18n.py en|fr|yo` schreibt nach `abbildungen/<sprache>/`. Gemeinsames Layout: `wp_common.py`.

Neu erzeugen (Python mit reportlab, matplotlib und Pillow, Schrift DejaVu):

```
cd docs/whitepaper
python3 figures.py   # Abbildungen nach abbildungen/
python3 build.py     # PDF
```

Marktzahlen geben den Stand vom 18.10.2026 wieder und müssen bei einer neuen Fassung aktualisiert werden. Maßgeblich bleibt die Spezifikation `docs/01-spezifikation.md`.
