# ToySafe: Schutz- und Prüf-App im ToyVerse

Stand 24.10.2026 · **Konzept für später**, ändert nichts an Phase 1 · Entscheidung 134 (Name und Rolle entschieden, alles Weitere Vorschlag)

**ToySafe** ist eine eigenständige App, die Nutzer, das ToyVerse und auch andere schützt. Sie prüft Geräte, Wallets, Adressen, Links, Knoten und Dienste, warnt verständlich und hilft, Schutz einzurichten. Dazu kommt ein Prüflabor für Sicherheitstests (Pentests) gegen eigene oder ausdrücklich freigegebene Ziele. ToySafe funktioniert allein. Zusammen mit TOYPAY, ToyScan, ToyPost und TOYQUEST entsteht mehr Nutzen, ohne dass eine App von der anderen abhängt.

## 1. Grundsätze (Vorschlag)

| Nr. | Grundsatz | Begründung |
|---|---|---|
| G1 | **Nie Schlüssel, nie Phrase, nie signieren.** Aktionen auf der Kette bereitet ToySafe nur vor. TOYPAY zeigt die Vorschau und signiert | wie ToyScan (Nr. 56), Wallet-Schutz (Nr. 82) |
| G2 | **Nur eigene oder freigegebene Ziele.** Aktive Prüfungen (Port-Scan, Lasttest, Angriffs-Übung) laufen nur gegen das eigene Gerät, den eigenen Knoten, das lokale Devnet (Chain-ID 7630715) oder Ziele mit einer **signierten Prüf-Freigabe** des Eigentümers (Abschnitt 6.4). Ohne Freigabe liest ToySafe nur öffentliche Daten | Schutz Dritter, rechtliche Sicherheit |
| G3 | **Warnen, nie selbst sperren.** Eingriffe in ein Konto gibt es nur über Einstellungen, die der Eigentümer selbst gesetzt hat (Konto-Schutz, Spezifikation 7.3 und 7.4). Mustererkennung bleibt eine Warnung | CLAUDE.md, Nr. 83 |
| G4 | **Lokal zuerst, offline möglich.** Prüfungen laufen auf dem Gerät. Daten verlassen es nur mit Zustimmung und ohne Inhalte (keine Beträge, keine Nachrichten). Keine proprietären Push- und Prüfdienste | Nr. 108 |
| G5 | **Offene Daten.** Warnlisten sind öffentlich, versioniert und signiert. Jeder kann sie prüfen und nutzen, auch andere Projekte | Nr. 89, 100 |
| G6 | **Verantwortungsvolle Offenlegung.** Funde gehen vertraulich an das betroffene Projekt. Veröffentlichung erst nach Behebung oder Frist. Belohnung über das Bug-Bounty-Programm aus der Projekt-Wallet | Nr. 88, 130 |
| G7 | **Gleiche Warnungen überall.** 45 Sprachen, vier Modi, Sicherheitswarnungen in jedem Modus gleich | Nr. 103, 104 |
| G8 | **Nur geprüfte Verfahren.** Geprüfte Bibliotheken, Testvektoren aus Standards, MIT-Lizenz, reproduzierbare Builds | Spezifikation 13, Nr. 87 |

## 2. Bausteine

| Baustein | Was er tut | Verbindung |
|---|---|---|
| **A Sicherheits-Check** | Prüft Bildschirmsperre, Updates, Root/Jailbreak (nur Hinweis), Zwischenablage-Schutz, Bildschirmfoto-Sperre, Sicherungs-Quiz, Hardware-Wallet, Phrase in Teilen (SLIP-39). Ergebnis als **Schutzstufe** mit Ampel und nächstem Schritt | TOYPAY (Nr. 89, 97, 99) |
| **B Konto-Schutz-Assistent** | Erklärt, simuliert und bereitet vor: Tresor-Verzögerung, Notfall-Schlüssel mit Sicherheitsadresse (nicht aus derselben Phrase), Selbst-Sperre, Erben-Adresse. **Notfall-Knopf**, der die Selbst-Sperre vorbereitet | ToyChain (Nr. 83, 106, M12), TOYPAY signiert |
| **C Adress- und Link-Prüfung** | Adress-Fallen (ähnlich aussehende Adressen), Warnliste, alte ToyCoin-Token, Zahlungslinks `toychain:`, lesbare Signier-Anfragen (EIP-712), Token-Freigaben auf anderen Ketten, falsche Prägeseiten | TOYPAY (Nr. 82, 86, 94), ToyPost |
| **D Wächter** | Beobachtet eigene Adressen nur lesend: Adress-Alarm bei jeder Bewegung, Warnampel A1 bis A9, ToyVerse-Ampel B1 bis B4. Meldung lokal oder über ToyPost | ToyScan (Nr. 100), ToyPost |
| **E Knoten-Prüfung** | Für den eigenen Knoten: offene Ports, RPC nur dort erreichbar, wo gewollt, Version und Prüfsumme des reproduzierbaren Builds, Uhrzeit-Abweichung (höchstens 15 s), Speicher-Budget 256 MB, Validator-Schlüssel getrennt, Doppelgänger-Hinweis | ToyChain (Nr. 80, 87), TOYPAY-Stufen |
| **F Prüflabor** | Pentests im lokalen Devnet oder mit Freigabe: Doppelausgabe, Wiederholung mit falscher Chain-ID, Spam über das Kontingent, Doppelsignatur, Überlauf, kaputte Transaktionen (Fuzzing). Prüft, **ob die Regeln greifen**. Funde als vertraulicher Bericht | ToyChain-Tests, Bug-Bounty (Nr. 88) |
| **G Dienste-Prüfung** | Für eigene Instanzen: Webseite (Sicherheits-Header, keine Inhalte Dritter), ToyPost (Geheimnisse nicht im Repository, Webhooks signiert), später Polygon-Verträge (kein Admin, Limits, Notschalter) und ToyCash-Emittent (signierter Reserve-Bericht) | Webseite, ToyPost, TOYQUEST, ToyCash |
| **H Lernen und Üben** | Phishing-Übungen, Sicherheits-Quiz, Missionen mit der weisen Warnerin in TOYQUEST. Belohnung nur Wissens-Siegel, nie TOY | TOYQUEST (Nr. 129) |

**ToySafe braucht keine eigene Transaktionsart.** Alles, was auf der Kette wirkt, gibt es schon als fest eingebaute Funktion (Konto-Schutz, Nr. 83, M12). ToySafe macht sie nur leicht nutzbar. Das passt zu „jede Funktion ist eine fest eingebaute Transaktionsart“ (Nr. 36).

## 3. Synergien mit den anderen Teilen

| Mit | ToySafe gibt | ToySafe bekommt |
|---|---|---|
| **TOYPAY** | Risiko-Prüfung vor jeder Signatur (lokal, nur Hinweis), Schutzstufe, Konto-Schutz-Assistent, Notfall-Knopf | vorbereitete Aktionen werden in TOYPAY mit Vorschau signiert. Gemeinsamer Rust-Kern, damit TOYPAY die Prüfungen auch ohne ToySafe-App nutzen kann |
| **ToyChain** | Prüflabor im Devnet, Knoten-Prüfung, Fuzzing-Fälle | feste Werte aus `shared/toyverse.json`, später JSON-RPC (M7) |
| **ToyScan** | Muster neuer Betrugsadressen als Vorschlag für die öffentliche Warnliste (nur Warnung) | Adress-Alarm, Ampeln, Supply-Daten |
| **ToyPost** | signierte Warnungen und Prüfberichte als Ereignisse, Warnliste | Benachrichtigung in 45 Sprachen über den gewählten Kanal, Betrugsmuster aus Nachrichten (ohne Inhalt) |
| **TOYQUEST** | Übungen und Missionen „Falle erkennen“ | Wissens-Siegel als Motivation, Erklärungen der NPCs |
| **ToyCash** | Prüfung von Gutschein-Codes (gelten als Geheimnis), Händler-Kasse, Reserve-Bericht | ToyVerse-Ampel B1 |
| **Webseite** | Selbsttest der Seite (keine Drittinhalte, Header) | Erklärseite ToySafe in 45 Sprachen |

Jede Verbindung ist **optional**. Fehlt ToySafe, warnt TOYPAY mit seinen eingebauten Prüfungen weiter. Fehlt ToyPost, meldet ToySafe lokal.

## 4. Anwendungsfälle

| Nr. | Fall | Ablauf |
|---|---|---|
| U1 | **Sicher senden** | TOYPAY fragt vor dem Signieren den ToySafe-Kern: neue Adresse? ähnlich wie eine bekannte? auf der Warnliste? altes ToyCoin-Token? Die Vorschau zeigt das Ergebnis als Ampel und schlägt bei neuen Adressen einen Probebetrag vor. Die Entscheidung bleibt beim Nutzer |
| U2 | **Handy verloren** | ToySafe auf einem zweiten Gerät (nur Ansehen) meldet einen Abfluss. Mit dem Notfall-Schlüssel löst der Nutzer die Selbst-Sperre aus. Die Tresor-Verzögerung gibt ihm Zeit, bevor große Beträge abfließen können |
| U3 | **Phishing-Nachricht** | ToyPost erkennt die Frage nach der Phrase und warnt. Mit Zustimmung geht nur das Muster (Domain, Adresse, ohne Inhalt) als Vorschlag an die Warnliste. Nach Prüfung warnen TOYPAY, ToySafe und ToyScan alle Nutzer |
| U4 | **Knotenbetreiber** | Wöchentlicher Selbsttest des eigenen Knotens. Auf Wunsch erscheint ein Prüfsiegel auf der Knotenkarte in ToyScan |
| U5 | **Händler** | Selbsttest der Kasse, Gutschein-Codes nie im Klartext über unsichere Kanäle, Warnung bei ungewöhnlichen Rückholungen (ToyCash) |
| U6 | **Vererben** | Assistent für die Erben-Adresse mit 30 Epochen Abbruchfrist (Nr. 106). Erinnerung über ToyPost, solange der Eigentümer aktiv ist |
| U7 | **Entwickler und Pentester** | Prüflabor gegen das Devnet in einem Befehl (Nr. 101). Fund → vertraulicher Bericht → Behebung → Bug-Bounty → Mitteilung über ToyPost |
| U8 | **Schutz für alle** | Eine öffentlich bekannte Betrugsadresse landet nach Prüfung auf der signierten Warnliste. Jede Wallet, auch fremde, kann sie lesen. Gesperrt wird nichts |
| U9 | **Item prägen** | Vor dem Prägen prüft ToySafe Domain und Vertragsadresse der Prägeseite. TOYPAY signiert erst danach |
| U10 | **Lernen** | Mission in TOYQUEST: drei Nachrichten, eine davon Phishing. Wer sie erkennt, bekommt ein Wissens-Siegel |

## 5. Ausbaustufen

| Stufe | Inhalt | Voraussetzung |
|---|---|---|
| 1 | Sicherheits-Check, Warnliste, Adress- und Link-Prüfung offline, Webseiten-Selbsttest, Prüflabor gegen das lokale Devnet | geht ohne Testnet |
| 2 | Wächter, Knoten-Prüfung, Prüfsiegel für Knoten | M7 (RPC), M10 (Testnet), M11 (ToyScan) |
| 3 | Konto-Schutz-Assistent und Notfall-Knopf | M12 |
| 4 | Prägeseiten, ToyCash-Kasse, Reserve-Bericht | TOYQUEST-Items, ToyCash |

## 6. Schnittstellen (Vorschlag)

### 6.1 Feste Werte

ToySafe liest Chain-IDs, Dezimalstellen, Transaktionsarten und die alten ToyCoin-Token aus `shared/toyverse.json` (Abschnitt 4 in `15-verknuepfung-und-kompatibilitaet.md`) und hält keine eigenen Kopien.

### 6.2 Risiko-Prüfung für TOYPAY

Lokal im Rust-Kern, kein Netz nötig:

```text
Anfrage:  { version: 1, chainId, from, to, amountUnits: "…", memo?, origin?, kind }
Antwort:  { version: 1, level: "gruen" | "gelb" | "rot", reasons: [{ code, text_key }], suggestTestAmount: bool }
```

`text_key` verweist auf die gemeinsamen Texte in 45 Sprachen. „Rot“ heißt nur: deutlich warnen und zweimal bestätigen lassen. Senden bleibt möglich (G3).

### 6.3 Warnliste

```text
{ schema: "toysafe-warnliste/1", version, created, entries: [ { kind: "address" | "domain" | "pattern",
  value, chainId?, reason_key, source, since, evidence_url? } ], signature }
```

Signiert mit einer Signatur nach EIP-191 durch mindestens eine bekannte Projektadresse, später mehrere (Nr. 87). Einträge haben einen Beleg und lassen sich zurücknehmen. Die Liste sperrt nichts, sie warnt.

### 6.4 Prüf-Freigabe für aktive Tests

```text
ToySafe-Pruef-Freigabe v1
Ziel: <Knoten-Adresse, Domain oder Instanz>
Pruefungen: <Liste>
Von: <Datum>  Bis: <Datum>
Pruefer: <Adresse>
```

Der Eigentümer des Ziels signiert diesen Text mit seiner Adresse (EIP-191) oder legt ihn als Datei auf seine Domain. Ohne gültige Freigabe startet ToySafe keine aktive Prüfung gegen fremde Ziele.

### 6.5 Ereignisse für ToyPost

`toysafe.warnung`, `toysafe.pruefbericht`, `toysafe.warnliste`, signiert wie die ToyScan-Webhooks (`10-toypost-schnittstelle.md`, Abschnitt 11).

## 7. Technik (Vorschlag)

- **Rust-Kern** `toy-safe` mit allen Prüfungen. Ihn nutzen die ToySafe-App, TOYPAY (eingebaut) und eine Kommandozeile `toysafe` für Knotenbetreiber und das Prüflabor.
- **Oberfläche** wie TOYPAY mit Compose Multiplatform für Android und Desktop (`../04-toypay-wallet.md`), Anbindung über UniFFI (Nr. 77). Zielplattformen wie der Knoten, auch alte Android-Geräte.
- **Prüflabor** nutzt die bestehenden Fuzzing-Ziele und Integrationstests des Workspace und startet ein lokales Devnet (Chain-ID 7630715).
- **Ort im Repository** offen: eigener Ordner `toysafe/` in `TOYfoundation/toycoin` (wie `toyscan/`) oder eigenes Repository `TOYfoundation/ToySafe`. Empfehlung: Ordner in toycoin, damit Kern, Tests und feste Werte an einem Ort bleiben.

## 8. Grenzen

- ToySafe kann Risiken senken, nicht ausschließen. Ein gehacktes Gerät kann auch ToySafe täuschen. Darum gilt weiter: Phrase nie weitergeben, Hardware-Wallet für große Beträge.
- Root- und Jailbreak-Erkennung ist nie sicher, sie ist nur ein Hinweis.
- Werkzeuge für Sicherheitstests sind rechtlich heikel. Vor der Veröffentlichung des Prüflabors ist eine rechtliche Prüfung nötig. Die Freigabe-Pflicht (G2) ist im Code durchzusetzen und mit Tests zu belegen.
- Warnlisten können irren. Jeder Eintrag braucht einen Beleg und einen Weg zur Rücknahme.

## 9. Vorschläge zur Entscheidung

| Nr. | Vorschlag |
|---|---|
| S1 | Grundsätze G1 bis G8 |
| S2 | Bausteine A bis H und Ausbaustufen 1 bis 4 |
| S3 | Gemeinsamer Rust-Kern `toy-safe`, den TOYPAY einbaut |
| S4 | Öffentliche, signierte Warnliste im Format 6.3 als gemeinsamer Dienst für alle Wallets |
| S5 | Prüf-Freigabe 6.4 als Pflicht für aktive Tests gegen fremde Ziele |
| S6 | Ort im Repository: Ordner `toysafe/` in `TOYfoundation/toycoin` |
