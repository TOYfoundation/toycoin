# Whitepaper

Vorläufiges Whitepaper (Entwurf 0.4, 20.10.2026, **vor der Testphase**): `ToyChain-Whitepaper-v0.4.pdf`, 21 Seiten, Deutsch, mit 16 Abbildungen, 42 Web-Links (Stand Oktober 2026) und einer Einschätzung der Marktchancen (Abschnitt 16). Entwurf 0.4 bringt den Stand nach M2 und M3 und die Entscheidungen 115 bis 121; alle Änderungen stehen in Anhang D.

**Kurzanleitung für Nutzer** (neu, Entwurf 0.4): `ToyChain-Kurzanleitung-v0.4.pdf`, 5 Seiten, Deutsch. Sie zeigt, wie man ToyChain einrichtet, sobald Wallet und Testnetz fertig sind: App holen, Wörter sichern, Stufe wählen, erste TOY, senden, Konto schützen, MetaMask, goldene Regeln, häufige Fragen. Erzeugen: `python3 build_kurzanleitung.py`.

Ältere Fassung: `ToyChain-Whitepaper-v0.3.pdf` (19.10.2026). Übersetzungen, alle noch auf Stand Entwurf 0.3 (die deutsche Fassung ist maßgeblich):

- Englisch: `ToyChain-Whitepaper-v0.3-en.pdf` (19 Seiten), `build_en.py`
- Französisch: `ToyChain-Whitepaper-v0.3-fr.pdf` (20 Seiten), `build_fr.py`
- Yoruba (aus dem Englischen übersetzt): `ToyChain-Whitepaper-v0.3-yo.pdf` (16 Seiten), `build_yo.py`. Fachbegriffe ohne klare Yoruba-Entsprechung bleiben englisch, alle Links führen auf englische Seiten. Maschinell übersetzt, vor einer Veröffentlichung von Muttersprachlern gegenlesen lassen.
- Yoruba (direkt aus dem Deutschen übersetzt, Absatz für Absatz, mit Anhang D): `ToyChain-Whitepaper-v0.3-yo-de.pdf` (18 Seiten), `build_yo_de.py`. Fachbegriffe und Links englisch. Maschinell übersetzt, vor einer Veröffentlichung von Muttersprachlern gegenlesen lassen.
- Abbildungen: `python3 figures_i18n.py en|fr|yo` schreibt nach `abbildungen/<sprache>/`. Gemeinsames Layout: `wp_common.py`.

Neu erzeugen (Python mit reportlab, matplotlib und Pillow, Schrift DejaVu):

```
cd docs/whitepaper
python3 figures.py   # Abbildungen nach abbildungen/
python3 build.py     # Whitepaper-PDF
python3 build_kurzanleitung.py   # Kurzanleitung
```

Marktzahlen geben den Stand vom 19.10.2026 wieder und müssen bei einer neuen Fassung aktualisiert werden. Maßgeblich bleibt die Spezifikation `docs/01-spezifikation.md`.
