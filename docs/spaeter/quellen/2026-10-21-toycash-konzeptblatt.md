# ToyCash (bisher Invisible Coin) – Konzeptblatt: Methodik und Erweiterungen

Entwurf vom 21.10.2026 · Recherchestand Oktober 2026 · Theoretisches Konzept, noch nicht Teil der Phase-1-Spezifikation der ToyChain

## 1. Idee in einem Satz

Ein durch Euro-Reserven gedeckter Stablecoin (Dollar später möglich), dessen Besitz und Beträge nur in der eigenen Wallet des Teilnehmers sichtbar sind. Die ToyChain dient nur als Notar, der Hashes bezeugt. Zahlungen sind auch offline möglich.

## 2. Bisherige Entscheidungen

- Stablecoin mit **Euro-Reserven (Dollar später möglich)**, **eigener Emittent** (Variante A).
- Existiert **separat** auf der ToyChain, nur in der eigenen **Invisible-Wallet** sichtbar, später in TOYPAY integrierbar.
- Nur Sender und Empfänger kennen den Betrag.
- Eigene Methodik aus ToyChain, Smart Contracts, eigener App und Verschlüsselung, die auch Offline-Transaktionen erlaubt.
- **Vorschlag:** Ein zweites Asset gehört frühestens in Phase 3 der ToyChain. Schon jetzt ein Feld `asset_id` einplanen (TOY = 0).

## 3. Kernmethodik „Invisible Ledger“ (Vorschlag)

Prinzip: **Client-Side Validation.** Die Kette führt kein Kontobuch. Besitz und Historie liegen bei den Teilnehmern.

1. **App (Invisible-Wallet):** Hält die Coins als verschlüsselte Notes samt Besitzbeweis. Bei jeder Zahlung geht ein Paket aus Note und Beweis an den Empfänger.
2. **ToyChain als Notar:** Speichert nur Hashes. Jeder Coin hat ein Siegel, das genau einmal geschlossen werden kann. Viele Zahlungen werden zu einem Hash gebündelt. Das spart Platz im 256-MB-Budget.
3. **Smart Contract (Polygon):** Zählt Ausgabe und Rücktausch öffentlich, damit die Gesamtmenge prüfbar bleibt. Jeder Coin muss auf eine gezählte Ausgabe zurückführbar sein.
4. **Verschlüsselung:** Nur geprüfte Verfahren, dazu Beweise, die den Besitz belegen, ohne ihn zu zeigen.
5. **Offline:** Pakete per QR, NFC oder Bluetooth. Ein vorab hinterlegtes Offline-Budget begrenzt das Risiko.

## 4. Ablauf

1. **Einzahlung:** Der Nutzer zahlt Euro beim Emittenten ein. Der Smart Contract zählt die Ausgabe, die Wallet erhält verschlüsselte Notes.
2. **Zahlung online:** Paket an den Empfänger, Empfänger prüft, danach wird das Siegel auf der ToyChain gesetzt (gebündelter Hash).
3. **Zahlung offline:** Paket per NFC, QR oder Bluetooth. Der Empfänger prüft ohne Netz und setzt das Siegel später online. Das Restrisiko ist durch das Offline-Budget begrenzt.
4. **Auszahlung:** Der Nutzer löst Coins beim Emittenten ein. Der Smart Contract zählt den Rücktausch, der Emittent überweist Euro.

## 5. Erweiterungen aus der Recherche (Stand Oktober 2026)

### 5.1 Hybrid-Anker nach dem RGB++-Prinzip

RGB hält den Zustand außerhalb der Kette und nutzt Bitcoin nur für Verpflichtungen und Besitz-Anker. Die Wallet muss dabei Austausch und Prüfung der Historie leisten, und das gilt als komplex. RGB++ koppelt eine zweite Kette als überprüfbaren Zustandsanker, sodass nicht jeder die volle Historie prüfen muss. Tether hat angekündigt, USDT auf RGB zu bringen. Das zeigt, dass Stablecoins in diesem Modell ernsthaft verfolgt werden.

**Für ToyChain:** Die Kette speichert zusätzlich Epochenwurzeln als Zustandsanker. Empfänger auf alten Geräten müssen dann nicht die ganze Historie prüfen.

### 5.2 eCash-Bausteine (Cashu) für den schnellen Offline-Prototyp

- **Blinde Signaturen:** Der Emittent kann Ausgabe und Einlösung nicht verknüpfen.
- **DLEQ-Beweis (NUT-12):** Erlaubt die Prüfung von Token zwischen Nutzern ohne Netz.
- **Empfänger-Sperre (P2PK, NUT-11) und HTLC (NUT-14):** Token lassen sich an den Schlüssel des Empfängers binden.
- **Seed-basierte Wiederherstellung (NUT-13):** Guthaben lassen sich nach Geräteverlust zurückholen, wenn die Seed vorher gesichert wurde.
- **Schon vorgeführt:** Offline-eCash per NFC-Tippen und per Bluetooth.

**Einschränkung:** Empfänger sollten später beim Emittenten gegenprüfen, sonst bleibt Doppelausgabe möglich. Fällt ein einzelner Emittent dauerhaft aus, sind seine Token wertlos. (Extra Anmerkung: Doppelausgabe kann auch als Nützliche Funktion eingebaut werden. Bitte ausarbeiten und vorschlagen)

### 5.3 Föderierter Emittent und Gesundheitsanzeige

- **Föderation (Fedimint-Prinzip):** Statt eines Betreibers hält eine Gruppe von Guardians die Reserven per Schwellen-Signatur. Mehrere müssen gleichzeitig falsch handeln, damit Gelder verschwinden. Eine Föderation mit sieben Guardians läuft laut einer Übersicht seit Mitte 2026 im Echtbetrieb.
- **Auditor-Prinzip (Cashu):** Automatische Testzahlungen machen die Verfügbarkeit eines Emittenten sichtbar. ToyScan kann Verfügbarkeit, Umlauf und gemeldete Reserven in einer Ampel zeigen.

**Einschränkung:** Das System bleibt verwahrend. Die rechtliche Verantwortung bleibt beim Emittenten.

### 5.4 Komprimierte Besitzbeweise durch Folding

Folding-Verfahren gelten laut einem Überblick von 2026 als einfachster und schnellster Weg zu rekursiven Beweisen mit konstantem Aufwand pro Schritt. Aztecs CHONK ist so gebaut, dass der Beweis mit wenig Speicher auf einem Handy entsteht. Die Ethereum Foundation beschreibt mit PlasmaBlind eine Konstruktion, bei der der Nutzer vorläufig in unter 100 ms beweist. FibRace misst Beweise auf Handys weltweit, und das Toolkit Mopro erleichtert sie.

**Nutzen:** Der Besitzbeweis bleibt klein, auch nach tausenden Weitergaben. Das löst das Wachstum der Historie in den Paketen.

**Einschränkung:** Noch frühe Forschung. Messung auf alten Android-Geräten ist Pflicht.

### 5.5 Offline-Regeln nach dem Vorbild des digitalen Euro

Der digitale Euro sieht vor: Aufladen online oder am Automaten, Zahlung direkt zwischen Geräten, Haltegrenzen, Hardware-Sicherheit. Die Forschungsarbeit PayOff beschreibt Halte-, Sende- und Empfangsgrenzen und Schutz bei Ausfall des Sicherheitselements. Das Europäische Parlament will Offline-Zahlungen gebührenfrei.

**Für Invisible Coin:** Offline-Wallet mit Obergrenze, Sende- und Empfangslimits, gerätegebundener Schlüssel, Zurücksetzen der Limits nur online.

### 5.6 Compliance-Beweise ohne Identität (Privacy Pools)

Privacy Pools erlauben einen Beweis, dass eine Einzahlung zu einer geprüften Menge gehört (Association Set), ohne zu verraten, welche. Die Menge pflegt ein Anbieter (ASP). Das Protokoll von 0xbow ist auf Ethereum live, ein Papier mit Vitalik Buterin und Chainalysis-Forschern beschreibt die Idee.

**Für Invisible Coin:** Der Emittent führt eine Menge geprüfter Einzahlungen. Nutzer beweisen die Zugehörigkeit, ohne ihre Identität zu zeigen. Dazu kommen Ansichtsschlüssel für Prüfer.

**Einschränkung:** Frühe Phase, die Nutzung ist bisher bescheiden, und die Pflege der Mengen ist eine Vertrauensfrage.

### 5.7 Post-Quanten-Schutz ohne Kettenballast

Post-Quanten-Signaturen sind viel größer (ML-DSA-65 etwa 3.309 Byte gegenüber 64 Byte bei ECDSA). Laut Hedera hat kein großes Netz die Migration vollständig geschafft. NEAR unterstützt ML-DSA-65 seit Juli 2026, FN-DSA (FIPS 206) ist noch nicht final.

**Eigene Einschätzung:** Beim Invisible Coin liegen die Signaturen bei den Notes in der App, nicht in der Kette. Die Größe belastet das 256-MB-Budget daher kaum. Vorschlag: Besitzschlüssel hybrid (klassisch plus ML-DSA) und Krypto-Agilität von Anfang an.

### 5.8 Wallet-Komfort auf der Polygon-Seite

- **Account Abstraction (ERC-4337) und EIP-7702** (seit Mai 2025 live): Paymaster, Passkeys (WebAuthn), Session-Keys, Gas in Stablecoin (z. B. EURC oder USDC).
- **Verifizierende Paymaster mit Gas-Richtlinien** schützen davor, dass der Topf leergesaugt wird.
- Dienste wie Pimlico, Alchemy, Biconomy, ZeroDev und Candide gibt es bereits. Das passt zum POL-Topf von TOYQUEST.

### 5.9 Doppelausgabe als nützliche Funktion (Vorschlag)

Bei Offline-Zahlungen ist Doppelausgabe sonst ein Fehler. Man kann sie aber gezielt in drei nützliche Funktionen verwandeln:

1. **Betrug wird bestraft und entschädigt:** Beim klassischen Offline-eCash (Chaum, Fiat, Naor und später Brands) bleibt ein ehrlicher Nutzer anonym. Wer dasselbe Siegel zweimal ausgibt, verrät dabei ein Geheimnis und wird identifizierbar. Dient das Offline-Budget als Kaution, wird der Geschädigte sofort daraus entschädigt, und der Betrüger verliert den Betrag. So lohnt sich Betrug nicht, und Empfänger können offline kleine Beträge sicher annehmen.
2. **Rückholfrist (Storno):** Jede Offline-Zahlung hat eine Ablaufzeit. Löst der Empfänger nicht rechtzeitig ein, darf der Sender die Note zurückholen. Das ist eine kontrollierte zweite Verwendung, ähnlich den Zeitsperren bei HTLCs. Sie hilft bei falschem Empfänger oder verlorenem Gerät.
3. **Reservierung mit Teilabbuchung (Pre-Authorization):** Der Sender sperrt einen Rahmen, etwa für Tanken oder ein Hotel. Der Händler bucht später den tatsächlichen Betrag ab, der Rest geht automatisch zurück. Ein Siegel wird so kontrolliert mehrfach genutzt.

**Zusammenspiel mit der Offline-Obergrenze:** Der Nutzer wählt seinen Offline-Rahmen selbst. Dieser Rahmen ist zugleich die Kaution für Funktion 1. Je höher der Rahmen, desto strenger die Anforderung an das Gerät.

**Grenzen:** Die Identitäts-Aufdeckung gilt nur bei Betrug, ehrliche Zahlungen bleiben unsichtbar. Die Technik ist aufwendiger als einfache Token und braucht sorgfältige Prüfung. Die Kaution funktioniert am besten mit einem gerätegebundenen Sicherheitselement.

**Empfehlung:** Mit der Rückholfrist (2) beginnen, weil sie einfach ist. Die Betrugs-Strafe (1) kommt mit der Offline-Wallet in Stufe 2, die Reservierung (3) später.

### 5.10 Gutscheine: mehrfach oder von jedem einlösbar (Vorschlag)

Ein Gutschein ist ein Siegel, das bewusst mehrfach geöffnet werden darf, aber nur innerhalb fester Grenzen. Die ToyChain zählt als Notar die Einlösungen in gebündelten Hashes. Niemand sieht Namen oder Beträge.

**Drei Arten:**

1. **Mehrfach-Gutschein:** Fester Gesamtbetrag, Betrag je Einlösung und höchstens erlaubte Zahl der Einlösungen (z. B. 5 × 10 €). Was nach Ablauf übrig ist, geht über die Rückholfrist aus 5.9 an den Aussteller zurück.
2. **Gruppen-Gutschein:** Einlösbar nur durch eine festgelegte Gruppe von Schlüsseln (z. B. Familie, Gilde, Team). Jeder aus der Gruppe darf bis zum Gesamtbudget einlösen.
3. **Inhaber-Gutschein:** Wer den Code oder QR hat, kann einlösen, wie bei Bargeld oder einer Geschenkkarte. Wer zuerst kommt, bekommt es. Das passt für Geschenke, Aktionen und Spenden.

**Gemeinsame Regeln:** Ablaufzeit, Obergrenze je Einlösung, Gesamtbudget, optional Geheimwort oder PIN, Limit pro Person und Tag. Die Betrugs-Strafe und die Kaution aus 5.9 gelten auch hier.

**Offline:** Einlösen offline nur bis zur Offline-Obergrenze des Einlösers, die Zählung wird später online festgeschrieben. Bei Inhaber-Gutscheinen könnten zwei Personen gleichzeitig offline einlösen, deshalb dort nur kleine Beträge.

**Anwendung:** In TOYPAY zum Beispiel Geschenk-QR, Trinkgeld, Spenden und Familienkasse. In TOYQUEST zum Beispiel Gilden-Kasse, Event-Gutscheine und Belohnungen.

**Grenzen:** Inhaber-Gutscheine sind wie Bargeld. Verloren oder gestohlen heißt weg, und die Weitergabe lässt sich nicht kontrollieren. Öffentliche Codes brauchen Limits und PIN gegen Bots. Rechtlich sind Gutscheine mit Geldwert eventuell reguliert, das gehört zum Schluss geprüft.

**Empfehlung:** Mit dem Mehrfach-Gutschein beginnen, weil er nur einen Zähler braucht. Danach der Gruppen-Gutschein, zuletzt der Inhaber-Gutschein.

## 6. Empfohlene Reihenfolge

1. **Prototyp:** eCash-artige Token (blinde Signaturen, DLEQ, P2PK) mit Test-Token, ToyChain-Anker zunächst simuliert.
2. **Offline-Wallet:** Haltegrenzen, Seed-Backup, ToyScan-Ampel für den Emittenten (Umlauf gegenüber gemeldeter Reserve).
3. **zk-Notes:** Folding-Beweise, Hybrid-Anker, Post-Quanten-Hybridschlüssel.
4. **Compliance und Föderation:** Association Sets, föderierter Emittent.

Alles erst nach einem stabilen Phase 1 bis 3 der ToyChain.

## 7. Risiken und Grenzen

- **Datenverlust:** Ohne Backup sind Coins weg, denn die Kette hat keine Kopie.
- **Vertrauen:** Euro-Reserven liegen außerhalb der Kette und sind nur über Berichte und Prüfungen belegbar.
- **Offline-Doppelausgabe:** Bleibt bis zum Setzen des Siegels ein Restrisiko.
- **Metadaten:** Zeitpunkte, Netzwerkdaten und die Ein- und Auszahlung verraten etwas.
- **Reife:** Folding und Association Sets sind neu. Vor dem Einsatz sind Messungen und Audits nötig.
- **Recht (kurz):** Bei Euro-/Dollar-Stablecoins besonders sensibel, Prüfung zum Schluss.

## 8. Entscheidungen und offene Punkte

**Entschieden am 21.10.2026:**

1. **Startmethodik:** Hybrid, erst eCash-Prototyp, später zk-Notes.
2. **Emittent:** Erst einzeln, später föderiert.
3. **Assets:** Zunächst nur Euro. Dollar später möglich, das Feld `asset_id` ist vorgesehen.
4. **Offline-Obergrenze:** Option D: Sie startet bei 20 € und wächst mit der Nutzung bis höchstens 500 €. Der Nutzer wählt seinen Rahmen innerhalb der freigeschalteten Grenze selbst. Das Wachstum ist langsam (Stufenwerte als Vorschlag): 20 € zum Start, 50 € nach 14 Tagen und 10 Zahlungen, 100 € nach 60 Tagen und 40 Zahlungen, 250 € nach 180 Tagen und 100 Zahlungen, 500 € nach 365 Tagen und 250 Zahlungen. Über 100 € nur mit gerätegebundenem Schlüssel. Bei Betrug oder Streit um Rückholungen wird zurückgestuft, und der Nutzer darf seinen Rahmen jederzeit niedriger stellen.
5. **Doppelausgabe-Funktionen:** Alle drei, stufenweise: erst Rückholfrist, dann Betrugs-Strafe, dann Reservierung (siehe Abschnitt 5.9). (Möglich denkbar auch als Gutscheine die von mehreren oder jedem eingelöst werden können)

**Name (entschieden):** ToyCash. Invisible Coin war der Arbeitstitel. Wo im Text noch Invisible Coin oder Invisible-Wallet steht, ist ToyCash gemeint.

**Gutscheine (entschieden):** Alle drei Arten, stufenweise: erst Mehrfach-Gutschein, dann Gruppen-Gutschein, zuletzt Inhaber-Gutschein (siehe Abschnitt 5.10).

**Noch offen:**

- **Stufenwerte des langsamen Wachstums** bestätigen oder anpassen.
- **Marke und Domain** für ToyCash prüfen (Verwechslungen, Verfügbarkeit).

## 9. Quellen (Recherche Oktober 2026, teils Fachblogs, vor Entscheidungen Primärquellen prüfen)

- Spark: RGB Protocol: Smart Contracts and Assets on Bitcoin (11.03.2026), spark.money/research/rgb-protocol-bitcoin-smart-contracts
- Spark: Client-Side Validation: How RGB and CSV Protocols Scale Bitcoin (29.07.2026), spark.money/research/client-side-validation-scaling-bitcoin
- Nervos Docs: RGB++ Protocol, docs.nervos.org/docs/tech-explanation/rgbpp
- Spark: Cashu vs Fedimint, spark.money/tools/bitcoin-cashu-vs-fedimint-comparison
- Spark: eCash und Offline Payment (Glossar), spark.money/glossary/ecash und spark.money/glossary/offline-payment
- OpenSats: Fedimint und Advancements in Ecash, opensats.org/topics/fedimint und opensats.org/blog/advancements-in-ecash
- Cashu Blog: Q1 2026 Highlights, blog.cashu.space/q1-2026-highlights
- A survey of folding-based zero-knowledge proofs (Information Sciences, 2026), repository.seku.ac.ke/handle/123456789/8283
- Aztec: CHONK, barretenberg.aztec.network/docs/explainers/advanced/chonk
- PlasmaBlind (IACR ePrint 2026/634), eprint.iacr.org/2026/634
- FibRace (arXiv 2510.14693), arxiv.org/pdf/2510.14693
- PayOff (arXiv 2408.06956), arxiv.org/html/2408.06956v1
- Europäische Zentralbank, Fortschrittsbericht digitaler Euro (über unlock-bc.com) und Europäisches Parlament (23.06.2026, über mondovisione.com)
- Privacy Pools (L2BEAT), l2beat.com/privacy/projects/privacy-pools, und Bericht zum Papier mit Vitalik Buterin, nobsbitcoin.com
- Ethereum Magicians: Post-quantum Ethereum und Aggregation (Februar 2026), ethereum-magicians.org
- Hedera: Post-Quantum Cryptography and Blockchain (10.04.2026), hedera.com/blog/post-quantum-cryptography-and-blockchain
- Fireblocks: ML-DSA auf NEAR (2026), fireblocks.com
- eco.com: Guides zu ERC-4337, EIP-7702 und Gas Sponsorship (2026) sowie Candide-Dokumentation zu Paymaster-Richtlinien, docs.candide.dev/instagas/architecture
