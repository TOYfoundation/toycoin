# Whitepaper

Vorläufiges Whitepaper (Entwurf 0.2, 18.10.2026, **vor der Testphase**): `ToyChain-Whitepaper-v0.2.pdf`, 18 Seiten, Deutsch, mit 16 Abbildungen, rund 40 geprüften Web-Links und einer Einschätzung der Marktchancen (Abschnitt 16).

Übersetzungen (gleicher Inhalt, Stand 18.10.2026, die deutsche Fassung ist maßgeblich):

- Englisch: `ToyChain-Whitepaper-v0.2-en.pdf` (17 Seiten), erzeugt mit `build_en.py`
- Französisch: `ToyChain-Whitepaper-v0.2-fr.pdf` (19 Seiten), erzeugt mit `build_fr.py`
- Die Abbildungen dazu liegen in `abbildungen/en/` und `abbildungen/fr/` und entstehen mit `python3 figures_i18n.py en` bzw. `fr` (übersetzt die Beschriftungen von `figures.py`). Gemeinsames Layout: `wp_common.py`.

Neu erzeugen (Python mit reportlab, matplotlib und Pillow, Schrift DejaVu):

```
cd docs/whitepaper
python3 figures.py   # Abbildungen nach abbildungen/
python3 build.py     # PDF
```

Marktzahlen geben den Stand vom 18.10.2026 wieder und müssen bei einer neuen Fassung aktualisiert werden. Maßgeblich bleibt die Spezifikation `docs/01-spezifikation.md`.
