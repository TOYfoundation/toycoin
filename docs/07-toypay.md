# TOYPAY im Gesamtsystem

Entwurf v0.1 vom 17.10.2026 · Grundlage: TOYPAY-Unterlagen Stand Runde 8 (im Projekt unter `toypay/docs/`) · Status: **zur Prüfung, mehrere Grundsatzentscheidungen offen (Abschnitt 7)**

Dieses Dokument beschreibt, wie die TOYPAY-App mit ToyChain zusammenspielt. Die Spielinhalte (TOYQUEST 3D, Grafik, Erfindungen, Sprachen) sind nicht Teil des ToyChain-Konzepts und werden hier nur erwähnt, wo sie die Kette berühren.

## 1. Ist-Stand laut Unterlagen

| Teil | Stand |
|---|---|
| **TOYPAY-App** | Android, Kotlin. Enthält die „Spielwährung ToyCoin“, Stadtviertel, **Poker** und das 2D-Abenteuer TOYQUEST. 28 Sprachen. |
| **TOYQUEST 3D** | Eigenes Spiel (Godot 4.4), Paket `app.toypay.quest3d`, Exporte für Android arm64, Windows, Linux. Spielgeld **Dukaten**, im Spiel gibt es eine **Wechselstube**. 45 Sprachen. |
| **TOYPAY-Server** | Spielserver mit Online-Modus und Weltdaten. |
| **Hinweise auf Blockchain** | `local.properties` enthält einen **Alchemy-Key** (Zugang zu Ethereum/Polygon), und Seed-Phrasen werden erwähnt. Die App spricht also vermutlich schon mit der Ethereum- oder Polygon-Kette. **Zu bestätigen.** |

**Nicht beschrieben** sind: welche Wallet-Funktionen die App heute hat, ob „ToyCoin“ in der App der ERC-20-Token auf Polygon ist oder interne Punkte, und ob Dukaten in ToyCoin getauscht werden können.

## 2. Zielbild

TOYPAY wird die **Haupt-App für den nativen TOY**:

1. **Wallet:** TOY senden und empfangen, Kontostand, kostenloses Kontingent anzeigen, Bridge zu Polygon und Ethereum.
2. **Light-Knoten:** Die App prüft die Kette selbst (Sammelsignaturen, später den rekursiven Beweis) und muss keinem Server vertrauen.
3. **Teilhabe:** Optional mitsignieren (Attestierungs-Gruppen) und ToyWork-Arbeit leisten, wenn das Gerät lädt und im WLAN ist.
4. **Spiele:** bleiben eigene Bereiche. Ihre Verbindung zu TOY ist bewusst eng begrenzt (Abschnitt 7).

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

## 9. Fragen an den Projektinhaber

1. Was kann die App heute mit ToyCoin: Ist es der ERC-20-Token auf Polygon (über Alchemy) oder ein interner Punktestand?
2. Kann man heute Dukaten oder Poker-Gewinne in ToyCoin tauschen, oder ToyCoin im Spiel einsetzen?
3. Für welches Alter sind TOYPAY und TOYQUEST gedacht?
4. Eine App mit allem, oder Wallet und Spiele getrennt (K7)?
5. Nur Android, oder später auch iPhone?
