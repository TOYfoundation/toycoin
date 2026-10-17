# ToyChain – Technische Spezifikation v0.2

Entwurf vom 16.10.2026, ergänzt am 17.10.2026 · Kryptowährung ToyCoin (TOY) · Status: Entscheidungen vom 16. und 17.10.2026 eingearbeitet, zur Prüfung

## 1. Zusammenfassung

ToyChain ist eine eigenständige Blockchain mit der nativen Währung ToyCoin (Ticker TOY, 18 Dezimalstellen, maximal 100.000.000 TOY). Sie soll auf alten Android-Geräten, Raspberry Pi, Linux-PCs und Windows-Rechnern laufen, in der Größe auf 256 MB begrenzt bleiben und jedem Nutzer erlauben, am Signieren und am Minting teilzunehmen.

Die Kernideen:

- **Schlanke Kette nach Mina-Vorbild:** Die Historie wird in einen rekursiven Beweis gefaltet. Knoten speichern nur den aktuellen Zustand, ein kurzes Block-Fenster und den Beweis.
- **Ethereum-kompatible Konten:** Gleiche Adressen und gleiche 12/24-Wörter-Phrase wie bei Ethereum. Dadurch funktionieren bestehende Wallets, Hardware-Wallets und Tools für Adressen und Signaturen.
- **Teilhabe statt Mining-Farmen:** Proof-of-Stake mit BLS-Sammelsignaturen, damit sehr viele Teilnehmer mitsignieren können, ohne die Kette aufzublähen.
- **Dynamische Emission:** Die Ausschüttung folgt einer Kurve mit Halbwertszeit von rund 3,8 Jahren, die sich an Auslastung und Leistung anpasst und die 25-Jahres-Vorgabe einhält.
- **Spam wird teuer:** Jede Wallet hat ein kostenloses Kontingent. Wer mehr braucht, zahlt progressive Gebühren oder leistet Rechenarbeit (CPU/GPU).
- **Privatsphäre, Offline-Zahlung und Bridge** kommen in späteren Phasen, sind aber von Anfang an im Design vorgesehen. Der Start ist ein Fair Launch mit nur 1 % offengelegter Reserve.

**Grundregel (entschieden am 17.10.2026, Entscheidung 36):** Auf ToyChain gibt es **nur den nativen Coin TOY**. Es gibt dort **keine Token, keine NFTs und keine Smart Contracts**, auch nicht in späteren Phasen. Alles, was die Kette können muss, ist eine **fest eingebaute Transaktionsart** des Protokolls (Abschnitt 6a). Token, NFTs und Verträge gibt es nur auf anderen Ketten, z. B. die Spiel-NFTs und die Bridge-Tresore auf Polygon und Ethereum.

## 2. Ehrliche Einordnung

Damit die Erwartungen stimmen:

- **Sicherheit „wie Bitcoin“** entsteht nicht durch Design allein, sondern durch viele unabhängige Teilnehmer und wirtschaftlichen Einsatz. Eine neue Kette ist anfangs verwundbar. Deshalb gehören Testnet, Audits und ein vorsichtiger Start fest zum Plan.
- **„Eigene Verschlüsselung“** bedeutet hier eine eigene Zusammenstellung geprüfter Verfahren, nicht neu erfundene Algorithmen. Selbst entwickelte Kryptografie ist ein bekanntes Risiko und wird bewusst vermieden (siehe Abschnitt 8).
- **256 MB** lassen sich nur mit Pruning und Beweisen halten. Vollständige Historie gibt es dann nur auf freiwilligen Archiv-Knoten (siehe Abschnitt 4).
- **EVM-Kompatibilität** bedeutet: gleiche Adressen, Signaturen und eine JSON-RPC-Schnittstelle. Eine Ausführung von EVM-Verträgen gibt es auf ToyChain **nie** (Entscheidung 36). dApps, die Verträge brauchen, nutzen TOY auf Polygon oder Ethereum über die Bridge.
- Eine Kryptowährung mit Datenschutzfunktionen und eigener Ausgabe berührt Recht und Regulierung (z. B. MiCA in der EU, Steuer, Börsenlistings). Vor einem Mainnet-Start wird eine rechtliche Prüfung empfohlen.

## 3. Architektur im Überblick

Schichten von unten nach oben:

1. **Netzwerk:** libp2p über QUIC mit Noise-Verschlüsselung, Dandelion++ zur Verschleierung der Absender-IP, mindestens 8 ausgehende Verbindungen gegen Eclipse-Angriffe.
2. **Konsens:** Stake-gewichtetes Komitee (BFT, HotStuff-Stil) mit BLS-Sammelsignaturen und VRF-Auswahl.
3. **Zustand:** Konten-Modell (Adresse, Guthaben, Nonce, Arbeitskredit) in einem Merkle-Baum.
4. **Beweis-Schicht:** Rekursiver, transparenter Beweis (ohne Trusted Setup, z. B. Halo2/IPA oder STARK-basiert) über die gesamte Kettenhistorie.
5. **Schnittstellen:** Eigene API plus Ethereum-kompatibles JSON-RPC (eth_chainId, eth_getBalance, eth_sendRawTransaction u. a.).
6. **Clients:** Ein gemeinsamer Rust-Kern, verpackt als Android-App, Linux/Windows-Programm und Raspberry-Pi-Dienst.

**Knotenrollen** (jede Rolle ist optional zuschaltbar):

- **Light:** Prüft nur Beweis und Zustandswurzel, signiert Transaktionen. Läuft auf alten Android-Geräten (ab ca. 1 GB RAM).
- **Voll:** Hält Zustand und Block-Fenster, validiert alles.
- **Attestierer/Validator:** Signiert Blöcke (BLS), erhält Anteil an der Emission.
- **Prover:** Erzeugt rekursive Beweise, benötigt kräftige CPU/GPU, erhält den größten Arbeitsanteil.
- **Archiv:** Optional, speichert die vollständige Historie, unterliegt nicht der 256-MB-Grenze.

## 4. Das 256-MB-Budget

Die Grenze gilt für die Daten eines Standard-Knotens (Voll-Rolle). Aufteilung:

- **Zustandsbaum (aktive Konten):** 128 MB
- **Block-Fenster (ca. letzte 24 Stunden, nach Bytes begrenzt):** 48 MB
- **Rekursiver Beweis plus Header-Commitments:** 8 MB
- **Privatsphäre-Pool (ab Phase 4, pro Epoche):** 32 MB
- **Indizes, Mempool-Puffer, Reserve:** 40 MB

**Wie die Historie klein bleibt:**

- Alles, was älter als der Finalitäts-Horizont ist (ca. 1.440 Blöcke), wird in den rekursiven Beweis gefaltet und danach gelöscht.
- Ein Block-Header mit Sammelsignatur ist ca. 300 Byte. Ohne Falten wären das über 150 MB pro Jahr. Deshalb müssen auch Header gefaltet werden.
- Wallets speichern ihre eigene Historie lokal und können jede Transaktion mit einem Merkle-Beweis belegen. Explorer und Auswertungen laufen auf Archiv-Knoten.

**Kapazität:** Bei ca. 100 Byte pro Konto (Adresse 20, Guthaben 16, Nonce 8, Arbeitskredit 8, Ablauf-Marker, Baum-Overhead) passen rund 1,3 Mio. aktive Konten in 128 MB. Das ist eine Schätzung und wird im Testnet gemessen. Bis Phase 4 kann der Platz des Privatsphäre-Pools dem Zustand zugeschlagen werden.

**Schutz des Zustands vor Aufblähung:**

- **Mindestguthaben (Existential Deposit, Polkadot-Prinzip):** Startwert 0,1 TOY. Konten darunter werden entfernt.
- **Zustandsablauf (State Expiry):** Konten unter einer Schwelle ohne Aktivität über 1001 Tage (ca. 3 Jahre) wandern in einen Merkle-Akkumulator. Die Wiederbelebung ist unkompliziert: Die Wallet holt den nötigen Beweis automatisch von Archiv-Knoten oder ToyScan, der Nutzer bestätigt nur mit seiner Phrase. Größere Guthaben laufen nie ab.
- **Risiko:** Für die Wiederbelebung braucht man einen Beweis (Witness) von einem Archiv-Knoten oder aus dem eigenen Wallet-Backup. Deshalb veröffentlichen mehrere unabhängige Archiv-Knoten und ToyScan die Ablauf-Snapshots jeder Epoche, und die Wallet warnt den Nutzer rechtzeitig vor dem Ablauf.

## 5. Konsens und Teilnahme am Signieren

**Ablauf pro Block:**

1. Ein Vorschlagender wird per VRF aus dem Komitee gewählt.
2. Das Komitee (Startwert 128 Mitglieder, rotiert pro Epoche) stimmt per BFT ab.
3. Alle Stimmen werden zu **einer** BLS-Signatur (96 Byte) aggregiert. Ab 2/3 gilt der Block als final.

**Alle können teilhaben:**

- Jede Wallet mit Mindest-Stake kann sich als Attestierer eintragen. Zusätzlich gibt es rotierende **Attestierungs-Gruppen** (Sync-Komitee-Prinzip aus Ethereum), in denen auch kleine Teilnehmer regelmäßig mitsignieren und dafür Belohnung erhalten.
- Light-Clients prüfen Sammelsignaturen mit wenigen Millisekunden Aufwand.

**Zeitverhalten:** Ziel-Blockzeit 60 Sekunden. Ist der Transaktionspool vorher voll, wird früher ein Block vorgeschlagen (frühestens nach 10 Sekunden). Finalität nach einem Block, also höchstens ca. 1 Minute.

**Sicherheitsmechanismen:**

- **Slashing:** Doppelsignaturen kosten einen Teil des Stakes.
- **Inaktivitäts-Abzug:** Fehlende Teilnehmer verlieren allmählich Einfluss, damit die Kette bei Ausfällen weiterläuft.
- **Long-Range-Schutz:** Checkpoints (Weak Subjectivity) sowie der rekursive Beweis als Anker.
- **Anti-Zentralisierung:** Der wirksame Stake pro Teilnehmer wird gedeckelt, Belohnungen steigen unterproportional (Wurzel-Gewichtung).

## 6. Transaktionen, Gebühren und Spam-Schutz

**Format:** Kompakte Transaktion (ca. 110 Byte) mit Ethereum-kompatibler Signatur (secp256k1, EIP-155-Chain-ID). Zielwert: Blockgröße höchstens 128 KB, also über 1.000 Transaktionen pro Block. Die Parameter sind dynamisch anpassbar.

**Kostenloses Kontingent:**

- Jede Wallet erhält pro Epoche (ca. 24 Stunden) **Q = 10 bis 100** kostenlose Transaktionen. Je höher die Netzauslastung, desto näher an 10: Q = max(10, 100 × (1 − Auslastung)).
- Die kleine Gebühr zahlt nominell die Wallet, wird aber aus dem Erstattungs-Pool vollständig zurückerstattet.
- Voraussetzung gegen Sybil-Angriffe: Mindestguthaben, Mindestalter des Kontos (Startwert 100 Blöcke) und ein einmaliger kleiner Arbeitsnachweis bei Kontoerstellung.

**Über dem Kontingent:** Jede weitere Transaktion muss durch **Gebühr oder Arbeit** gedeckt werden:

- **Progressive Gebühr:** Pro Block weiterer Transaktionen (Startwert 10) verdoppelt sich die Gebühr dieses Absenders. Dazu kommt eine dynamische Grundgebühr nach EIP-1559-Prinzip, die bei hoher Last steigt.
- **Arbeit statt Gebühr:** Der Absender hängt einen **ToyWork-Share** an (siehe Abschnitt 7) und erhält dafür Rabatt bis zu 100 %.
- **Pflicht für Vielnutzer:** Ab einer hohen Schwelle (Startwert 1.000 Transaktionen pro Epoche) muss mindestens die Hälfte der Überschreitung durch Arbeit gedeckt sein. So unterstützen starke Nutzer das Netzwerk mit CPU/GPU, wie gefordert.

**Verteilung der Gebühren (Startwerte, im Testnet zu kalibrieren):**

- 40 % **Teilnehmer-Lotterie:** Pro Block werden per VRF zufällig aktive Teilnehmer (Attestierer, Arbeiter, aktive Wallets der Epoche) bedacht.
- 40 % **Erstattungs-Pool** für die kostenlosen Transaktionen.
- 20 % Block-Vorschlagender.

Berechtigt für die Lotterie sind nur Teilnehmer, die in der letzten Epoche nachweislich aktiv waren. Das erschwert Manipulation durch viele leere Konten.

## 6a. Fest eingebaute Transaktionsarten (abschließende Liste)

Weil es keine Smart Contracts gibt, kennt das Protokoll nur diese Transaktionsarten. Jede neue Art braucht ein Protokoll-Update, das alle Knoten übernehmen.

| Art | Zweck | ab Phase |
|---|---|---|
| **Überweisung** | TOY von Konto zu Konto, optional mit Hybrid-Signatur | 1 |
| **Stake / Unstake** | TOY für Konsens und Attestierung binden oder lösen | 1 |
| **Stake delegieren** | gebundenes TOY einem Validator zuordnen, ohne ihm das TOY zu geben (z. B. Reserve an das Start-Komitee, Entscheidung 37) | 1 |
| **Validator anmelden / abmelden** | BLS-Schlüssel registrieren | 1 |
| **Attestierung** | Mitsignieren in Komitee oder Attestierungs-Gruppe | 1 |
| **Belohnung und Lotterie** | vom Protokoll selbst ausgelöst, keine Nutzer-Transaktion | 1 |
| **Reserve-Freigabe** | 3 Jahre Sperre, danach Freigabe je Epoche über 3 Jahre auf die offizielle Cold-Wallet, vom Protokoll erzwungen (Entscheidungen 41, 42, 44) | 1 |
| **Mehrfachsignatur-Konto** | Konto, das nur mit m von n Unterschriften bewegt werden kann (z. B. Bug-Bounty-Topf 3 von 5). Ersatz für Multisig-Verträge, die es ohne Smart Contracts nicht gibt | 1 |
| **Vorschlag, Abstimmung, Stimme delegieren** | Mitbestimmung der Community (`09-mitbestimmung.md`) | 1 |
| **Konto wiederbeleben** | abgelaufenes Konto mit Merkle-Beweis zurückholen | 1 |
| **Bridge-Ausgang / Bridge-Eingang** | TOY im protokolleigenen Bridge-Konto sperren oder daraus freigeben | 3 |
| **Abgeschirmter Pool** | Ein- und Auszahlung, private Überweisung | 4 |
| **Offline-Escrow / Scheck einlösen** | Betrag für Offline-Schecks sperren, Schecks einlösen | 5 |

Protokolleigene Konten wie Reserve, Deckungsposten und Bridge-Konto haben **keinen privaten Schlüssel**. Nur die Regeln des Protokolls können sie bewegen.

**Vorteile:** kleinere Angriffsfläche (keine Vertragsfehler wie bei DeFi-Hacks), vorhersehbare Größe der Transaktionen (gut für das 256-MB-Budget), einfachere Beweise in Phase 2 und schnellere Prüfung auf Light-Clients.

## 7. Arbeit: CPU und GPU

Reine Hash-Arbeit verschwendet Energie. Deshalb zwei Stufen:

- **ToyWork (CPU, Arbeit der Nutzer):** Speicherharte Funktion auf Basis von Argon2id mit moderatem Speicherbedarf (Startwert 16 MiB, damit alte Android-Geräte mitmachen). Der Share ist an Absender, Nonce und Epoche gebunden, also nicht wiederverwendbar. Er dient als Spam-Bremse und als Nachweis von Beitrag.
- **ToyProve (GPU/Server, Prover-Rolle):** Beweisaufgaben der Kette, vor allem das Falten der Historie. Diese Arbeit wird tatsächlich gebraucht und wird am höchsten gewichtet.

Prüfung: Shares und Beweise lassen sich viel schneller prüfen als erzeugen. Alles wird vor Anrechnung verifiziert. Fehlerhafte oder doppelte Einreichungen werden verworfen.

## 8. Kryptografie

Grundsatz: **keine selbst erfundenen Algorithmen.** Stattdessen eine eigene, klar versionierte Auswahl geprüfter Verfahren mit **Krypto-Agilität** (Algorithmen lassen sich per Protokoll-Update austauschen).

- **Konten und Adressen:** secp256k1, Adresse = letzte 20 Byte von Keccak-256 des öffentlichen Schlüssels. Wiederherstellung per BIP-39 (12 oder 24 Wörter), Pfad m/44'/60'/0'/0/n wie bei Ethereum.
- **Replay-Schutz:** Jede Signatur enthält die ToyChain-Chain-ID (EIP-155/EIP-712-Domänentrennung). Hinweis: Wer dieselbe Phrase auf Ethereum und ToyChain nutzt, teilt sich den Schlüssel. Deshalb wird in der Wallet ein optionaler eigener Ableitungspfad für ToyChain angeboten.
- **Validatoren:** BLS12-381 mit Sammelsignaturen.
- **Interne Hashes:** BLAKE3 für Merkle-Bäume (schnell auf Raspberry Pi), Keccak-256 nur für Ethereum-Kompatibilität.
- **Post-Quanten-Option:** Freiwillige **Hybrid-Signatur** (secp256k1 plus ML-DSA nach NIST FIPS 204) für hochwertige Konten, ab Phase 1 von Anfang an verfügbar. Die großen Signaturen (ca. 2,4 KB) liegen nur im Block-Fenster, nicht in der gefalteten Historie, damit das Budget hält.
- **Wallet-Schutz auf dem Gerät:** Argon2id plus XChaCha20-Poly1305. Schlüssel nach Möglichkeit in Android Keystore/StrongBox, TPM (Windows) oder Secure Enclave.
- **Mehrere Geräte und Wiederherstellung:** Schwellensignaturen (FROST) für Multi-Geräte-Konten, Social Recovery und Aufteilung der Phrase per SLIP-39 (Shamir).
- **Session-Keys:** Zeitlich und betragsmäßig begrenzte Unterschlüssel, etwa für Apps und Spiele, ohne die Haupt-Phrase zu benutzen.
- **Netzwerk:** Noise-Protokoll über QUIC, Peer-Reputation und Ratenbegrenzung.

## 9. Privatsphäre (Phase 4)

Optionaler **privater Pool** neben den öffentlichen Ethereum-kompatiblen Konten:

- **Stealth-Adressen** (Einmaladressen per ECDH, wie bei Monero und EIP-5564).
- **Abgeschirmter Pool mit zk-Beweisen:** Commitments und Nullifier verhindern Doppelausgabe. Die Anonymitätsmenge ist der gesamte Pool.
- **Verdeckte Beträge:** Beträge liegen als Commitments im Pool und werden im selben zk-Beweis geprüft.
- **Ansichtsschlüssel** für selektive Offenlegung, z. B. gegenüber Steuerberater oder Prüfern.

**Entscheidung: abgeschirmter zk-Pool statt Ringsignaturen.** Ringsignaturen verstecken den Absender nur unter wenigen Lockvögeln (z. B. 16) und brauchen dafür Historie, die diese Kette bewusst nicht dauerhaft speichert. Der abgeschirmte Pool nutzt dieselbe Beweistechnik wie die Kette (ohne Trusted Setup), versteckt Absender, Empfänger und Betrag im ganzen Pool und passt besser in das 256-MB-Budget. Von Monero bleiben Stealth-Adressen, verdeckte Beträge, Ansichtsschlüssel und Dandelion++ erhalten. Damit die Nullifier-Menge begrenzt bleibt, werden ungenutzte Noten pro Epoche automatisch durch die Wallet in den nächsten Pool migriert.

## 10. Emission und dynamisches Halving

**Ziel:** 100.000.000 TOY insgesamt: 1 Mio. Reserve beim Start plus 99 Mio. Emission, davon rund 99 % in etwa 25 Jahren, die Obergrenze darf nie überschritten werden.

**Startverteilung (entschieden am 16.10.2026):** 99 % der 100 Mio. TOY entstehen ausschließlich durch den Emissionsmechanismus. 1 % (1 Mio. TOY) ist eine offengelegte Reserve. Davon decken 10.000 TOY den Polygon-Fremdbestand (Entscheidung 14). **Die übrigen 990.000 TOY gehören dem Ersteller des ToyCoin-Tokens** (`0x18da0be974424d54519c63f62e4b26c0318c00c2`, Entscheidung 41) und liegen im Genesis-Block so:
  - **90.000 TOY Bug-Bounty-Topf**, sofort verfügbar, auf einem offiziellen Mehrfachsignatur-Konto (Entscheidung 43).
  - **900.000 TOY** auf einer **offiziellen, öffentlich bekannten Cold-Wallet** (Entscheidung 44). Dass sie dem Ersteller gehört, belegt eine signierte Nachricht der Ersteller-Adresse. **Sperre:** In den ersten 3 Jahren nach dem Mainnet-Start (1.095 Epochen) wird nichts freigegeben. Danach wird **je Epoche gleichmäßig über 3 Jahre** freigegeben (1.095 Epochen, rund 822 TOY pro Epoche), sodass nach 6 Jahren alles frei ist (Entscheidung 42). Das sind nur etwa 3 bis 5 % der täglich neu entstehenden TOY und damit fair für alle, die später dazukommen. Die Sperre erzwingt das Protokoll. Es gibt keinen Presale und keinen weiteren Vorab-Anteil. Das ist kein reiner Fair Launch mehr, sondern ein Fair Launch mit kleiner, transparenter Reserve. Ob das rechtlich als ausreichend dezentral gilt, muss vorab juristisch geprüft werden.

**Basiskurve:** Pro Epoche (ca. 24 Stunden) werden k × Restmenge ausgeschüttet.

- k₀ ≈ 0,0505 % pro Epoche. Daraus folgt eine Halbwertszeit von ca. 3,76 Jahren und ca. 49.995 TOY in der ersten Epoche (99 Mio. × 0,0505 %, korrigiert am 17.10.2026, vorher irrtümlich 49.950).
- **Epoche nach Blöcken (entschieden am 17.10.2026):** Eine Epoche endet nach 1.440 Blöcken, nicht nach 24 Stunden. Kommen Blöcke bei vollem Pool schneller, ist die Epoche kürzer. Der Zeitplan-Regler gleicht das über m aus. Für den 25-Jahres-Plan gilt ein Jahr = 365 Epochen.
- Im ersten Jahr werden bei Faktor 1 rund 16,6 Mio. TOY ausgeschüttet, nach 25 Jahren bleiben ca. 1 % (1 Mio.) übrig, die danach asymptotisch auslaufen.
- Weil die Ausschüttung immer nur ein Bruchteil des Rests ist, wird die Obergrenze von 100 Mio. **mathematisch nie überschritten**.

**Dynamischer Faktor m:** k = k₀ × m, mit m zwischen 0,5 und 2,0:

- **Auslastung:** Hohe, **bezahlte oder durch Arbeit gedeckte** Nutzung erhöht m. Kostenlose Massen-Transaktionen und Spam zählen nicht, sonst ließe sich die Emission durch Spam hochtreiben.
- **Leistung:** Hohe Beteiligung (Attestierungen, geprüfte Arbeit) erhöht m, geringe Beteiligung senkt es.
- **Zeitplan-Regler:** Liegt die bisherige Ausschüttung hinter dem 25-Jahres-Plan, steigt m leicht, liegt sie davor, sinkt m. Damit bleibt der Zeitrahmen verlässlich.

**Aufteilung der Emission (Startwerte):**

- 50 % Konsens-Signaturen (gewichtet nach Stake, Verfügbarkeit und Reaktionszeit)
- 40 % Arbeit (ToyWork und ToyProve, nach geprüften Shares)
- 10 % Erstattungs-Pool für kostenlose Transaktionen

**Leistungsbelohnung:** Starke und zuverlässige Teilnehmer erhalten einen Bonus bis zu 1,5 ×, begrenzt durch den Wurzel-Deckel aus Abschnitt 5, damit Großteilnehmer nicht dominieren.

## 11. Offline-Transaktionen (Phase 5)

Das Doppelausgabe-Problem wird durch ein **vorab hinterlegtes Offline-Budget** gelöst:

1. Der Nutzer sperrt online einen Betrag (Escrow) für einen gerätegebundenen Schlüssel. Das ist eine native Protokollfunktion, kein benutzerdefinierter Vertrag.
2. Offline signiert das Gerät **Schecks** über Teilbeträge. Übertragung per NFC, Bluetooth oder QR-Code.
3. Der Empfänger prüft Signatur und den Merkle-Beweis des Escrows, auch ohne Netz.
4. Sobald online, wird eingelöst. Da der Betrag gesperrt ist, kann der Aussteller nicht mehr ausgeben, als hinterlegt wurde.

Sicherheitsregeln: Ablaufzeit je Scheck, Höchstbetrag je Scheck, Gerätebindung über Secure Element oder Keystore. Auf Geräten ohne Hardware-Schutz gelten niedrigere Limits.

## 12. Ethereum-Kompatibilität und Bridge (Phase 3)

**Auf ToyChain:** Ethereum-Adressen, Signaturformat und JSON-RPC-Teilmenge. MetaMask und ähnliche Wallets können ToyChain als eigenes Netzwerk einbinden und TOY senden. Smart Contracts gibt es nicht (Entscheidung 36).

**Verträge nur auf den anderen Ketten:** Tresor-Verträge und der NFT-Vertrag liegen auf Polygon und Ethereum. Auf ToyChain ist die Bridge eine fest eingebaute Transaktionsart mit einem protokolleigenen Bridge-Konto ohne Schlüssel (Abschnitt 6a).

**Bridge (entschieden: das Sicherste):** TOY wird 1:1 auf Polygon, Ethereum und weitere Ketten gebridgt. Weil auf Polygon und Ethereum schon je 100 Mio. ToyCoin existieren, wird nichts neu geprägt: Die Bridge arbeitet mit Tresor-Verträgen (Vorschlag, zur Bestätigung).

- **Altbestand:** Auf Polygon und Ethereum existieren schon je 100 Mio. ToyCoin, zusammen 200 Mio. und damit doppelt so viel wie die Obergrenze. Was in der Contract-Wallet liegt (bis zu 99 Mio. je Kette), wird in einen Tresor-Vertrag gesperrt. Der Tresor gibt Token nur frei gegen einen verifizierten ToyChain-Beweis, 1:1.
- **Fremdbestand (entschieden am 17.10.2026, ersetzt die Regel vom 16.10.):** Vor dem Start werden die Token außerhalb der Contract-Wallet bis auf höchstens **10.000 TOY je Kette** an die Contract-Wallet zurückübertragen (Befund und Adressen in `04-toyscan-und-altbestand.md`, Abschnitt 4). Fremdbestand erzeugt nie neues TOY.
- **Deckungsposten (entschieden am 17.10.2026):** ERC-20-Token tragen keine Herkunft. Eine Regel „Fremdbestand nicht einlösbar“ lässt sich deshalb technisch nicht durchsetzen: Wer alte Token in den Tresor einzahlt, würde TOY abziehen, die für andere Bridge-Nutzer gesperrt sind. Darum wird der verbleibende Fremdbestand **im Genesis-Block mit nativen TOY gedeckt**: 10.000 TOY aus der Reserve liegen als Deckungsposten für Polygon auf einem Bridge-Konto ohne Schlüssel (Ethereum: 0, dort gibt es keinen Fremdbestand). Damit ist **jeder** bestehende ToyCoin-Token von Anfang an 1:1 einlösbar, auch der von Altbesitzern. Die Reserve für Team, Entwickler und Bug-Bounty beträgt dann 990.000 TOY.
- **Supply-Invariante:** Die Gesamtmenge TOY in Umlauf, gleich auf welcher Kette, ist nie größer als gemintetes TOY plus 1 Mio. Reserve, und das ist nie mehr als 100.000.000. Die Tresore halten mehr, als je freigegeben werden darf. Der Rest bleibt dauerhaft gesperrt.
- **Tresor-Regeln:** Kein Admin-Schlüssel, keine Mint-Funktion, verifizierter Quellcode, keine manuelle Entsperrung. Bei den Altbestand-Token-Verträgen hat die Adresse, die ToyCoin auf Polygon erstellt hat, im Code keine Sonderrechte (siehe Abschnitt 16). Laut den Verträgen lässt sich ToyCoin auf keiner Kette nachprägen, und es gibt je Kette nie mehr als 100.000.000. ToyScan prüft das fortlaufend am Vertrag selbst (Gesamtmenge, Mint-Funktion, Upgrade-Mechanismus, Admin-Rechte).
- **Prüfung durch Beweis:** Der Tresor auf der Zielkette prüft den rekursiven ToyChain-Beweis selbst (zk-Light-Client). Zusätzlich muss das Komitee per Schwellensignatur zustimmen (zwei Sicherungen), bis der Verifizierer lange genug bewährt ist.
- **Schutzschichten:** Limits pro Zeitraum und Kette, Zeitverzögerung bei großen Transfers, Notschalter, der nur pausieren kann und nie freigibt.
- **ToyScan-Ampel:** Grün, wenn live alle Kriterien erfüllt sind: (1) Umlauf ≤ gemintet + Reserve ≤ 100 Mio., (2) Tresore halten mindestens den nicht freigegebenen Rest, (3) Reserve: 900.000 TOY auf der offiziellen Cold-Wallet mit intakter Sperre, 90.000 TOY Bug-Bounty-Topf, 10.000 TOY Deckung (Entscheidungen 41 bis 44), (4) Emission nicht vor dem Zeitplan, (5) Altbestand-Verträge ohne Mint-Funktion und ohne Upgrade-Mechanismus, Gesamtmenge je Kette ≤ 100 Mio., Tresor ohne Admin-Schlüssel, (6) Notschalter nicht ausgelöst und Beweis-Verifizierer aktuell, (7) Fremdbestand je Kette ausgewiesen und nicht größer als der Deckungsposten dieser Kette (Polygon 10.000 TOY, Ethereum 0). Gelb bei Abweichung, rot bei Verletzung. Wird auf irgendeiner Kette mehr als 100.000.000 ToyCoin festgestellt oder ließe sich nachprägen, zeigt ToyScan eine große rote Lampe mit Warnung, und der Notschalter der Bridge kann Freigaben pausieren. Die Regeln sind offen, damit jeder nachrechnen kann. Die Ampel ersetzt keine eigene Prüfung.
- **Technisch offen:** Die Beweisprüfung auf Ethereum ist teuer. Der Beweis wird dafür in eine EVM-freundliche Form umhüllt. Die Wahl des Verfahrens (Trusted-Setup-Frage) wird in Phase 2 geklärt.

## 12a. Mitbestimmung

Die Community stimmt über Parameter und Protokoll-Updates ab (Entscheidung 45). Stimmen sind fest eingebaute Transaktionsarten. Eine **Verfassung** aus unveränderbaren Regeln (Obergrenze, Emissionsformel, Reserve, nur nativer Coin, Bridge-Deckung, kein Zugriff auf fremde Guthaben) kann keine Abstimmung ändern. Details: `09-mitbestimmung.md`.

## 13. Bedrohungsmodell

- **Sybil und Spam:** Mindestguthaben, Kontingente, Arbeitsnachweise, progressive Gebühren.
- **51-%-Angriff:** Stake-Deckel pro Teilnehmer, Slashing, Checkpoints, hohe Teilnehmerzahl.
- **Long-Range:** Weak-Subjectivity-Checkpoints, Beweis-Anker.
- **Eclipse und DoS:** Peer-Vielfalt, Ratenbegrenzung, Dandelion++.
- **Schlüsseldiebstahl:** Hardware-Keystore, Hybrid-Signatur, Multi-Geräte-Schwellenschlüssel.
- **Replay:** Chain-ID in jeder Signatur.
- **Fehler im Code:** Fuzzing, Property-Tests, reproduzierbare Builds, signierte Releases, externe Audits, Bug-Bounty.
- **Zentralisierung durch Prover:** Mehrere unabhängige Prover, offene Software, Belohnung für Verfügbarkeit.

## 14. Parameter-Startwerte

- Ticker TOY, 18 Dezimalstellen, Gesamtmenge 10²⁶ kleinste Einheiten (passt in 128-Bit-Ganzzahlen)
- Blockzeit 60 s, frühester Block bei vollem Pool nach 10 s, Epoche 1.440 Blöcke
- Maximale Blockgröße 128 KB, Komitee 128, Finalität nach 1 Block
- Kostenloses Kontingent 10 bis 100 pro Epoche, Vielnutzer-Schwelle 1.000 pro Epoche
- Mindestguthaben 0,1 TOY, Kontoablauf nach 1001 Tagen (nur unterhalb einer Schwelle)
- Emission k₀ ≈ 0,0505 % pro Epoche, Faktor m zwischen 0,5 und 2,0
- ToyWork: Argon2id, 16 MiB

Alle Werte sind Startannahmen und werden im Testnet gemessen und angepasst.

## 15. Fahrplan

1. **Phase 0, Spezifikation:** Dieses Dokument, Parameter festlegen, Entscheidungen aus Abschnitt 16 treffen.
2. **Phase 1, Kern:** Rust-Knoten mit Konten, Konsens, 60-Sekunden-Blöcken, Emission, Kontingent und Gebühren. Wallet für Android, Linux, Windows und Raspberry Pi. Ethereum-kompatible Adressen und JSON-RPC. Öffentliches Testnet.
3. **Phase 2, Beweise:** Rekursives Falten der Historie, Prover-Rolle, Einhaltung des 256-MB-Budgets, Light-Clients.
4. **Phase 3, Bridge** (vorgezogen, Entscheidung 39; ohne EVM-Ausführung, Entscheidung 36). Danach startet die NFT-Prägung mit TOY.
5. **Phase 4, Privatsphäre:** Stealth-Adressen, privater Pool.
6. **Phase 5, Offline-Schecks.**

**Mainnet-Zeitpunkt (Entscheidung 38):** Mainnet nach Phase 1 und externem Audit. Neue Knoten starten bis zur Beweis-Schicht von veröffentlichten Checkpoints. Phase 2 muss innerhalb von etwa 12 Monaten nach dem Mainnet folgen, weil die Block-Header sonst das 256-MB-Budget sprengen (ca. 158 MB pro Jahr).

**Pflicht-Freigaben:** Kein Mainnet ohne externes Audit der Kryptografie und des Konsenses, ohne Testnet-Phase mit Lasttests und ohne rechtliche Prüfung.

## 16. Entscheidungen und offene Punkte

**Entschieden am 16.10.2026:**

1. **Zustandsablauf:** Ja, nach 1001 Tagen (ca. 3 Jahre), mit unkomplizierter Wiederbelebung.
2. **Privater Pool:** Abgeschirmter zk-Pool (sicherer und kompatibler), siehe Abschnitt 9.
3. **Startverteilung** (Empfänger und Sperre geändert durch Entscheidung 41): Fair Launch mit 1 % offengelegter Reserve (1 Mio. TOY für Team, Entwickler und Bug-Bounty, höchstens 100 Wallets), sonst keine Vorab-Menge. Die Reserve entsteht im Genesis-Block mit Zeitsperre, der gesamte Altbestand bleibt gesperrt.
4. **Bridge:** 1:1 zu Polygon, Ethereum und weiteren Ketten. Die vorhandenen 2 × 100 Mio. Altbestand-Token werden zu gesperrten Reserven im Tresor-Modell. Anzeige und Ampel in ToyScan. Zum Fremdbestand gilt die Neuregelung vom 17.10.2026 (Abschnitt 12): Rückübertragung bis auf höchstens 10.000 TOY je Kette, Rest im Genesis-Block gedeckt.
5. **Bridge-Typ:** zk-Light-Client mit Komitee als zweiter Sicherung.
6. **Hybrid-Signaturen:** Ab Phase 1 optional verfügbar.

**Entschieden am 17.10.2026:**

7. **Erste Epoche:** k₀ = 0,0505 % gilt, die erste Epoche bringt damit ca. 49.995 TOY (Zahl korrigiert).
8. **Zeitsperre der Reserve:** ersetzt durch Entscheidung 41: 3 Jahre gesperrt, danach schrittweise Freigabe an die Ersteller-Adresse.
9. **Epochenlänge:** Nach Blöcken (1.440), der Zeitplan-Regler gleicht schnellere Epochen aus.
10. **Lizenz:** Der Code steht unter der MIT-Lizenz.
11. **Fremdbestand:** Rückübertragung an die Contract-Wallet bis auf höchstens 10.000 TOY je Kette vor dem Start (ersetzt die Sperr-Regel aus Nr. 4).
12. **Deckungsposten:** 10.000 TOY aus der Reserve decken im Genesis-Block den Polygon-Fremdbestand. Jeder bestehende Token ist 1:1 einlösbar. Reserve für Team, Entwickler und Bug-Bounty: 990.000 TOY.
13. **Nur nativer Coin:** Auf ToyChain gibt es nur TOY, keine Token, keine NFTs, keine Smart Contracts, auch später nicht (Entscheidung 36).

**Noch offen:**

- **Altbestand-Verträge:** Beide Verträge (Ethereum, Chain-ID 1, und Polygon, Chain-ID 137) wurden anhand der Funktionsliste im Bytecode geprüft: Standard-ERC-20 mit Permit, ohne Mint-, Burn-, Owner-, Pause-, Sperr- oder Upgrade-Funktion und kein Proxy. Noch offen sind das Ablesen der Gesamtmenge je Kette und die Quellcode-Verifizierung auf Etherscan und Polygonscan. Die Ersteller-Adresse auf Polygon hat im Code keine Admin-Rolle, sie hält nur den Bestand. Der Tresor-Vertrag selbst bekommt ebenfalls keine Admin-Rechte.
- **Rechtliche Prüfung** vor dem Mainnet (z. B. MiCA, Steuer, Börsenlistings), auch zur 1 % Reserve und zum Fremdbestand.
