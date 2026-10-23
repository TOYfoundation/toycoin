# TOYPAY im Gesamtsystem

Entwurf v0.2 vom 17.10.2026 · Grundlage: TOYPAY-Unterlagen Stand Runde 8 (im Projekt unter `toypay/docs/`) · Status: **zur Prüfung, mehrere Grundsatzentscheidungen offen (Abschnitt 7)**

> **Hinweis vom 24.10.2026 (Entscheidung 121):** Präge-Gebühr, Gas und Gebühren-Kreislauf (Nr. 23, 24, 26) sind ersetzt durch `13-toyquest-items-nft.md` (ERC-1155, Gas-Formel, POL-Topf). Welche der übrigen Präge-Regeln weiter gelten, steht dort in Abschnitt 7. Mindestbestand und Sperre (Nr. 19, 21) sind neu zu prüfen. Die Bridge-Teile hier sind durch Nr. 63 überholt.

Dieses Dokument beschreibt, wie die TOYPAY-App mit ToyChain zusammenspielt. Die Spielinhalte (TOYQUEST 3D, Grafik, Erfindungen, Sprachen) sind nicht Teil des ToyChain-Konzepts und werden hier nur erwähnt, wo sie die Kette berühren.

## 1. Ist-Stand laut Unterlagen

| Teil | Stand |
|---|---|
| **TOYPAY-App** | Android, Kotlin (künftig auch Windows, Linux, Raspberry Pi, Entscheidung 30). Enthält die „Spielwährung ToyCoin“, Stadtviertel, **Poker** und das 2D-Abenteuer TOYQUEST. 28 Sprachen. |
| **TOYQUEST 3D** | Eigenes Spiel (Godot 4.4), Paket `app.toypay.quest3d`, Exporte für Android arm64, Windows, Linux. Spielgeld **Dukaten**, im Spiel gibt es eine **Wechselstube**. 45 Sprachen. |
| **TOYPAY-Server** | Spielserver mit Online-Modus und Weltdaten. |
| **Blockchain** | `local.properties` enthält einen **Alchemy-Key** (Zugang zu Ethereum/Polygon), Seed-Phrasen werden erwähnt. **Bestätigt am 17.10.2026:** Das Spiel nutzt die ToyCoin-Token auf **Polygon**, um **NFTs** zu prägen (Spielgegenstände und Spielfiguren). |

**Inzwischen entschieden:** Dukaten werden nie zurück in TOY getauscht (Nr. 25, 28). **Offen:** ob es schon einen NFT-Vertrag auf Polygon gibt.

## 2. Zielbild

TOYPAY wird die **Haupt-App für den nativen TOY**:

1. **Wallet für TOY auf allen Ketten (entschieden am 17.10.2026):** Die App nimmt ToyCoin auf **jeder** Kette an, auf der es ihn gibt: nativ auf ToyChain sowie als Token auf Polygon und Ethereum (später weitere). Sie zeigt den Bestand je Kette und in Summe und bridgt nativen TOY zu den anderen Ketten und zurück.
2. **Light-Knoten:** Die App prüft die Kette selbst (Sammelsignaturen, später den rekursiven Beweis) und muss keinem Server vertrauen.
3. **Teilhabe:** Optional mitsignieren (Attestierungs-Gruppen) und ToyWork-Arbeit leisten, wenn das Gerät lädt und im WLAN ist.
4. **Spiele:** Spielgegenstände und Spielfiguren können als **NFTs auf Polygon** geprägt werden, bezahlt mit Polygon-TOY (Abschnitt 5a). Sonst ist die Verbindung der Spiele zu TOY bewusst eng begrenzt (Abschnitt 7).

## 3. Technische Anbindung

- **Ein gemeinsamer Rust-Kern.** Die Crates `toy-crypto`, `toy-tx` und `toy-wallet` sowie ein Light-Client werden als Android-Bibliothek gebaut (arm64-v8a, zusätzlich armeabi-v7a für alte Geräte). Die Kotlin-App ruft sie über **UniFFI** auf, das Werkzeug von Mozilla, das aus Rust automatisch Kotlin-Anbindungen erzeugt. Kryptografie wird **nicht** in Kotlin neu geschrieben. Derselbe Kern läuft auf Linux, Windows und Raspberry Pi.
- **Vier Plattformen (entschieden am 17.10.2026):** TOYPAY gibt es als App für **Android, Windows, Linux und Raspberry Pi**. Kein iPhone vorerst. Die heutige App ist Android-Kotlin. **Vorschlag:** Die Oberfläche wird auf **Compose Multiplatform** umgestellt. Damit läuft derselbe Kotlin-Code auf Android und als Desktop-Programm auf Windows, Linux und Raspberry Pi (Linux arm64). UniFFI erzeugt die Rust-Anbindung für beide Fälle. So entsteht **eine** Codebasis statt vier. Zu prüfen beim Bau: die Desktop-Unterstützung für Linux arm64 auf dem Pi.
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
| Validator im Komitee | **nein** auf Android | muss rund um die Uhr erreichbar sein. Android beendet Hintergrund-Apps, und Ausfälle kosten Einfluss |

**Auf PC und Raspberry Pi** kann TOYPAY mehr: Voll-Knoten (256 MB) und Validator sind dort möglich, auf dem Pi als Hintergrunddienst, der rund um die Uhr läuft. Der Raspberry Pi ist damit die natürliche Plattform für Teilnehmer, die mitsignieren und an der Emission teilhaben wollen.

**TOYQUEST 3D auf dem Raspberry Pi:** Das 3D-Spiel braucht viel Grafikleistung (1 bis 3 Mio. Dreiecke je Szene). Auf einem Pi 5 ist es höchstens mit der Stufe „Niedrig“ und dem OpenGL-Renderer denkbar, auf einem Pi 4 kaum. Das muss getestet werden. Wallet und Knoten laufen auf dem Pi unabhängig davon.

**Schutz vor Sybil-Angriffen:** Belohnungen gibt es nur für Stake und geprüfte Arbeit, nie pro Gerät oder Installation. Sonst könnte jemand tausend Emulatoren starten.

## 5. Zahlungen

- **Unter Personen:** Adresse oder QR-Code, Endgültigkeit nach höchstens ca. 1 Minute. Der QR-Code folgt dem Ethereum-Format (EIP-681: `ethereum:<adresse>@<chain-id>?value=…`), dadurch können ihn auch andere Wallets lesen.
- **Im Laden:** Eine Minute an der Kasse ist spürbar lang. Möglichkeiten: Anzeige „unterwegs, wird bestätigt“ bei kleinen Beträgen, oder die Offline-Schecks (Spezifikation Abschnitt 11) von Phase 5 vorziehen. **Entscheidung offen.**
- **Kontingent:** Die App zeigt die verbleibenden kostenlosen Transaktionen der Epoche an und warnt vor der Gebühr.

## 5a. Mehrere Ketten und Spiel-NFTs

**Mehrere Ketten in einer Wallet**
- Ein Schlüssel, eine Adresse: Dank der Ethereum-kompatiblen Adressen hat der Nutzer auf ToyChain, Polygon und Ethereum **dieselbe** Adresse aus derselben Phrase.
- **Gas-Problem:** Wer TOY auf Polygon oder Ethereum bewegt, braucht dort zusätzlich POL bzw. ETH für die Gebühr. Das verwirrt Einsteiger. **Lösung:** Beide Token-Verträge können **Permit (EIP-2612)**. Der Nutzer unterschreibt nur, ein Relayer des Projekts reicht die Transaktion ein und zahlt das Gas. Die Kosten können in TOY verrechnet werden.
- **Zugang zu Polygon und Ethereum:** Für den ToyChain-Teil reicht der eingebaute Light-Knoten. Für Polygon und Ethereum braucht die App einen RPC-Anbieter. Der Alchemy-Key darf dafür nicht in der App stecken. Stattdessen läuft der Zugang über einen kleinen Proxy des Projekts, oder der Nutzer wählt öffentliche bzw. eigene RPC-Adressen.
- **Anzeige:** „TOY gesamt“ ist die Summe über alle Ketten. Für jede Kette wird angezeigt, ob der Bestand aus dem Tresor stammt und damit 1:1 einlösbar ist. Laut Entscheidung 14 gilt das für alle Token.

**Spiel-NFTs auf Polygon**
- **Wo:** Die NFTs liegen auf **Polygon**, nicht auf ToyChain. ToyChain selbst hat keine NFTs und keine Verträge (Entscheidung 36), und die NFTs sind sofort in bestehenden Wallets und Marktplätzen sichtbar.
- **Vertrag:** Dafür braucht es einen **neuen NFT-Vertrag** (ERC-721 oder ERC-1155) auf Polygon. Anders als der Token-Vertrag hat er zwangsläufig eine Präge-Funktion. Sie muss streng begrenzt sein:
  - Prägen nur mit einer Freigabe des Spielservers (signierter Gutschein nach EIP-712: „Gegenstand X gehört Spieler Y“).
  - Jeder Gutschein ist nur einmal gültig.
  - Bezahlt wird in TOY per Permit, also ohne gesonderte Freigabe.
  - Kein Admin, der beliebig prägen kann (entschieden am 17.10.2026).
  - Tageslimit für alle Prägungen zusammen und ein Notschalter, der nur pausieren kann (entschieden am 17.10.2026). Das Limit begrenzt den Schaden, falls der Schlüssel des Spielservers gestohlen wird.
  - Der Vertrag selbst ist **nicht** nachträglich änderbar (kein Proxy). „Aufwertbar“ bezieht sich nur auf die Spielfiguren.

**Präge-Regeln (entschieden am 17.10.2026)**

*Grundidee:* Der Spieler baut sich Stück für Stück eine **Gang** auf. Spielfiguren mit unterschiedlichen Stärken und Fähigkeiten, die sie in Missionen und Herausforderungen erwerben, greifen auf gemeinsame Gegenstände zu und erzeugen Synergie-Effekte. NFTs entstehen nur aus erspieltem Fortschritt, nicht aus Geld allein.

| Regel | Wert | Durchsetzung |
|---|---|---|
| Spielfigur prägbar ab | **Stufe 20** | Spielserver bestätigt die Stufe im Gutschein, der Vertrag prüft den Gutschein |
| Reihenfolge | Figur prägen → mindestens **5 Gegenstände** prägen → erst dann die nächste Figur. Die Zahl steigt nicht (bis zu 10 pro Woche sind möglich) | Vertrag zählt die Gegenstände seit der letzten Figur je Adresse |
| Spielfiguren je Adresse | höchstens **5 geprägt**. Gekaufte Figuren dürfen dazukommen, im Spiel sind höchstens **5 gleichzeitig aktiv** in der Gang | Präge-Zähler je Adresse im Vertrag, aktive Gang im Spiel |
| Spielgegenstände je Adresse | höchstens **50 geprägt**, Kauf frei | Präge-Zähler je Adresse |
| Pro Woche | höchstens **1 Spielfigur** und **10 Gegenstände** je Adresse | Zähler je Adresse und Woche (7 × 86.400 s nach Blockzeit) |
| Auflage je Art | Rubinglut höchstens **1.001 pro Saison**, Sternenfeuer höchstens **101 pro Saison** und **1.001 insgesamt** (Nr. 33, 46). Saison = 101 Tage | Zähler je Art und Saison im Vertrag, Saison aus Blockzeit und festem Start |
| Sperre je NFT | **10 TOY für 20 Tage** | Vertrag übernimmt beim Prägen 10 TOY per Permit und gibt sie nach 20 Tagen an den Präger zurück |
| Mindestbestand | **100 TOY** auf der Adresse, **zusätzlich** zur Sperre | `balanceOf ≥ 100 × 10^18` beim Prägen, vor Abzug der 10 TOY Sperre |
| Spielfiguren | teils selbst, teils zufällig konfiguriert, **aufwertbar** | Zufall entsteht beim kostenlosen Anlegen der Figur im Spiel, nicht beim Prägen |
| Aufwerten nach dem Prägen | Neue Stufe wird **auf der Kette** festgehalten, **nur steigend**, **ohne Extrakosten** (nur Gas) | Funktion `levelUp` mit Gutschein des Spielservers, nur durch den Besitzer |

**Beispiel für den schnellsten Weg:** Woche 1: Figur 1 (Stufe 20) und bis zu 10 Gegenstände. Woche 2: Figur 2, sobald 5 Gegenstände seit Figur 1 geprägt sind, usw. Eine volle Gang mit 5 Figuren braucht also mindestens 5 Wochen und mindestens 20 Gegenstände zwischen den Figuren. In einer vollen Woche (1 Figur und 10 Gegenstände) sind 110 TOY gesperrt. Weil jede Sperre 20 Tage dauert, liegen im Dauerbetrieb höchstens 330 TOY je Adresse (Prägungen an Tag 0, 7 und 14 überlappen) gleichzeitig fest.

**Wie die Regeln zusammen wirken**
- **Gegen Massenprägung:** Stufe 20 kostet echte Spielzeit, und jede Adresse sperrt 10 TOY je NFT. Mit vielen Adressen zu arbeiten bringt deshalb wenig. Das ist wirksamer als ein reiner Kontostand-Check, den man durch Weiterreichen derselben 100 TOY umgehen könnte.
- **Gegen Lootboxen (K9):** Der Zufall entsteht beim kostenlosen Anlegen der Figur. Geprägt wird nur eine Figur, deren Eigenschaften der Spieler kennt, weil er sie bis Stufe 20 gespielt hat. Niemand bezahlt für eine Überraschung.
- **Gang und Verkauf:** Jede Figur und jeder Gegenstand ist ein eigenes NFT. Synergien und gemeinsame Gegenstände berechnet das Spiel aus dem, was eine Adresse **gerade besitzt**. Wird eine Figur verkauft, verlässt sie die Gang, die Gegenstände bleiben beim Verkäufer.

**Regeln für die Sperre**
- Die 10 TOY gehören immer dem **Präger**, auch wenn das NFT innerhalb der 20 Tage verkauft wird.
- Nach 20 Tagen kann der Präger sie jederzeit abholen. Niemand sonst kann sie bewegen, auch das Projekt nicht.
- Der Notschalter kann nur **neues Prägen** pausieren, **nie** das Abholen gesperrter TOY. Sonst könnte ein Fehler oder Angriff fremde TOY festhalten.
- Gesperrte TOY bleiben gedeckte TOY im Umlauf. An der ToyScan-Rechnung ändert sich nichts.

**Präge-Gebühr und Belohnungs-Kreislauf (entschieden am 17.10.2026)**

- Neben der Sperre gibt es eine **Präge-Gebühr** in TOY (entschieden am 17.10.2026):

  | NFT | Gebühr |
  |---|---|
  | Spielfigur | **1 TOY** |
  | Spielgegenstand | **1 bis 100 TOY** je nach Wert, z. B. Flöte 1, Dolch 2, Kamel 5, Boot bis 25, schönes Haus 100 |

  Der vollständige Katalog mit **500 Gegenständen** (Gebühr, Seltenheit, Boni) steht in `toypay/docs/06_NFT_GEGENSTAENDE.md` (Entscheidung 27).

  Die Gebühr eines Gegenstands steht im Gutschein des Spielservers. Der Vertrag nimmt nur Gebühren zwischen 1 und 100 TOY an. Neue Gegenstände brauchen deshalb keinen Admin, und ein gestohlener Server-Schlüssel kann nichts außerhalb dieser Spanne prägen (dazu kommen Tages- und Wochenlimits).
- **Was man auf der Adresse braucht:** 100 TOY Mindestbestand (geprüft vor dem Prägen), davon gehen Gebühr und 10 TOY Sperre ab. Für ein Haus sind also mindestens 110 TOY nötig.
- Die Gebühren gehen an eine öffentlich bekannte **Gebühren-Adresse des Projekts** auf Polygon. Sie ist **nicht** die Ersteller-Adresse und wird mit mehreren Schlüsseln gesichert (Multisig). ToyScan zeigt ihren Bestand.
- Die Gebühr **fließt als Spielwert zurück ins Spiel**, nicht als TOY (Entscheidung 25, Einbahnstraße).
- **Kreislauf:** Belohnungen wie Goldtaler, Gegenstände, Nahrungsmittel, Zaubertränke, Kamelsättel und Werkzeuge im Gegenwert von **0,01 bis 2 TOY** (1 bis 200 Dukaten) werden zufällig an logischen Orten der Spielwelt verteilt. **Nur geprägte Spielfiguren (NFTs)** können sie einsammeln. Ausgezahlt wird **im Spiel** als Dukaten, Gold oder Gegenstand, **nie als TOY**.
- **Fester Kurs:** **1 TOY = 100 Dukaten = 1.000 Gold.** Dukaten und Gold sind die Spielwährungen. Der Kurs bleibt fest, auch wenn sich der Marktpreis von TOY ändert.
- **Haushaltsregel (Vorschlag):** Der Gegenwert der verteilten Belohnungen einer Woche entspricht höchstens den Gebühren der Vorwoche. Rechtlich nötig ist das bei der Einbahnstraße nicht, es hält aber die Spielwirtschaft im Gleichgewicht und macht den Kreislauf nachprüfbar. Das Spiel erzeugt nie neues TOY.

**Gas und Bezahlen mit Dukaten (entschieden am 17.10.2026)**

- **Vorerst** zahlt der Spieler die Polygon-Gebühr (POL) selbst.
- **Später:** Der Spieler kann im Spiel auch mit **Dukaten** zahlen. Ein Dienst des Projekts (Relayer) reicht die Prägung ein und bezahlt POL und die TOY-Gebühr. Die 10 TOY Sperre und die 100 TOY Mindestbestand bleiben echte TOY auf der Adresse des Spielers.

**Zwei Sorten Dukaten (entschieden am 17.10.2026)**

Damit erspielte Dukaten nicht über Präge-Gebühren und NFT-Verkäufe zu TOY werden, gibt es zwei Sorten:

| | **Sultansdukaten** | **Königsdukaten** |
|---|---|---|
| Herkunft | nur in der **Wechselstube** gegen TOY (1 TOY = 100) | **erspielt**: Belohnungen, Missionen, Funde, Schätze alter Könige |
| Präge-Gebühren bezahlen | **ja** (über den Dienst des Projekts) | **nein** |
| Alles andere im Spiel | ja | ja |
| Zurück in TOY | nein (Einbahnstraße) | nein |
| Tausch gegen die andere Sorte | **nein** | **nein** |
| Zwischen Spielern übertragbar | **nein**, an das Spielerkonto gebunden | ja, im Spiel (Handel, Geschenke) |

- **Gold** ist die kleine Münze der erspielten Sorte: 10 Gold = 1 Königsdukat (Entscheidung 34).
- **Königsdukaten und Gold** dürfen Spieler einander schenken und im Spiel handeln (Entscheidung 35). Sultansdukaten nie.
- **Prüfbar:** Die Summe aller ausgegebenen Sultansdukaten ist immer genau 100 × die in der Wechselstube eingegangenen TOY. Der Spielserver führt beide Konten getrennt.
- **Warum Sultansdukaten nicht übertragbar sind:** Sonst könnte ein Spieler erspielte Königsdukaten oder Gegenstände an andere gegen deren Sultansdukaten verkaufen, damit Gebühren zahlen und so doch zu TOY kommen.
- **Was bewusst bleibt:** Geprägte Spielfiguren und Gegenstände sind erspielt und dürfen als NFTs verkauft werden. Das ist der Kern des Gang-Prinzips. Ihren Preis bestimmen die Spieler auf dem Marktplatz, nicht das Spiel. Dieser Punkt gehört auf die Liste der rechtlichen Prüfung (K10).

- **Spiel und Kette synchron halten:** Ist ein Gegenstand als NFT geprägt, gilt **das NFT** als Eigentumsnachweis. Wird das NFT verkauft, wandert der Gegenstand im Spiel zum neuen Besitzer. Der Spielserver liest dazu die Kette. Ein geprägter Gegenstand darf im Spiel nicht zusätzlich weiter gehandelt werden, sonst gäbe es ihn doppelt.
- **Schlüssel des Spielservers:** Der Server signiert die Gutscheine und ist damit ein lohnendes Angriffsziel. Deshalb: eigener Schlüssel nur für Gutscheine (nie für TOY), Tageslimits im Vertrag und ein Notschalter, der das Prägen pausieren kann.
- **Wohin die TOY für das Prägen fließen:** an die Multisig-Gebühren-Adresse des Projekts auf Polygon und von dort zurück ins Spiel (Nr. 26, 27). ToyScan zeigt ihren Bestand.

## 6. Schlüssel und Verwahrung

- **Nicht verwahrend:** Die Schlüssel liegen nur auf dem Gerät (Android Keystore, wenn vorhanden StrongBox), gesichert mit Argon2id und XChaCha20-Poly1305. Weder der Server noch das Projekt kann auf Guthaben zugreifen. Das ist sicherer und rechtlich in der Regel einfacher als eine verwahrende App (siehe Abschnitt 7, K5).
- Phrase-Sicherung beim Einrichten, wahlweise eigener ToyChain-Ableitungspfad (Spezifikation Abschnitt 8).
- Schlüssel und Phrasen erscheinen nie in Logs, Absturzberichten oder Server-Anfragen.

## 7. Kritische Punkte, die vor dem Bau entschieden werden müssen

Hinweis: Das ist keine Rechtsberatung. Die Punkte zeigen, wo eine rechtliche Prüfung nötig ist.

**K1 Spielwährung oder Kryptowährung.** In TOYPAY heißt ToyCoin „Spielwährung“. Der native TOY wird aber 1:1 mit den ERC-20-Token auf Polygon und Ethereum verbunden. Auf Polygon gab es bereits einen Handelspool. Damit hat TOY einen echten Geldwert und ist rechtlich keine reine Spielwährung mehr. Bezeichnung, App-Store-Angaben und Nutzungsbedingungen müssen das widerspiegeln.

**K2 Poker.** Poker mit TOY als Einsatz wäre Echtgeld-Glücksspiel. In Deutschland ist Online-Glücksspiel nur mit deutscher Erlaubnis zulässig, und erlaubte Anbieter dürfen Kryptowährungen nicht als Zahlungsmittel annehmen. Google Play verlangt für Krypto-Funktionen zertifizierte Dienste in regulierten Ländern und verbietet ausdrücklich, Krypto-Werte als Einsatz für Gewinne mit Geldwert zu nutzen. **Vorschlag:** Poker nur mit Spielchips, die nie in TOY getauscht und nie mit TOY gekauft werden können. Oder Poker ganz aus der App nehmen, in der TOY liegt.

**K3 Dukaten, Gold und TOY: Einbahnstraße oder Kreislauf?** Mit dem festen Kurs 1 TOY = 100 Dukaten = 1.000 Gold und Belohnungen im Wert von 0,01 bis 2 TOY (Entscheidung 23) muss geklärt sein, ob aus dem Spiel **echte TOY herauskommen**. Drei Möglichkeiten:

| | A: Einbahnstraße | B: Kreislauf auf der Kette | C: Dukaten frei tauschbar |
|---|---|---|---|
| TOY → Dukaten | ja | ja | ja |
| Dukaten → TOY | **nein** | nein, aber Belohnungen werden als **echte TOY** aus einem Belohnungs-Vertrag ausgezahlt | **ja**, jederzeit 1:100 |
| Wo liegen die Gebühren | beim Projekt, das sie in Form von Spiel-Belohnungen zurückgibt | in einem **Belohnungs-Vertrag** auf Polygon, sichtbar für alle | beim Spielserver |
| Wer verwahrt Werte der Spieler | niemand, Dukaten sind reines Spielgeld | der Vertrag, nicht das Projekt | **der Spielserver**, er wird zur Bank |
| Rechtliches Risiko | gering, wie Spielwährung in üblichen Spielen | **mittel bis hoch**: bezahlter Zugang (Gebühr, Sperre) plus zufällig verteilte Gewinne mit Geldwert kann als Glücksspiel gelten. Dazu Steuern für Spieler und Jugendschutz | **hoch**: Verwahrung und Umtausch nach MiCA zulassungspflichtig |
| Technik | einfach | Belohnungs-Vertrag mit Gutscheinen, Wochenlimit und „nie mehr aus als rein“ | Hot-Wallet mit allen Spielerwerten, großes Angriffsziel |

**Entschieden am 17.10.2026: A, Einbahnstraße.** TOY lassen sich in Dukaten tauschen, Dukaten und Gold nie zurück in TOY. Belohnungen gibt es nur als Spielwerte. B käme frühestens nach einer rechtlichen Prüfung in Frage, C nicht. In allen Fällen gilt: Das Spiel erzeugt **nie** neues TOY. Der Umweg über NFT-Verkäufe ist durch zwei Sorten Dukaten geschlossen (Entscheidung 28, Abschnitt 5a).

**K4 Jugendschutz.** **Entschieden am 17.10.2026: ab 16 Jahren.** Daraus folgt eine Frage für die rechtliche Prüfung: 16- und 17-Jährige sind in Deutschland nur beschränkt geschäftsfähig. Verträge wie der Kauf von TOY, Sultansdukaten oder NFTs brauchen grundsätzlich die Zustimmung der Eltern, außer bei kleinen Beträgen aus dem eigenen Taschengeld. Viele Krypto-Dienste verlangen deshalb 18 Jahre. **Entschieden am 17.10.2026 (Entscheidung 31):** Die Spiele ab 16. Alles, was echte TOY bewegt (Wechselstube, Sultansdukaten, Prägen, Bridge, NFT-Handel), **ab 18 mit Altersnachweis**. Unter 18 gibt es nur Königsdukaten und Gold, und geprägte NFTs sind dann nicht möglich. Auch Poker mit Spielchips braucht eine passende Altersfreigabe (USK).

**K5 Verwahrung.** Hält TOYPAY nie fremde Schlüssel, ist die App eine Software-Wallet. Wer Guthaben für Nutzer verwahrt oder tauscht, braucht nach MiCA voraussichtlich eine Zulassung als Krypto-Dienstleister. **Vorschlag:** strikt nicht verwahrend.

**K6 Datenschutz.** Der Spielserver kennt Nutzerkonten. Verknüpft er sie mit TOY-Adressen, entstehen personenbezogene Finanzdaten (DSGVO). **Vorschlag:** Der Server speichert keine Wallet-Adressen, oder nur mit ausdrücklicher Einwilligung.

**K8 NFT-Prägung vor dem Start von ToyChain berührt den Fremdbestand.** Wer im Spiel NFTs prägen will, braucht TOY auf Polygon. Vor dem Start von ToyChain und der Bridge können diese TOY nur aus der Ersteller-Adresse kommen. Jeder TOY, der von dort an Spieler geht, **erhöht den Fremdbestand**. Laut Entscheidung 13 und 14 darf er beim Start aber höchstens 10.000 TOY betragen, gedeckt aus der Reserve. Möglichkeiten:
- (a) Die NFT-Prägung mit TOY startet erst **nach** dem Start der Bridge. Dann kommen alle Polygon-TOY aus dem Tresor und sind gedeckt.
- (b) Die TOY, mit denen geprägt wird, fließen an die Ersteller-Adresse zurück. Der Fremdbestand bleibt dann ungefähr gleich, muss aber zum Start trotzdem ≤ 10.000 sein.
- (c) Die Deckung im Genesis-Block wird auf den tatsächlichen Fremdbestand am Stichtag erhöht. Das geht zulasten der Reserve (990.000 TOY), und der Fremdbestand darf diese nie übersteigen.

**Entschieden am 17.10.2026: (a).** Die NFT-Prägung mit TOY startet erst nach dem Start der Bridge. Alle Polygon-TOY, mit denen Spieler prägen, kommen dann aus dem Tresor und sind durch native TOY gedeckt.

Folgen:
- **Die Ersteller-Adresse gibt bis zur Einzahlung in den Tresor keine TOY mehr aus.** Erlaubt ist nur die Rückübertragung nach Entscheidung 13, in Richtung Ersteller-Adresse. Jeder Abgang würde den Fremdbestand über die 10.000 TOY hinaus erhöhen. ToyScan überwacht die Adresse ab sofort und zeigt jeden Abgang an.
- **NFTs ohne TOY** (kostenlos oder gegen POL) sind auch vorher möglich, ohne die Deckung zu berühren. Ob das gewünscht ist, ist offen.

**K9 Zufalls-NFTs (Lootboxen).** NFTs mit zufälligem Wert gegen TOY zu verkaufen, gilt nach den Google-Play-Regeln als Glücksspiel, wenn die App nicht als Glücksspiel-App zugelassen ist. Außerdem dürfen NFTs nicht als Einsatz für Gewinne mit Geldwert dienen. Das betrifft auch Poker (K2). Spielfiguren sollen teils zufällig konfiguriert werden (Entscheidung 19). **Gelöst durch Entscheidung 20:** Figuren werden erst ab Stufe 20 geprägt. Der Zufall entsteht beim kostenlosen Anlegen im Spiel, geprägt wird nur eine bekannte, erspielte Figur. Dasselbe gilt für Gegenstände: Prägbar ist nur, was der Spieler schon im Spiel besitzt.

**K10 NFTs und Recht.** Einzigartige NFTs fallen in der Regel nicht unter MiCA. Große Serien gleichartiger NFTs können aber anders bewertet werden. Gehandelte Spielgegenstände berühren zudem Steuer- und Verbraucherrecht. Teil der rechtlichen Prüfung.

**K7 Eine App oder zwei.** TOYPAY kommt auf vier Plattformen (Entscheidung 30). TOYQUEST 3D ist schon heute ein eigenes Programm. Offen ist, ob Wallet, Knoten und die 2D-Spiele (Stadtviertel, Poker, TOYQUEST) **in einer** TOYPAY-App bleiben. Wallet und Spiele in einer App machen sie groß und vermischen Risiken (Glücksspiel, Jugendschutz, Finanzfunktionen). Vor allem die Altersgrenze 18 für TOY-Funktionen (K4) spricht für eine Trennung. **Entschieden am 17.10.2026 (Entscheidung 32):** Es gibt die **TOYPAY Wallet** als schlanke App mit Knoten auf allen vier Plattformen (ab 18). Die Spiele sind getrennte Apps (ab 16): die 2D-Spiele (Stadtviertel, Poker, TOYQUEST) und TOYQUEST 3D. Spiele und Wallet verbinden sich nur über eine Anmeldung mit der Wallet-Adresse. Das Spiel sieht die Adresse, nie den Schlüssel.

## 8. Abhängigkeiten zum ToyChain-Bauplan

| TOYPAY braucht | kommt aus |
|---|---|
| Adressen, Phrase, Signaturen | M2 (`toy-crypto`) |
| Transaktionen, Kontingent | M3 (`toy-tx`) |
| Ethereum-kompatible Schnittstelle für den Umstieg | M7 (`toy-rpc`) |
| Wallet-Kern, Keystore | M8 (`toy-wallet`) |
| Android-Bibliothek mit Kotlin-Anbindung | M9 (Plattformen) |
| Offline-Zahlung im Laden | Phase 5 (oder vorgezogen, siehe Abschnitt 5) |
| Bridge ToyChain ↔ Polygon/Ethereum in der App | Phase 3 (Bridge, vorgezogen), Konzept in `06-bridge.md` |
| NFT-Vertrag auf Polygon mit Gutschein-Prägung | eigener Baustein außerhalb von ToyChain, eigenes Audit |

## 9. Fragen an den Projektinhaber

Beantwortet am 17.10.2026: Die App nimmt TOY auf allen Ketten an, bridgt nativen TOY und prägt NFTs auf Polygon mit Polygon-TOY (Entscheidungen 15 und 16). NFT-Prägung mit TOY erst nach dem Bridge-Start (Entscheidung 17).

Beantwortet am 17.10.2026: Altersgruppe ab 16 (Entscheidung 29), Plattformen Android, Windows, Linux und Raspberry Pi (Entscheidung 30), TOY-Funktionen ab 18 (Entscheidung 31), Wallet und Spiele getrennt (Entscheidung 32).

Offen:
1. ~~Poker-Chips getrennt (K2)?~~ Entschieden: Poker nur mit Basar-Chips, getrennt von Dukaten und TOY (`toypay/docs/07_SPIELDESIGN.md`, Abschnitt 6).
2. Sollen NFTs schon vor dem Bridge-Start **ohne TOY** geprägt werden können (kostenlos oder gegen POL)?
3. Gibt es schon einen NFT-Vertrag auf Polygon? Wenn ja, welche Adresse?
