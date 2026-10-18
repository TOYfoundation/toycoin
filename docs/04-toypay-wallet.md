# TOYPAY Wallet: Wallet und Knoten in einem

Entwurf v0.1 vom 18.10.2026 · Grundlage: Spezifikation v0.3 (Abschnitte 4, 8, 9, 11), Entscheidungen 61 bis 77 · Status: **entschieden** bis auf Abschnitt 10 · Spiel, Spiel-NFTs und Dukaten: `spaeter/07-toypay-spiel-und-nft.md`

## 1. Ziel

Die TOYPAY Wallet verwaltet TOY, ist zugleich ein **ToyChain-Knoten** und läuft auf möglichst vielen Geräten, vom alten Android-Handy bis zum Server. Jeder kann ohne Guthaben mitmachen und mit gebundenem TOY schrittweise mehr Rechte bekommen. Dazu zeigt und nutzt die Wallet dieselbe Adresse auch auf anderen EVM-Ketten.

## 2. Plattformen und Client-Typen

| Client | Für wen | Plattformen |
|---|---|---|
| **TOYPAY App** | alle Nutzer, mit Oberfläche | Android (ab ca. 1 GB RAM, arm64 und armv7), Windows, Linux, Raspberry Pi (Linux arm64) |
| **ToyNode** | Dauerbetrieb ohne Oberfläche, z. B. Raspberry Pi oder Server | Linux x86_64 und arm64, Windows. Bedienung über Kommandozeile (`toy-cli`) und eine lokale Weboberfläche |
| **Kern-Bibliothek** | andere Apps, die TOY einbauen wollen | Rust-Kern mit Anbindung für Kotlin (Android und Desktop) |

**Ein gemeinsamer Rust-Kern:** Kryptografie, Transaktionen, Knoten und Wallet stecken in denselben Crates (`toy-crypto`, `toy-tx`, `toy-node`, `toy-wallet`). Nichts davon wird in einer anderen Sprache neu geschrieben. **Vorschlag:** Die Oberfläche läuft mit Compose Multiplatform, dann gibt es **eine** Codebasis für Android und Desktop. Die Rust-Anbindung erzeugt UniFFI (Lizenz MPL-2.0, für unveränderte Nutzung freigegeben, Entscheidung 77).

## 3. Stufen

Die Stufen sind in der Spezifikation (Abschnitt 4) festgelegt. In der App sieht das so aus:

| Stufe | Anzeige in der App | Typisches Gerät |
|---|---|---|
| 0 · Leicht | „Mitmachen ohne Guthaben“ | altes Handy, Tablet |
| 1 · Konto | „Senden mit freiem Kontingent“ | jedes Gerät mit aktiviertem Konto (0,1 TOY Aktivierungs-Pfand) |
| 2 · Attestierer | „Mitsignieren“, ab 10 TOY Stake | Handy am Ladegerät, PC |
| 3 · Voll | „Voll-Knoten“, ab 100 TOY Stake | PC, Raspberry Pi 4/5 mit SSD |
| 4 · Validator | „Validator“, ab 1.000 TOY Stake | ständig laufender PC, Pi oder Server |

- Beim ersten Start misst die App kurz das Gerät (Speicher, Kerne, Akku, Netz) und **schlägt eine Stufe vor**. Der Nutzer entscheidet und kann jederzeit wechseln.
- Höhere Stufen sind nur wählbar, wenn das Gerät sie schafft. Eine Validator-Rolle auf einem Handy mit 1 GB RAM bietet die App gar nicht an.
- Die App zeigt offen, was jede Stufe bringt, was sie kostet (Strom, Daten, Speicher) und welche Sperrzeiten bei Verstößen drohen (Spezifikation Abschnitt 8).

## 4. Energie-Modus

Die App passt ihren Aufwand **automatisch** an. Der Nutzer wählt nur die Obergrenze.

| Modus | Verhalten |
|---|---|
| **Sparsam** | nur Leicht-Knoten, kein ToyWork, synchronisiert selten. Für alte Geräte und unterwegs |
| **Ausgewogen** (Standard) | ToyWork nur am Ladegerät, im WLAN und unter einer Temperaturgrenze, mit 1 Kern |
| **Leistung** | alle freigegebenen Kerne, auch Voll-Knoten und Mitsignieren, solange Strom und Temperatur passen |
| **Auto** | wechselt selbst zwischen den Modi, je nach Akkustand, Ladegerät, Temperatur, Netz (WLAN oder mobil), Datenvolumen und ob das Gerät gerade benutzt wird |

**Regeln für Auto (Startwerte):**
- Unter 30 % Akku ohne Ladegerät: Sparsam.
- Am Ladegerät und im WLAN: bis zur gewählten Obergrenze.
- Gerätetemperatur über 40 °C: Arbeit pausieren.
- Mobiles Netz: höchstens 20 MB pro Tag, sonst nur Kopfdaten.
- Wird das Gerät aktiv benutzt: Arbeit auf niedrigste Priorität.

**Wichtig für höhere Stufen:** Ein Validator, der wegen Akku oder Hitze plötzlich pausiert, gilt als inaktiv (Spezifikation Abschnitt 5.4). Die App warnt deshalb, bevor der Auto-Modus eine Rolle mit Stake herunterstuft, und empfiehlt für Stufe 4 ein dauerhaft versorgtes Gerät.

## 5. Mint-Funktionen

### 5.1 TOY verdienen

| Weg | Stufe | Wie |
|---|---|---|
| **Arbeits-Lotterie** | ab 0 | Das Gerät rechnet ToyWork-Lose (Argon2id, 16 MiB) und reicht sie ein. Je Block werden Gewinner gezogen. Jeder Gewinn ist mindestens 0,2 TOY. Beim ersten Gewinn werden 0,1 TOY zum Aktivierungs-Pfand, der Rest ist frei |
| **Mitsignieren** | ab 2 | Anteil an der Konsens-Belohnung (nach der Startphase, vorher geht er an die Start-Validatoren) |
| **Voll-Knoten** | ab 3 | zusätzliches Gewicht für Verfügbarkeit |
| **Validator** | 4 | Konsens-Belohnung und 20 % der Gebühren des eigenen Blocks |
| **Gebühren-Lotterie** | ab 2 | Anteil an 40 % der Gebühren für aktive Teilnehmer |

Die App zeigt die erwartete Belohnung je Tag als Spanne und ehrlich als Schätzung. Versprochen wird keine Rendite.

### 5.2 NFTs prägen

**Später** mit der Spiel-Anbindung (Entscheidung 68). Die Testnet-Wallet prägt keine NFTs. Sie **zeigt** aber NFTs, die eine Adresse auf anderen EVM-Ketten besitzt (Abschnitt 6). Ideen für später: `spaeter/07-toypay-spiel-und-nft.md`.

## 6. Andere Ketten (Entscheidung 69)

- **Dieselbe Adresse** wie auf ToyChain funktioniert auf allen EVM-Ketten. Die Wallet zeigt dort Guthaben, Token und NFTs und kann senden.
- **Zum Start:** Ethereum, Polygon, Arbitrum, Base, BNB Chain und deren Testnetze. Weitere EVM-Ketten lassen sich mit Chain-ID und RPC-Adresse selbst hinzufügen. Nicht-EVM-Ketten wie Bitcoin oder Solana kommen später, weil sie andere Schlüssel und Ableitungspfade brauchen.
- **Eigene Schlüssel je Kette (optional):** Wer ToyChain-Guthaben getrennt halten will, wählt einen eigenen Ableitungspfad für ToyChain (Spezifikation Abschnitt 10).
- **RPC-Zugänge:** Die App nutzt öffentliche Zugänge oder vom Nutzer eingetragene. **Kein API-Schlüssel steckt in der App**, weil jeder ihn aus der Installationsdatei auslesen könnte.
- **Warnung bei gleichnamigen Token:** Ein Token mit dem Namen ToyCoin oder dem Ticker TOY auf einer anderen Kette bekommt immer den Hinweis „Nicht ToyChain-TOY, nicht eintauschbar“. Natives TOY erscheint nur unter dem Netzwerk ToyChain.
- **Keine Bridge** im jetzigen Umfang.

## 7. Konto aktivieren und auflösen

- Die App erklärt beim ersten Eingang: „0,1 TOY sind als Aktivierungs-Pfand gebunden. Sie gehören weiter dir und kommen beim Auflösen zurück.“
- Vor einer Überweisung an eine neue Adresse prüft die App, ob sie mindestens 0,1 TOY beträgt.
- **Ruhende Konten:** Wird das eigene Konto ausgelagert, speichert die App den Beweis (knapp 1 KB) selbst. Beim nächsten Senden oder Empfangen weckt sie das Konto automatisch auf. Geht die App verloren, holt sie den Beweis nach der Wiederherstellung mit der Phrase von Archiv-Knoten oder ToyScan. Vor einer Zahlung an eine ruhende Adresse holt die App deren Beweis ebenfalls automatisch.
- **Konto auflösen** in den Einstellungen: Die App löst zuerst gebundenen Stake (7 Epochen), dann schickt sie alles samt Pfand an eine Zieladresse. Wahlweise spendet der Nutzer den Rest in den Erstattungs-Pool. Kontingent und Stufe lassen sich nicht übertragen.

## 8. Sicherheit

- Phrase nach BIP-39, verschlüsselt mit Argon2id und XChaCha20-Poly1305, wo möglich im Android Keystore/StrongBox, im TPM oder in der Secure Enclave.
- Schlüssel und Phrasen werden **nie** geloggt, nie im Klartext gespeichert und nach Gebrauch gelöscht (`zeroize`).
- Validator-Schlüssel (BLS) sind getrennt vom Geld-Schlüssel. Wer Stufe 4 betreibt, braucht den Geld-Schlüssel dafür nicht auf dem Gerät.
- Signieren mit Chain-ID, Anzeige von Empfänger, Betrag, Kette und Gebühr vor jeder Unterschrift.
- Im Testnet: keine echten Schlüssel, keine echten Gelder.

## 9. Bauplan-Bezug

| Funktion | Meilenstein |
|---|---|
| Schlüssel, Adressen | M2 |
| Kontingent, Gebühren, ToyWork-Lose | M3 |
| Stufen, Startphase, Sperrzeiten | M3, M4 |
| Ethereum-kompatible Schnittstelle | M7 |
| Wallet-Kern, Keystore, Stufenwahl | M8 |
| Plattformen, Energie-Modus | M9 |
| Andere Ketten | M11 |

## 10. Offene Punkte

1. **Oberfläche:** Compose Multiplatform für Android und Desktop? (Wird im Plan zu M9 vorgeschlagen.)
