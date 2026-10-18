# Prüfbericht 2: Gegenlesen von Spezifikation v0.3 nach der Neuausrichtung

Stand 18.10.2026 · Prüfer: unabhängiger Prüf-Agent ohne Beteiligung an den Texten, nur lesend, Rechnungen mit Python nachgeprüft · Status: **eindeutige Fehler korrigiert (Abschnitt 5), Entscheidungen offen (Abschnitt 6)**

## 1. Was stimmt

- `docs/anhang/m-simulation.py` läuft und ergibt genau die Tabelle in Spezifikation 9.1.
- Erste Epoche 50.500 TOY, erstes Jahr 16,837 Mio., Halbwertszeit 1.372,2 Epochen, nach 25 Jahren 99,00 Mio.
- Größte Zwischenzahl 1,01 · 10^35, rund 3.000-fach Luft in `u128`. Die Emission einer Epoche kann die Restmenge nie übersteigen.
- Lohn der Start-Validatoren bei m = 1: 2,21 bis 2,22 Mio. TOY, je nach Lesart.

## 2. Kritische Befunde

| Nr. | Befund | Vorschlag |
|---|---|---|
| **K1** | **Übergabe aus der Startphase kaperbar.** Bei weniger als 100 Validatoren greift der 1-%-Deckel bei allen, jeder Validator zählt gleich. Wer mit viel Rechenleistung schnell 21 × 1.000 TOY aus der Lotterie holt, stellt 75 % der Stimmen. „Neu“ heißt nur „andere Adresse“ und ist nicht fälschungssicher. Ohne Deckel behalten dagegen die Start-Validatoren die Kontrolle | Übergabe erst, wenn neue Validatoren 2/3 des wirksamen Stakes halten, Stake erst nach Mindest-Bindezeit zählen, Mindestdauer der Startphase, Deckel eindeutig definieren |
| **K2** | **Lohn der Start-Validatoren lässt sich um ca. 65 % steigern.** Der Lohn sinkt je Epoche, der Plan rechnet nach Zeit. Langsame Blöcke (z. B. 3-Tage-Epochen) treiben m auf 2,0. Ergebnis: 3,66 statt 2,22 Mio. TOY | Lohn nach Tagen seit Genesis abschmelzen, Höchstabstand zwischen Blöcken, Genesis-Zeitstempel prüfen |
| **K3** | **Zusammenführen hebelt Schutz aus.** Ein Dieb schickt TOY ohne Beweis an ein ruhendes, geschütztes Konto, setzt im neuen Konto eigene Einstellungen und weckt dann das alte auf | Geschützte Konten nie auslagern, beim Zusammenführen gilt die strengere und ältere Einstellung |

## 3. Hohe Befunde

| Nr. | Befund |
|---|---|
| H1 | Doppelsignatur falsch definiert: In BFT wird nach Zeitüberschreitung auf derselben Höhe neu abgestimmt. Ehrliche Validatoren würden bestraft oder die Kette bliebe stehen |
| H2 | Startphase kann sich lahmlegen: Bei 7 Start-Validatoren reichen 3 Sperren für Stillstand. Die Doppelgänger-Erkennung nach jedem Neustart legt die Kette nach einem Update 2 Tage still |
| H3 | Der Blockbauer steuert die Arbeits-Lotterie: Er wählt die Lose aus und kennt seinen Zufall vorher. Lose lassen sich im Voraus rechnen, Signatur der Lose nicht festgelegt |
| H4 | Aufnahme-Pflicht so nicht baubar: Der Zusammenfassende kann Stimmen weglassen, die Listen belegen bis zur Hälfte des Blocks. „HotStuff“ und „endgültig nach 1 Block“ passen nicht zusammen |
| H5 | Mehrere Sperrregeln der Ebene A sind nicht eindeutig nachweisbar (Zeitstempel, ungültige Lose, Konten-Anlegen, Inaktivität) und treffen Ehrliche (Kran, Börsen, Sammelüberweisungen) |
| H6 | Kostenloses Kontingent: 171.500 Konten (17.150 TOY Pfand, rückholbar) füllen alle Blöcke. Erstattete Gebühren würden weiterverteilt (P9 offen) |
| H7 | Speicher-Füllen (P5) nicht gelöst: Konten mit Aktivität in 7 Epochen werden nie ausgelagert, Merkliste wächst mit jeder Auslagerung und steht nicht im Budget, 60 bis 100 Byte je Konto zu optimistisch |
| H8 | Belohnung nach Wurzel des Stakes lohnt das Aufteilen (100 × 10 TOY bringen das 10-Fache von 1 × 1.000 TOY) |
| H9 | Tresor-Lücken: Aufteilen unter die Grenze, andere Abflusswege (Auflösen, Spende, Zeitschloss, Treuhand, Abo, Sitzungs-Schlüssel, Delegation), sofortige Änderung ohne Tresor, Erbe ohne Abbruchfrist |
| H10 | Kompaktes Format mit Ablaufzeit widerspricht „MetaMask kann TOY senden“ |

## 4. Mittlere und niedrige Befunde

- M1: Zeitpunkt des Mintens unklar, Mindestgewinn 0,2 TOY passt später nicht mehr ins Budget.
- M2: Regel I0 nennt nicht alle Töpfe (Zeitschlösser, Treuhand, wartende Abhebungen, Stake im Lösen, festgehaltener Stake, Delegation).
- M3: Zeitregel beim atomaren Tausch nur zur Hälfte richtig.
- M4: Auflösen löscht den Sperr-Verlauf. Offen, ob man mit offenen Verpflichtungen auflösen darf.
- M5: Bei 7 Start-Validatoren kostet ein Ausrutscher 43 %. Delegierte entgehen der Kürzung nach 7 Epochen.
- M6: Beweise ruhender Konten hängen an Archiv-Knoten. Block-Fenster bei vollen 10-s-Blöcken nur ca. 64 Minuten.
- M7: Verfassung „kein Einfrieren“ widerspricht Pflicht-Pfand und Festhalten des Stakes.
- M8: Datenlecks (öffentliche RPC-Zugänge, Kursdienst, Adress-Alarm, Sync-Metadaten).
- M9: Betrüger leiten Opfer an, auf Vollprofi zu schalten. Täuschungs-PIN verspricht zu viel. Sicherheitsadresse darf nicht aus derselben Phrase stammen.
- M10: Arbeits-Lotterie bevorzugt Grafikkarten und Botnetze.
- M11: Leicht-Knoten können A1 nicht selbst prüfen.
- Niedrig: elf Widersprüche und veraltete Stellen (u. a. Delegieren in M4 statt M12, Hybrid-Signaturen, „Phase 4“, cargo-deny ab M1, „Konzept vollständig“, alte Verweise in 02, Kontingent je Konto, Komitee 128 bei 7 bis 28 Validatoren, Ablauf „nach dem Absenden“).
- Rechtlich: Begriff „Lotterie“, Einmal-Adressen und Tor (EU-Geldwäscheverordnung), Lohn der Start-Validatoren, Namen (Datenschutz, Markenrecht), Erben-Adresse ersetzt kein Erbrecht, Treuhand und Kassen-Modus können erlaubnispflichtig sein, Verwechslung mit älteren ToyCoin-Token, Testnet-Guthaben werden nie übernommen.
- CLAUDE.md: Für VRF, ML-DSA, Konsens und Dandelion++ gibt es wenig geprüfte Rust-Bibliotheken. Geräte-Prüfung und Push-Dienste der großen Anbieter sind proprietär. Drei unabhängige Signaturen sind ohne Team nicht erfüllbar.

**Aus Prüfbericht 1 noch offen:** P5 (Speicher), P6 (Header), P7 teilweise, P9 (Erstattungs-Pool), Höchstwert der progressiven Gebühr, Rundungsrest 50/40/10, Zählbeginn der Epochen.

**Urteil des Prüfers:** Für M1 nah dran, es fehlen 6 Festlegungen (Minten, Lohnformel, Rundungsreste, Epochen-Zählung, Datentypen der Eingaben, Umfang der formalen Prüfung). M1 bis M13 als Ganzes ist eine Aufgabe für ein Team über mehrere Jahre. Empfehlung: schlankes Testnet zuerst.

## 5. Direkt korrigiert (eindeutige Fehler)

Siehe Entscheidungen 106 bis 108 und die Änderungen in Spezifikation, Wallet, ToyScan und Bauplan vom 18.10.2026.

## 6. Entscheidungen

Werden mit dem Projektinhaber abgestimmt und als Entscheidungen ab Nr. 109 eingetragen.
