# TOYPAY im Gesamtsystem

Entwurf v0.2 vom 17.10.2026 · Grundlage: TOYPAY-Unterlagen Stand Runde 8 (im Projekt unter `toypay/docs/`) · Status: **zur Prüfung, mehrere Grundsatzentscheidungen offen (Abschnitt 7)**

Dieses Dokument beschreibt, wie die TOYPAY-App mit ToyChain zusammenspielt. Die Spielinhalte (TOYQUEST 3D, Grafik, Erfindungen, Sprachen) sind nicht Teil des ToyChain-Konzepts und werden hier nur erwähnt, wo sie die Kette berühren.

## 1. Ist-Stand laut Unterlagen

| Teil | Stand |
|---|---|
| **TOYPAY-App** | Android, Kotlin. Enthält die „Spielwährung ToyCoin“, Stadtviertel, **Poker** und das 2D-Abenteuer TOYQUEST. 28 Sprachen. |
| **TOYQUEST 3D** | Eigenes Spiel (Godot 4.4), Paket `app.toypay.quest3d`, Exporte für Android arm64, Windows, Linux. Spielgeld **Dukaten**, im Spiel gibt es eine **Wechselstube**. 45 Sprachen. |
| **TOYPAY-Server** | Spielserver mit Online-Modus und Weltdaten. |
| **Blockchain** | `local.properties` enthält einen **Alchemy-Key** (Zugang zu Ethereum/Polygon), Seed-Phrasen werden erwähnt. **Bestätigt am 17.10.2026:** Das Spiel nutzt die ToyCoin-Token auf **Polygon**, um **NFTs** zu prägen (Spielgegenstände und Spielfiguren). |

**Noch nicht beschrieben** sind: ob Dukaten in ToyCoin getauscht werden können, und ob es schon einen NFT-Vertrag auf Polygon gibt.

## 2. Zielbild

TOYPAY wird die **Haupt-App für den nativen TOY**:

1. **Wallet für TOY auf allen Ketten (entschieden am 17.10.2026):** Die App nimmt ToyCoin auf **jeder** Kette an, auf der es ihn gibt: nativ auf ToyChain sowie als Token auf Polygon und Ethereum (später weitere). Sie zeigt den Bestand je Kette und in Summe und bridgt nativen TOY zu den anderen Ketten und zurück.
2. **Light-Knoten:** Die App prüft die Kette selbst (Sammelsignaturen, später den rekursiven Beweis) und muss keinem Server vertrauen.
3. **Teilhabe:** Optional mitsignieren (Attestierungs-Gruppen) und ToyWork-Arbeit leisten, wenn das Gerät lädt und im WLAN ist.
4. **Spiele:** Spielgegenstände und Spielfiguren können als **NFTs auf Polygon** geprägt werden, bezahlt mit Polygon-TOY (Abschnitt 5a). Sonst ist die Verbindung der Spiele zu TOY bewusst eng begrenzt (Abschnitt 7).

## 3. Technische Anbindung

- **Ein gemeinsamer Rust-Kern.** Die Crates `toy-crypto`, `toy-tx` und `toy-wallet` sowie ein Light-Client werden als Android-Bibliothek gebaut (arm64-v8a, zusätzlich armeabi-v7a für alte Geräte). Die Kotlin-App ruft sie über **UniFFI** auf, das Werkzeug von Mozilla, das aus Rust automatisch Kotlin-Anbindungen erzeugt. Kryptografie wird **nicht** in Kotlin neu geschrieben. Derselbe Kern läuft auf Linux, Windows und Raspberry Pi.
- **Sanfter Umstieg dank Ethereum-Kompatibilität.** ToyChain bietet ab M7 eine Ethereum-kompatible Schnittstelle (`eth_chainId`, `eth_getBalance`, `eth_sendRawTransaction` …). Spricht die App heute über Alchemy mit Polygon, kann sie in einem ersten Schritt dieselben Aufrufe an einen ToyChain-Knoten richten: andere Adresse, andere Chain-ID. Danach übernimmt der eingebaute Light-Knoten.
- **Das 3D-Spiel hält keine Schlüssel.** TOYQUEST 3D und der Spielserver bekommen nie Zugriff auf Phrase oder private Schlüssel.
- **API-Schlüssel gehören nicht in die App.** Der Alchemy-Key darf nicht in der ausgelieferten App stecken, weil jeder ihn aus der APK auslesen kann. Spätestens mit dem Umstieg auf ToyChain entfällt er.

## 4. Knotenrollen in der App

| Rolle | In TOYPAY | Begründung |
|---|---|---|
| Light | **immer an** | wenige MB, wenig Akku, prüft Zustandswurzel und Sammelsignatur |
| Attestierungs-Gruppe (mitsignieren) | **optional**, nur beim Laden und im WLAN | bringt Belohnung. Wer offline geht, verliert nur diese Runde, keinen Stake |
| ToyWork-Shares (Arbeit) | **optional**, nur beim Laden | Argon2id mit 16 MiB läuft auch auf alten Geräten |
| Voll-Knoten (256 MB) | **aus**, auf Tablets einschaltbar | braucht Speicher und eine dauerhafte Verbindung |
| Validator im Komitee | **nein** | muss rund um die Uhr erreichbar sein. Android beendet Hintergrund-Apps, und Ausfälle kosten Einfluss. Dafür eignen sich Raspberry Pi oder PC |

**Schutz vor Sybil-Angriffen:** Belohnungen gibt es nur für Stake und geprüfte Arbeit, nie pro Gerät oder Installation. Sonst könnte jemand tausend Emulatoren starten.

## 5. Zahlungen

- **Unter Personen:** Adresse oder QR-Code, Endgültigkeit nach höchstens ca. 1 Minute. Der QR-Code folgt dem Ethereum-Format (EIP-681: `ethereum:<adresse>@<chain-id>?value=…`), dadurch können ihn auch andere Wallets lesen.
- **Im Laden:** Eine Minute an der Kasse ist spürbar lang. Möglichkeiten: Anzeige „unterwegs, wird bestätigt“ bei kleinen Beträgen, oder die Offline-Schecks (Spezifikation Abschnitt 11) von Phase 4 vorziehen. **Entscheidung offen.**
- **Kontingent:** Die App zeigt die verbleibenden kostenlosen Transaktionen der Epoche an und warnt vor der Gebühr.

## 5a. Mehrere Ketten und Spiel-NFTs

**Mehrere Ketten in einer Wallet**
- Ein Schlüssel, eine Adresse: Dank der Ethereum-kompatiblen Adressen hat der Nutzer auf ToyChain, Polygon und Ethereum **dieselbe** Adresse aus derselben Phrase.
- **Gas-Problem:** Wer TOY auf Polygon oder Ethereum bewegt, braucht dort zusätzlich POL bzw. ETH für die Gebühr. Das verwirrt Einsteiger. **Lösung:** Beide Token-Verträge können **Permit (EIP-2612)**. Der Nutzer unterschreibt nur, ein Relayer des Projekts reicht die Transaktion ein und zahlt das Gas. Die Kosten können in TOY verrechnet werden.
- **Zugang zu Polygon und Ethereum:** Für den ToyChain-Teil reicht der eingebaute Light-Knoten. Für Polygon und Ethereum braucht die App einen RPC-Anbieter. Der Alchemy-Key darf dafür nicht in der App stecken. Stattdessen läuft der Zugang über einen kleinen Proxy des Projekts, oder der Nutzer wählt öffentliche bzw. eigene RPC-Adressen.
- **Anzeige:** „TOY gesamt“ ist die Summe über alle Ketten. Für jede Kette wird angezeigt, ob der Bestand aus dem Tresor stammt und damit 1:1 einlösbar ist. Laut Entscheidung 14 gilt das für alle Token.

**Spiel-NFTs auf Polygon**
- **Wo:** Die NFTs liegen auf **Polygon**, nicht auf ToyChain. Damit bleibt die Vorgabe „keine NFTs in Phase 1“ für ToyChain bestehen, und die NFTs sind sofort in bestehenden Wallets und Marktplätzen sichtbar.
- **Vertrag:** Dafür braucht es einen **neuen NFT-Vertrag** (ERC-721 oder ERC-1155) auf Polygon. Anders als der Token-Vertrag hat er zwangsläufig eine Präge-Funktion. Sie muss streng begrenzt sein:
  - Prägen nur mit einer Freigabe des Spielservers (signierter Gutschein nach EIP-712: „Gegenstand X gehört Spieler Y“).
  - Jeder Gutschein ist nur einmal gültig.
  - Bezahlt wird in TOY per Permit, also ohne gesonderte Freigabe.
  - Kein Admin, der beliebig prägen kann.
- **Spiel und Kette synchron halten:** Ist ein Gegenstand als NFT geprägt, gilt **das NFT** als Eigentumsnachweis. Wird das NFT verkauft, wandert der Gegenstand im Spiel zum neuen Besitzer. Der Spielserver liest dazu die Kette. Ein geprägter Gegenstand darf im Spiel nicht zusätzlich weiter gehandelt werden, sonst gäbe es ihn doppelt.
- **Schlüssel des Spielservers:** Der Server signiert die Gutscheine und ist damit ein lohnendes Angriffsziel. Deshalb: eigener Schlüssel nur für Gutscheine (nie für TOY), Tageslimits im Vertrag und ein Notschalter, der das Prägen pausieren kann.
- **Wohin die TOY für das Prägen fließen:** ist offen (Abschnitt 9). Die Wahl wirkt auf die ToyScan-Rechnung, siehe K8.

## 6. Schlüssel und Verwahrung

- **Nicht verwahrend:** Die Schlüssel liegen nur auf dem Gerät (Android Keystore, wenn vorhanden StrongBox), gesichert mit Argon2id und XChaCha20-Poly1305. Weder der Server noch das Projekt kann auf Guthaben zugreifen. Das ist sicherer und rechtlich in der Regel einfacher als eine verwahrende App (siehe Abschnitt 7, K5).
- Phrase-Sicherung beim Einrichten, wahlweise eigener ToyChain-Ableitungspfad (Spezifikation Abschnitt 8).
- Schlüssel und Phrasen erscheinen nie in Logs, Absturzberichten oder Server-Anfragen.

## 7. Kritische Punkte, die vor dem Bau entschieden werden müssen

Hinweis: Das ist keine Rechtsberatung. Die Punkte zeigen, wo eine rechtliche Prüfung nötig ist.

**K1 Spielwährung oder Kryptowährung.** In TOYPAY heißt ToyCoin „Spielwährung“. Der native TOY wird aber 1:1 mit den ERC-20-Token auf Polygon und Ethereum verbunden. Auf Polygon gab es bereits einen Handelspool. Damit hat TOY einen echten Geldwert und ist rechtlich keine reine Spielwährung mehr. Bezeichnung, App-Store-Angaben und Nutzungsbedingungen müssen das widerspiegeln.

**K2 Poker.** Poker mit TOY als Einsatz wäre Echtgeld-Glücksspiel. In Deutschland ist Online-Glücksspiel nur mit deutscher Erlaubnis zulässig, und erlaubte Anbieter dürfen Kryptowährungen nicht als Zahlungsmittel annehmen. Google Play verlangt für Krypto-Funktionen zertifizierte Dienste in regulierten Ländern und verbietet ausdrücklich, Krypto-Werte als Einsatz für Gewinne mit Geldwert zu nutzen. **Vorschlag:** Poker nur mit Spielchips, die nie in TOY getauscht und nie mit TOY gekauft werden können. Oder Poker ganz aus der App nehmen, in der TOY liegt.

**K3 Dukaten und Wechselstube.** Wenn Spiel-Belohnungen in TOY getauscht werden können („Play-to-Earn“), gilt:
- Das Spiel darf **nie neues TOY erzeugen.** Die Obergrenze von 100 Mio. hängt allein an der Emission auf der Kette.
- TOY für Belohnungen müsste aus einem vorab gefüllten, öffentlich sichtbaren Spiel-Topf kommen. Der Server bräuchte dafür einen Schlüssel, also eine Hot-Wallet, die angreifbar ist.
- Das berührt Glücksspiel-, Jugendschutz- und Steuerrecht.

**Vorschlag:** Zum Start **kein** Umtausch zwischen Dukaten und TOY.

**K4 Jugendschutz.** Für welches Alter sind die Spiele gedacht? Spielen Minderjährige, brauchen alle TOY-Funktionen eine Altersgrenze, oder Spiel und Wallet werden getrennte Apps.

**K5 Verwahrung.** Hält TOYPAY nie fremde Schlüssel, ist die App eine Software-Wallet. Wer Guthaben für Nutzer verwahrt oder tauscht, braucht nach MiCA voraussichtlich eine Zulassung als Krypto-Dienstleister. **Vorschlag:** strikt nicht verwahrend.

**K6 Datenschutz.** Der Spielserver kennt Nutzerkonten. Verknüpft er sie mit TOY-Adressen, entstehen personenbezogene Finanzdaten (DSGVO). **Vorschlag:** Der Server speichert keine Wallet-Adressen, oder nur mit ausdrücklicher Einwilligung.

**K8 NFT-Prägung vor dem Start von ToyChain berührt den Fremdbestand.** Wer im Spiel NFTs prägen will, braucht TOY auf Polygon. Vor dem Start von ToyChain und der Bridge können diese TOY nur aus der Ersteller-Adresse kommen. Jeder TOY, der von dort an Spieler geht, **erhöht den Fremdbestand**. Laut Entscheidung 13 und 14 darf er beim Start aber höchstens 10.000 TOY betragen, gedeckt aus der Reserve. Möglichkeiten:
- (a) Die NFT-Prägung mit TOY startet erst **nach** dem Start der Bridge. Dann kommen alle Polygon-TOY aus dem Tresor und sind gedeckt.
- (b) Die TOY, mit denen geprägt wird, fließen an die Ersteller-Adresse zurück. Der Fremdbestand bleibt dann ungefähr gleich, muss aber zum Start trotzdem ≤ 10.000 sein.
- (c) Die Deckung im Genesis-Block wird auf den tatsächlichen Fremdbestand am Stichtag erhöht. Das geht zulasten der Reserve (990.000 TOY), und der Fremdbestand darf diese nie übersteigen.

**Vorschlag:** (a), oder (b) mit einem festen Stichtag vor dem Genesis-Block.

**K9 Zufalls-NFTs (Lootboxen).** NFTs mit zufälligem Wert gegen TOY zu verkaufen, gilt nach den Google-Play-Regeln als Glücksspiel, wenn die App nicht als Glücksspiel-App zugelassen ist. Außerdem dürfen NFTs nicht als Einsatz für Gewinne mit Geldwert dienen. Das betrifft auch Poker (K2). **Vorschlag:** Ein NFT bildet immer genau einen bekannten Gegenstand ab, ohne Zufall beim Kauf.

**K10 NFTs und Recht.** Einzigartige NFTs fallen in der Regel nicht unter MiCA. Große Serien gleichartiger NFTs können aber anders bewertet werden. Gehandelte Spielgegenstände berühren zudem Steuer- und Verbraucherrecht. Teil der rechtlichen Prüfung.

**K7 Eine App oder zwei.** Wallet mit Light-Knoten und Spiele in einer App machen die App groß und vermischen Risiken (Glücksspiel, Jugendschutz, Finanzfunktionen). **Möglichkeit:** „TOYPAY Wallet“ als schlanke eigene App mit Knoten, die Spiele als eigene Apps (TOYQUEST 3D ist es bereits).

## 8. Abhängigkeiten zum ToyChain-Bauplan

| TOYPAY braucht | kommt aus |
|---|---|
| Adressen, Phrase, Signaturen | M2 (`toy-crypto`) |
| Transaktionen, Kontingent | M3 (`toy-tx`) |
| Ethereum-kompatible Schnittstelle für den Umstieg | M7 (`toy-rpc`) |
| Wallet-Kern, Keystore | M8 (`toy-wallet`) |
| Android-Bibliothek mit Kotlin-Anbindung | M9 (Plattformen) |
| Offline-Zahlung im Laden | Phase 4 (oder vorgezogen, siehe Abschnitt 5) |
| Bridge ToyChain ↔ Polygon/Ethereum in der App | Phase 5 (Bridge), Konzept in `06-bridge.md` |
| NFT-Vertrag auf Polygon mit Gutschein-Prägung | eigener Baustein außerhalb von ToyChain, eigenes Audit |

## 9. Fragen an den Projektinhaber

Beantwortet am 17.10.2026: Die App nimmt TOY auf allen Ketten an, bridgt nativen TOY und prägt NFTs auf Polygon mit Polygon-TOY (Entscheidungen 15 und 16).

Offen:
1. Kann man Dukaten oder Poker-Gewinne in ToyCoin tauschen (K3)?
2. Für welches Alter sind TOYPAY und TOYQUEST gedacht (K4)?
3. Eine App mit allem, oder Wallet und Spiele getrennt (K7)?
4. Nur Android, oder später auch iPhone?
5. Wann soll die NFT-Prägung mit TOY starten: vor oder nach dem Start von ToyChain und der Bridge (K8)?
6. Wohin fließen die TOY, mit denen geprägt wird: an das Projekt, an einen Spiel-Topf oder an eine Burn-Adresse, also dauerhaft aus dem Umlauf?
7. Gibt es schon einen NFT-Vertrag auf Polygon? Wenn ja, welche Adresse?
