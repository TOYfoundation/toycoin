# TOYQUEST: Items als NFT auf Polygon

Stand 24.10.2026 · **für später** (Spiel-Anbindung, Entscheidung 72), nicht Teil von Phase 1 · Quellen: Konzeptblatt „TOYQUEST: Items als NFT auf Polygon“ vom 21.10.2026 (Abschnitte 1 bis 8), ToyVerse-Beschreibung S. 14 · Entscheidungen 126 und 127 · Lernwelt und Zeitreise: `../../toypay/docs/08_LERNWELT_ZEITREISE.md` · Überblick: `11-toyverse.md`

Dieses Dokument **ersetzt** in `07-toypay-spiel-und-nft.md` die Regeln zu Präge-Gebühr, Gas und Gebühren-Kreislauf (Entscheidungen 23, 24, 26). Die übrigen Präge-Regeln dort gelten weiter, soweit Abschnitt 7 nichts anderes sagt.

## 1. Ziel

Spieler erspielen Items in TOYQUEST und können sie **optional** als NFT auf Polygon prägen. Die Prägung kostet eine kleine Gebühr. Die Netzgebühr (POL) übernimmt ein Topf, damit Spieler selbst kein POL brauchen.

## 2. Grundprinzip (entschieden am 21.10.2026)

- Items werden **im Spiel erspielt**, nicht gekauft. Kein Pay-to-Win.
- Die Gebühr dient **nur** für das Prägen als NFT.
- Das Item funktioniert im Spiel auch ohne NFT (offline und online).
- Ein **POL-Topf** zahlt das Polygon-Gas (Gas-Sponsoring).
- Auf der ToyChain gibt es keine NFTs (Entscheidung 36). Die Items liegen als **ERC-1155** auf Polygon.

## 3. Ablauf

1. Der Spieler verdient ein Item im Spiel.
2. Der Spielserver signiert einen **einmaligen Mint-Gutschein** nach EIP-712 (Wallet, Item, Menge, Preis-Angebot, Nonce, Ablaufzeit von wenigen Minuten).
3. Der Spieler tippt in TOYPAY auf „Als NFT prägen“, stellt optional den Regler ein und signiert die Zahlung per `permit` (EIP-2612), ohne Extra-Transaktion.
4. Ein ERC-4337-Paymaster oder Relayer zahlt das POL aus dem Topf.
5. Der Mint-Vertrag prüft den Gutschein, zieht die Gebühr ein, verteilt sie und prägt das Item.
6. ToyScan zeigt Topf-Stand, Mint-Zahlen und Verteilung, gelb bei niedrigem Topf.

## 4. Prägegebühr

**Formel (entschieden):** Prägegebühr = Gaskosten in POL × Faktor ÷ Kurs (Gebühren-Währung je POL).

| Teil | Regel | Status |
|---|---|---|
| Faktor | Start **2**, später **1,5**, sobald der Betrieb stabil läuft | Entschieden |
| Gaskosten | Ein Mint verbraucht ungefähr gleich viel Gas, der Gaspreis wird aktuell gemessen | Entschieden |
| Preis im Gutschein | gilt nur wenige Minuten | Entschieden |
| Schutz | Mindest- und Höchstgebühr, Begrenzung der Preisänderung je Stunde, Notschalter bei veralteten Preisdaten | Entschieden |
| **Währung der Gebühr** | siehe 4.1 | **Offen (Nr. 127)** |
| **Kurs** | Chainlink-Feed für POL. Für die Gebühren-Währung TWAP aus einem Pool oder ein signierter Preis mit festen Grenzen | **Offen (Nr. 127, am 24.10.2026 auf später gelegt)** |

### 4.1 Offen: Mit welchem Geld wird bezahlt?

Das Konzeptblatt sieht die Zahlung per `permit` über den **ToyCoin-Vertrag auf Polygon** vor, also mit dem alten ToyCoin-Token (0x1854…A5BF). Natives ToyChain-TOY gibt es auf Polygon nicht, weil es keine Bridge gibt (Entscheidung 63). Entscheidung 63 sagt außerdem, dass die alten Token **ignoriert** werden. Diese Entscheidung wird hier nicht geändert. Zur Wahl stehen:

| Weg | Vorteil | Nachteil |
|---|---|---|
| A. Alter ToyCoin-Token auf Polygon (Absicht des Konzeptblatts) | `permit` vorhanden, keine neue Währung | braucht eine Ausnahme von Nr. 63. Wallet und ToyScan müssen dann klar sagen: „bezahlt Prägegebühren, ist kein ToyChain-TOY, nicht eintauschbar“. Kurs nur mit Pool |
| B. Euro-Stablecoin, später ToyCash | stabile Gebühr, kein eigener Pool nötig, Kurs nur POL/EUR | TOY spielt beim Prägen keine Rolle mehr |
| C. Nur Dukaten, Projekt füllt den Topf | keine Kryptozahlung der Spieler | Projekt trägt die Kosten, Bots müssen anders gebremst werden |

Bis zur Entscheidung schreiben die Dokumente „Gebühren-Währung“ statt TOY.

## 5. Verteilung der Gebühr

**Entschieden:** Die Gebühr wird zwischen **Projekt-Wallet** und **POL-Topf** aufgeteilt, kein Burn. Der Spieler bestimmt die Aufteilung in einem Rahmen selbst.

**Vorschlag (zur Bestätigung):**

- Die ersten **1× Gaskosten** gehen immer in den Topf. So läuft er nicht leer.
- Den Rest (0,5 bis 1× Gaskosten) teilt der Spieler per Regler auf, Standard 70 % Projekt-Wallet, 30 % Topf.
- Wer mehr in den Topf gibt, bekommt ein Zeichen im Spiel (Titel oder Kosmetik).
- Aufteilung und Grenzen stehen fest im Mint-Vertrag, der Gesamtbetrag bleibt gleich.
- **Auffüllen:** Der Topf-Anteil muss in POL getauscht werden. Automatisch nur mit einem Pool mit genug Liquidität, sonst von Hand und in ToyScan belegt.

## 6. Überschüsse im Topf

**Entschieden (21.10.2026):** Überschüsse gehen anteilig an alle Spieler, die geprägt haben. **Entschieden (24.10.2026):** als Rabatt **oder** Auszahlung, gedeckelt auf die eigene Einzahlung.

**Vorschlag (zur Bestätigung):**

- **Überschuss** = alles über dem Zielstand (z. B. 3 Monate erwartete Gaskosten).
- **Anteil** nach gezahlter Prägegebühr, nicht nach Zahl der Mints.
- **Deckel:** Niemand bekommt mehr zurück, als er eingezahlt hat. Es ist eine Rückerstattung, keine Rendite.
- **Form:** Standard Rabatt auf spätere Prägungen (kostet kein Gas). Wahlweise Auszahlung mit einer Abhol-Transaktion.
- **Rhythmus:** monatlich, mit öffentlichem Merkle-Beweis je Anspruch, dargestellt in ToyScan.
- Sind die tatsächlichen Gaskosten geringer, erstattet der Vertrag die Differenz sofort.

## 7. Was mit den älteren Spiel-Entscheidungen passiert

Die Entscheidungen 16 bis 35 und 46 gehören zur Spiel-Anbindung (kommt später, Nr. 72). Sie werden so behandelt:

| Nr. | Inhalt | Ab jetzt |
|---|---|---|
| 16 | NFTs auf Polygon | gilt, jetzt ERC-1155 |
| 17 | Prägen erst nach Bridge-Start mit TOY | hängt an der Währung (4.1), bis dahin offen |
| 18 | kein Admin-Prägen, Tageslimit, Notschalter nur Pause | gilt |
| 19, 22 | 100 TOY Mindestbestand | **zu prüfen**, passt nur zu Weg A in 4.1 |
| 20 | Gang-Prinzip, Figuren ab Stufe 20 | gilt |
| 21 | 10 TOY Sperre je NFT für 20 Tage | **zu prüfen**, passt nur zu Weg A in 4.1 |
| 23 | Präge-Gebühr fließt als Schatzfunde zurück ins Spiel | **ersetzt** durch 5 und 6. Der Schatzwind wird aus der Spielwirtschaft gespeist (`../../toypay/docs/07_SPIELDESIGN.md`) |
| 24 | Spieler zahlt POL | **ersetzt** durch den POL-Topf |
| 25, 28, 34, 35 | Dukaten-Einbahnstraße | gilt. Ob Sultansdukaten Prägegebühren zahlen dürfen, hängt an 4.1 |
| 26 | Gebühr 1 bis 100 TOY nach Wert | **ersetzt** durch die Gas-Formel (4) |
| 27 | Katalog mit 500 Items | gilt als Item-Liste, die Gebühren-Spalte entfällt |
| 33, 46 | Auflage Sternenfeuer und Rubinglut | gilt als feste Höchstzahl je Item im Vertrag |

## 8. Schutz vor Missbrauch

- Nur Items mit **Gutschein vom Spielserver** (einmalig, Nonce, Ablaufzeit). Sonst könnten Bots beliebig viele erzeugen.
- Gebühr immer über den Gaskosten. Limits je Wallet und Tag, Notschalter für das Sponsoring.
- **Verifizierender Paymaster mit Gas-Richtlinien** (nur der Mint-Vertrag, nur gültige Gutscheine), damit niemand den Topf leersaugt.
- Zufällige Eigenschaften nur mit fairem Zufall (Commit-Reveal oder Chainlink VRF).
- Metadaten auf IPFS, ein Provenance-Hash im Vertrag belegt, dass sie unverändert sind.
- Mint-Vertrag und Paymaster brauchen vor dem Start ein **externes Audit**.
- Der Spielserver-Schlüssel ist der wichtigste Angriffspunkt. Das Tageslimit (Nr. 18) begrenzt den Schaden, wenn er gestohlen wird.

## 9. Technik

- **Netz:** Polygon (später Ethereum möglich).
- **Token:** ERC-1155, feste Höchstzahl je Item im Vertrag, kein Proxy, kein Admin-Prägen.
- **Zahlung:** `permit` (EIP-2612), also Signatur statt Freigabe-Transaktion.
- **Gas-Sponsoring:** ERC-4337-Paymaster oder Relayer, gespeist aus dem POL-Topf.
- **TOYPAY:** prägt über den Polygon-Smart-Account (Nr. 128). Die Spiel-App hält nie Schlüssel (Nr. 32).
- **ToyScan:** Kriterium B3 (Topf) und B4 (Mint-Vertrag), `../05-toyscan.md` Abschnitt 11.

## 10. Offene Punkte

1. Währung der Gebühr und Kurs (4.1, Nr. 127).
2. Rahmen des Reglers und Standardwerte (5).
3. Gibt es einen Pool auf Polygon mit genug Liquidität, oder muss er angelegt werden?
4. Größe und Art der Kollektion (fest begrenzt oder wachsend).
5. Zielstand und Rhythmus der Überschüsse (6).
6. Ob Nr. 19 und 21 (Mindestbestand, Sperre) weiter gelten (7).
7. Rechtliche Prüfung zum Schluss.
