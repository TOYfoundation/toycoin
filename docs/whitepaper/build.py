"""ToyChain whitepaper, draft 0.4 (German). Run figures.py first, then this script."""
from reportlab.lib.pagesizes import A4
from reportlab.lib.units import cm
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.enums import TA_JUSTIFY, TA_CENTER, TA_LEFT
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (BaseDocTemplate, Frame, PageTemplate, Paragraph, Spacer, Table, TableStyle,
                                Image, KeepTogether, ListFlowable, ListItem, CondPageBreak)
from PIL import Image as PILImage

D = "/usr/share/fonts/truetype/dejavu/"
for name, f in [("Serif", "DejaVuSerif.ttf"), ("Serif-B", "DejaVuSerif-Bold.ttf"), ("Serif-I", "DejaVuSerif-Italic.ttf"),
                ("Serif-BI", "DejaVuSerif-BoldItalic.ttf"), ("Sans", "DejaVuSans.ttf"), ("Sans-B", "DejaVuSans-Bold.ttf")]:
    pdfmetrics.registerFont(TTFont(name, D + f))
pdfmetrics.registerFontFamily("Serif", normal="Serif", bold="Serif-B", italic="Serif-I", boldItalic="Serif-BI")
pdfmetrics.registerFontFamily("Sans", normal="Sans", bold="Sans-B", italic="Sans", boldItalic="Sans-B")

INK = colors.HexColor("#1b1b1a")
MUTED = colors.HexColor("#52514e")
RULE = colors.HexColor("#c9c8c2")
TINT = colors.HexColor("#f3f2ee")
ACCENT = colors.HexColor("#2a78d6")
LINKC = "#1f5fae"

body = ParagraphStyle("body", fontName="Serif", fontSize=9.6, leading=13.6, alignment=TA_JUSTIFY, textColor=INK,
                      spaceAfter=5.5)
small = ParagraphStyle("small", parent=body, fontSize=8.2, leading=11, spaceAfter=3)
more = ParagraphStyle("more", parent=body, fontName="Sans", fontSize=7.9, leading=10.8, textColor=MUTED,
                      alignment=TA_LEFT, spaceAfter=6)
cell = ParagraphStyle("cell", fontName="Sans", fontSize=7.9, leading=10.2, textColor=INK, alignment=TA_LEFT)
cellb = ParagraphStyle("cellb", parent=cell, fontName="Sans-B")
h1 = ParagraphStyle("h1", fontName="Sans-B", fontSize=12, leading=15, spaceBefore=11, spaceAfter=5, textColor=INK)
h2 = ParagraphStyle("h2", fontName="Sans-B", fontSize=10, leading=13, spaceBefore=6, spaceAfter=3, textColor=INK)
title = ParagraphStyle("title", fontName="Serif-B", fontSize=19, leading=24, alignment=TA_CENTER, textColor=INK)
subtitle = ParagraphStyle("sub", fontName="Serif", fontSize=11, leading=15, alignment=TA_CENTER, textColor=MUTED)
center = ParagraphStyle("center", parent=body, alignment=TA_CENTER, fontSize=9.4)
abstract = ParagraphStyle("abs", parent=body, fontSize=9.2, leading=13, leftIndent=1.1 * cm, rightIndent=1.1 * cm)
caption = ParagraphStyle("cap", parent=small, alignment=TA_CENTER, textColor=MUTED, fontName="Serif-I", spaceAfter=8)
toc = ParagraphStyle("toc", parent=body, fontName="Sans", fontSize=8.6, leading=12, alignment=TA_LEFT, spaceAfter=0)

story = []
sec = [0]
fig_no = [0]
LINKS = []  # (topic, label, url)


def L(text, url, topic=None):
    if topic:
        LINKS.append((topic, text, url))
    return f'<a href="{url}" color="{LINKC}"><u>{text}</u></a>'


def H(text):
    sec[0] += 1
    story.append(CondPageBreak(3 * cm))
    story.append(Paragraph(f"{sec[0]}.&nbsp;&nbsp;{text}", h1))


def H2(text):
    story.append(CondPageBreak(2 * cm))
    story.append(Paragraph(text, h2))


def P(text, style=body):
    story.append(Paragraph(text, style))


def MORE(*links):
    P("<b>Mehr dazu:</b> " + " · ".join(links), more)


def B(items, style=body):
    story.append(ListFlowable([ListItem(Paragraph(t, style), leftIndent=12, value="•") for t in items],
                              bulletType="bullet", start="•", leftIndent=12, bulletFontName="Serif",
                              bulletFontSize=8, spaceAfter=4))


def T(rows, widths, header=True):
    data = [[Paragraph(c, cellb if (header and i == 0) else cell) for c in r] for i, r in enumerate(rows)]
    t = Table(data, colWidths=[w * cm for w in widths], repeatRows=1 if header else 0)
    st = [("VALIGN", (0, 0), (-1, -1), "TOP"), ("LINEBELOW", (0, 0), (-1, 0), 0.8, INK),
          ("LINEBELOW", (0, -1), (-1, -1), 0.6, RULE),
          ("TOPPADDING", (0, 0), (-1, -1), 3), ("BOTTOMPADDING", (0, 0), (-1, -1), 3),
          ("LEFTPADDING", (0, 0), (-1, -1), 4), ("RIGHTPADDING", (0, 0), (-1, -1), 4)]
    for i in range(2, len(rows), 2):
        st.append(("BACKGROUND", (0, i), (-1, i), TINT))
    t.setStyle(TableStyle(st))
    story.append(Spacer(1, 2))
    story.append(t)
    story.append(Spacer(1, 7))


def BOX(text):
    t = Table([[Paragraph(text, ParagraphStyle("bx", parent=body, fontName="Sans", fontSize=8.8, leading=12.4,
                                               alignment=TA_LEFT, spaceAfter=0))]], colWidths=[16.4 * cm])
    t.setStyle(TableStyle([("BOX", (0, 0), (-1, -1), 1.1, ACCENT),
                           ("BACKGROUND", (0, 0), (-1, -1), colors.HexColor("#eef4fc")),
                           ("TOPPADDING", (0, 0), (-1, -1), 7), ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
                           ("LEFTPADDING", (0, 0), (-1, -1), 9), ("RIGHTPADDING", (0, 0), (-1, -1), 9)]))
    story.append(t)
    story.append(Spacer(1, 8))


def FIG(path, text, width=15.6):
    fig_no[0] += 1
    w, h = PILImage.open(path).size
    story.append(Spacer(1, 3))
    story.append(KeepTogether([Image(path, width=width * cm, height=width * cm * h / w),
                               Paragraph(f"Abbildung {fig_no[0]}: {text}", caption)]))


# Links (all checked in October 2026)
U = dict(
    btc_pdf="https://bitcoin.org/bitcoin.pdf",
    btc_de="https://www.giga.de/downloads/bitcoin/specials/bitcoin-whitepaper-das-dokument-auf-deutsch-und-als-pdf/",
    btc_supply="https://en.bitcoin.it/wiki/Controlled_supply",
    btc_size="https://www.spark.money/research/bitcoin-full-node-cost-requirements-2026",
    cbeci="https://ccaf.io/cbnsi/about/cbeci",
    eth_consensus="https://ethereum.org/de/developers/docs/consensus-mechanisms/",
    eth_pos="https://ethereum.org/de/developers/docs/consensus-mechanisms/pos/",
    eth_mev="https://ethereum.org/de/developers/docs/mev/",
    eth_bridges="https://ethereum.org/de/bridges/",
    eip1559="https://eips.ethereum.org/EIPS/eip-1559",
    eip155="https://eips.ethereum.org/EIPS/eip-155",
    eip5564="https://www.quicknode.com/guides/ethereum-development/wallets/how-to-use-stealth-addresses-on-ethereum-eip-5564",
    bip39="https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki",
    tendermint="https://arxiv.org/abs/1807.04938",
    bls="https://hackmd.io/@benjaminion/bls12-381",
    bls_wiki="https://en.wikipedia.org/wiki/BLS_digital_signature",
    merkle="https://www.bitpanda.com/de/academy/was-sind-merkle-trees-und-warum-sind-sie-wichtig",
    argon2="https://www.rfc-editor.org/rfc/rfc9106",
    vrf="https://www.rfc-editor.org/rfc/rfc9381",
    dandelion="https://arxiv.org/abs/1805.11060",
    mldsa="https://csrc.nist.gov/pubs/fips/204/final",
    pqk="https://www.computerweekly.com/de/definition/Post-Quanten-Kryptografie-PQK",
    atomic="https://glossary.blockstream.com/atomic-swap",
    htlc="https://en.bitcoin.it/wiki/Hashed_Timelock_Contracts",
    mica="https://eur-lex.europa.eu/eli/reg/2023/1114/oj?locale=de",
    esma="https://www.esma.europa.eu/publications-and-data/interactive-single-rulebook/mica",
    amf="https://www.amf-france.org/en/news-publications/news/end-mica-transitional-period-esma-sets-out-its-expectations-professionals-and-warns-retail-investors",
    bmf="https://www.bundesfinanzministerium.de/Content/DE/Downloads/BMF_Schreiben/Steuerarten/Einkommensteuer/2025-03-06-einzelfragen-kryptowerte.html",
    vz="https://www.verbraucherzentrale.de/wissen/geld-versicherungen/sparen-und-anlegen/bitcoin-vom-alternativen-zahlungsmittel-zum-spekulationsobjekt-11641",
    test="https://www.test.de/Kryptowaehrungen-Finger-weg-von-diesen-Angeboten-5282801-0/",
    metamask="https://support.metamask.io/configure/networks/how-to-add-a-custom-network-rpc",
    repro="https://reproducible-builds.org/",
    kani="https://rust-training.ferrous-systems.com/latest/book/kani",
    libp2p="https://docs.libp2p.io/",
    mit="https://opensource.org/license/mit",
    repo="https://github.com/Safehaigit/toycoin",
    market="https://dailyforex.com/forex-technical-analysis/2026/10/crypto-overview-18-october-2026/250826",
    market2="https://www.tokenpost.com/news/investing/35318",
    ethtests="https://github.com/ethereum/tests",
    cg_charts="https://www.coingecko.com/en/charts",
    dead="https://forklog.com/en/news/coingecko-reports-record-token-mortality-in-2025",
    bridgehacks="https://decrypt.co/106718/cross-chain-crypto-bridge-hacks-hit-2-billion-2022-chainalysis",
)

# ================================================================ Titel
story.append(Spacer(1, 0.3 * cm))
story.append(Paragraph("ToyChain und ToyCoin (TOY)", title))
story.append(Spacer(1, 4))
story.append(Paragraph("Ein schlankes, fair gestartetes Zahlungsnetz,<br/>an dem jeder ohne Guthaben teilnehmen kann",
                       subtitle))
story.append(Spacer(1, 10))
story.append(Paragraph("ToyChain-Projekt · " + L("github.com/Safehaigit/toycoin", U["repo"]), center))
story.append(Paragraph("Vorläufiges Whitepaper · Entwurf 0.4 · 20. Oktober 2026", center))
story.append(Spacer(1, 12))
BOX("<b>Stand: vor der Testphase.</b> ToyChain existiert bisher als Konzept und als erste Programmbausteine. Fertig "
    "und getestet sind die Regeln der Geldentstehung, die Kryptografie für Konten und Unterschriften sowie Konten, "
    "Transaktionen, Gebühren und Schutzregeln (Meilensteine M1 bis M3 von 13). Es gibt noch "
    "<b>kein Testnetz, kein Hauptnetz und keine handelbaren TOY</b>. Alle Zahlen sind Startwerte, die im Testnetz "
    "gemessen und angepasst werden. Dieses Dokument ist keine Anlageberatung und kein Angebot zum Kauf von irgendetwas. "
    "Guthaben aus dem Testnetz haben keinen Wert und werden nie in ein späteres Hauptnetz übernommen. "
    "Unterstrichene blaue Begriffe sind Web-Links zu weiterführenden Erklärungen. Wie man ToyChain später als Nutzer "
    "einrichtet, zeigt die getrennte <b>Kurzanleitung</b> (ToyChain-Kurzanleitung-v0.4.pdf).")

P("<b>Zusammenfassung.</b> ToyChain ist eine eigenständige Blockchain mit der Währung ToyCoin (TOY), höchstens 100 "
  "Millionen Stück. Sie startet völlig fair: Der erste Block enthält kein einziges TOY, es gibt keine Reserve für "
  "Gründer, keinen Vorverkauf und keine Sonderzuteilung. Alle TOY entstehen nach und nach durch eine feste, öffentlich "
  "nachprüfbare Regel, ähnlich wie bei " + L("Bitcoin", U["btc_pdf"]) + ". Anders als bei Bitcoin entsteht Sicherheit "
  "nicht durch Stromverbrauch, sondern durch Teilnehmer, die TOY als Pfand binden und Blöcke gemeinsam unterschreiben. "
  "Ein Block ist nach etwa einer Minute endgültig. Jeder Knoten kommt mit 256 MB Daten aus und läuft auch auf alten "
  "Handys und auf dem Raspberry Pi. Mitmachen ist ohne Guthaben möglich, wer mehr beiträgt, bekommt schrittweise mehr "
  "Rechte. Bewusst verzichtet ToyChain auf Smart Contracts, Token und NFTs: Jede Funktion ist fest eingebaut, einfach "
  "prüfbar und kann keine fremden Guthaben antasten. Konten-Adressen sind dieselben wie bei Ethereum, so dass bekannte "
  "Wallets weiter funktionieren. Am Ende steht eine ehrliche Einschätzung der Chancen am Kryptomarkt.", abstract)

story.append(Spacer(1, 4))
toc_items = ["1 Einleitung", "2 ToyChain auf einen Blick", "3 Grundbegriffe einfach erklärt", "4 Aufbau",
             "5 Fairer Start und Geldmenge", "6 Mitmachen: Stufen, Kontingent, Lotterie", "7 Einigung im Netz",
             "8 Die Startphase", "9 Schlanke Knoten für alte Geräte", "10 Sicherheit für Nutzer",
             "11 Wallet, ToyScan und andere Ketten", "12 Anwendungsmöglichkeiten", "13 Ähnlichkeiten und Unterschiede",
             "14 Vorteile", "15 Risiken und offene Fragen", "16 Chancen am Kryptomarkt", "17 Stand und Fahrplan",
             "18 Fazit", "Anhang: Startwerte, Links, Quellen, Änderungen"]
half = (len(toc_items) + 1) // 2
tt = Table([[Paragraph(a, toc), Paragraph(b, toc)] for a, b in
            zip(toc_items[:half], toc_items[half:] + [""] * (2 * half - len(toc_items)))],
           colWidths=[8.2 * cm, 8.2 * cm])
tt.setStyle(TableStyle([("TOPPADDING", (0, 0), (-1, -1), 0.5), ("BOTTOMPADDING", (0, 0), (-1, -1), 0.5),
                        ("LEFTPADDING", (0, 0), (-1, -1), 1.1 * cm)]))
story.append(Paragraph("Inhalt", ParagraphStyle("ti", parent=h2, leftIndent=1.1 * cm)))
story.append(tt)

# ================================================================ 1
H("Einleitung")
P("Bitcoin hat 2008 gezeigt, dass Menschen sich ohne Bank darauf einigen können, wem welches Geld gehört. Seitdem sind "
  "Millionen Kryptowerte entstanden. Bei vielen davon haben Gründer und frühe Geldgeber einen großen Teil der Coins "
  "schon vor dem Start erhalten (Premint). Andere brauchen große Rechenzentren, viel Strom oder Festplatten mit "
  "hunderten Gigabyte, so dass normale Menschen das Netz kaum selbst prüfen können. Die vollständige Bitcoin-Kette ist "
  "inzwischen rund 760 GB groß. Und viele Ketten sind mit Smart Contracts so vielseitig geworden, dass Fehler in "
  "fremdem Code regelmäßig zu Verlusten führen.")
P("ToyChain versucht, die Grundidee von Bitcoin, also feste Menge, fairer Start und offene Regeln, mit heutigen "
  "Verfahren schlank und alltagstauglich umzusetzen. Die Leitfragen sind: Kann jeder mit einem alten Gerät mitmachen? "
  "Kann jeder die Regeln selbst nachprüfen? Bleibt das eigene Guthaben unter allen Umständen das eigene? Dieses "
  "Whitepaper erklärt die Ideen in einfacher Sprache. Die genauen Regeln stehen in der technischen Spezifikation "
  "(Version 0.3) im " + L("Projekt-Repository", U["repo"]) + ". Es ist während der Entwicklung noch privat und soll "
  "spätestens mit dem öffentlichen Testnetz freigegeben werden.")
MORE(L("Bitcoin-Whitepaper (englisch)", U["btc_pdf"], "Grundlagen"),
     L("Bitcoin-Whitepaper auf Deutsch", U["btc_de"], "Grundlagen"),
     L("Größe eines Bitcoin-Knotens 2026", U["btc_size"], "Grundlagen"))

# ================================================================ 2
H("ToyChain auf einen Blick")
T([["Eigenschaft", "ToyChain (geplante Startwerte)"],
   ["Währung", "ToyCoin, Kürzel TOY, teilbar in 18 Dezimalstellen"],
   ["Höchstmenge", "100.000.000 TOY, im Code als unveränderliche Regel festgeschrieben"],
   ["Start", "Totaler Fair Launch: erster Block ohne Guthaben, kein Premint, keine Reserve"],
   ["Neue TOY", "je Epoche (mindestens 24 Stunden) 0,0505 % der Restmenge, angepasst mit Faktor m zwischen 0,5 und "
                "2,0; erstes Jahr rund 16,84 Mio., Hälfte nach rund 3,76 Jahren, 99 % nach rund 25 Jahren"],
   ["Einigung (Konsens)", "Proof-of-Stake mit Komitee von höchstens 128 Validatoren, gemeinsame Unterschrift, "
                          "endgültig nach 1 Block"],
   ["Blockzeit", "60 Sekunden, bei vollem Wartebereich früher, frühestens nach 10 Sekunden"],
   ["Speicher je Knoten", "Ziel 256 MB, auch auf alten Android-Geräten und dem Raspberry Pi"],
   ["Konten", "wie Ethereum: gleiche Adressen, gleiche 12- oder 24-Wörter-Phrase"],
   ["Funktionen", "nur fest eingebaute Transaktionsarten, keine Smart Contracts, keine Token, keine NFTs"],
   ["Gebühren", "10 bis 100 kostenlose Transaktionen je Konto und Epoche (mindestens ein Tag), darüber eine kleine "
                "Gebühr (rund 0,00012 TOY je Überweisung) oder ein Rechenlos"],
   ["Netz-Kennung", "Chain-ID 7630713 (Hauptnetz, „toy“), 7630714 (Testnetz), 7630715 (lokales Probenetz)"],
   ["Programmiersprache", "Rust, ein gemeinsamer Kern für alle Plattformen"],
   ["Lizenz", L("MIT", U["mit"]) + " (freie Software)"]],
  [4.0, 12.4])

# ================================================================ 3
H("Grundbegriffe einfach erklärt")
P("Wer noch nie mit Kryptowährungen zu tun hatte, findet hier die wichtigsten Begriffe. Die folgenden Abschnitte "
  "bauen darauf auf. Ein Klick auf einen blauen Begriff führt zu einer guten Erklärung im Netz.")
T([["Begriff", "Einfach gesagt"],
   ["Blockchain", "Ein gemeinsames Kassenbuch, das viele Computer gleichzeitig führen. Neue Einträge werden in Paketen "
                  "(Blöcken) angehängt, jeder Block verweist fälschungssicher auf den vorigen."],
   ["Knoten", "Ein Gerät mit der ToyChain-Software. Es prüft selbst, ob alle Einträge den Regeln entsprechen, und muss "
              "dafür niemandem vertrauen."],
   ["Wallet", "Die App, die Ihre Schlüssel verwahrt und Überweisungen unterschreibt. Bei ToyChain ist die Wallet TOYPAY "
              "zugleich ein Knoten."],
   [L("Schlüssel und Phrase", U["bip39"], "Konten und Schlüssel"),
    "Ein geheimer Schlüssel beweist, dass Ihnen ein Konto gehört. Er lässt sich aus 12 oder 24 Wörtern "
    "wiederherstellen. Wer die Wörter kennt, kann über das Geld verfügen."],
   ["Signatur", "Eine digitale Unterschrift. Jeder kann prüfen, dass sie echt ist, aber niemand kann sie ohne den "
                "Schlüssel fälschen."],
   [L("Konsens", U["eth_consensus"], "Konsens"), "Das Verfahren, mit dem sich alle Knoten auf denselben nächsten "
                                                 "Block einigen."],
   [L("Stake", U["eth_pos"], "Konsens"), "TOY, die man freiwillig als Pfand bindet. Wer sich an die Regeln hält, "
                                         "bekommt Belohnungen. Wer nachweislich betrügt, verliert einen Teil des Pfands."],
   ["Validator", "Ein Knoten mit Stake, der Blöcke vorschlägt und mitunterschreibt."],
   [L("Merkle-Baum", U["merkle"], "Speicher"), "Eine Art Inhaltsverzeichnis aus Prüfsummen. Mit einem kleinen Beweis "
                                              "lässt sich zeigen, dass ein Eintrag dazugehört, ohne alles zu speichern."],
   ["Epoche", "Ein Abschnitt der Kette von mindestens 1.440 Blöcken und mindestens 24 Stunden. Viele Regeln zählen in "
              "Epochen, etwa die Ausgabe neuer TOY."],
   ["Testnetz", "Eine Probe-Kette, deren Coins nichts wert sind. Dort werden Fehler gesucht, bevor es ernst wird."]],
  [3.6, 12.8])

# ================================================================ 4
H("Aufbau")
P("ToyChain besteht aus übereinanderliegenden Schichten. Jede hat eine klar umrissene Aufgabe. Alle Schichten stecken "
  "in einem gemeinsamen Rust-Programm, das auf Android, Windows, Linux und dem Raspberry Pi gleich funktioniert. So "
  "muss nichts doppelt geschrieben und doppelt geprüft werden.")
FIG("abbildungen/f_arch.png", "Die Schichten von ToyChain. Die Beweis-Schicht (gestrichelt) folgt in Phase 2.", 14.8)
MORE(L("libp2p, das Netzwerk-Fundament", U["libp2p"], "Technik"),
     L("Dandelion++: Wer hat gesendet?", U["dandelion"], "Technik"))

# ================================================================ 5
H("Fairer Start und Geldmenge")
H2("5.1 Kein einziges TOY im ersten Block")
P("Der erste Block (Genesis-Block) enthält bei ToyChain <b>keinerlei Guthaben</b>. Niemand, auch nicht die Entwickler, "
  "besitzt beim Start TOY. Es gibt keinen Vorverkauf, keine Reserve und keinen Gründeranteil. Das ist strenger als bei "
  "den meisten neuen Projekten und entspricht dem Geist von Bitcoin. Diese Regel gehört zur „Verfassung“ (Abschnitt "
  "10.3) und kann durch kein Update geändert werden.")
H2("5.2 Wie neue TOY entstehen")
P("In jeder Epoche entsteht ein fester Bruchteil der noch nicht ausgegebenen Menge: 0,0505 % der Restmenge. In der "
  "ersten Epoche sind das genau 50.500 TOY. Weil die Restmenge schrumpft, sinkt die Ausgabe gleichmäßig, statt wie bei "
  "Bitcoin alle vier Jahre sprunghaft auf die Hälfte zu fallen. Nach rund 3,76 Jahren ist die Hälfte ausgegeben, nach "
  "rund 25 Jahren etwa 99 %. Die Höchstmenge wird nie erreicht und nie überschritten. Das ist bereits durch Tests "
  "belegt: Selbst mit doppelter Ausgabegeschwindigkeit über 100 Jahre bleibt die Summe darunter.")
FIG("abbildungen/f_emission.png", "Ausgegebener Anteil der Höchstmenge über die Zeit. ToyChain sinkt gleichmäßig, Bitcoin "
                      "halbiert in Stufen. Nach 25 Jahren sind beide bei rund 99 %.")
FIG("abbildungen/f_per_year.png", "Neue TOY je Jahr nach Plan (m = 1). Jedes Jahr entstehen etwa 17 % weniger als im Vorjahr.")
MORE(L("Wie Bitcoin seine Menge begrenzt", U["btc_supply"], "Geldmenge"))
H2("5.3 Der Faktor m: angepasst, aber gezügelt")
P("Die Ausgabe wird mit einem Faktor m zwischen 0,5 und 2,0 multipliziert. Er steigt leicht, wenn das Netz "
  "<i>wirklich</i> genutzt wird (bezahlte oder durch Rechenarbeit gedeckte Transaktionen) und wenn viele Validatoren "
  "zuverlässig mitunterschreiben. Kostenlose Transaktionen zählen nicht, sonst ließe sich die Ausgabe mit Spam "
  "hochtreiben. Ein Zeitplan-Anteil holt Abweichungen wieder ein, und m darf sich je Epoche nur um 0,05 ändern. In "
  "Simulationen weicht die Ausgabe im ersten Jahr um höchstens etwa 2,2 % vom Plan ab, auf Dauer um höchstens "
  "etwa 2,5 %. m verschiebt also nur, "
  "<i>wann</i> TOY entstehen, nicht <i>wie viele</i> insgesamt.")
H2("5.4 Wer die neuen TOY bekommt")
P("Jede Ausgabe wird fest aufgeteilt: <b>50 %</b> an alle, die das Netz absichern, <b>40 %</b> an eine "
  "Arbeits-Lotterie, an der jedes Gerät ohne Guthaben teilnehmen kann (Abschnitt 6.3), und <b>10 %</b> an einen "
  "Erstattungs-Topf, der die kostenlosen Transaktionen aller bezahlt. In der Startphase geht der Absicherungs-Anteil an "
  "die Start-Validatoren und sinkt bis auf null. Was sie nicht bekommen, fließt in die Lotterie. Gerechnet wird "
  "ausschließlich mit ganzen Zahlen in der kleinsten Einheit (10<super>-18</super> TOY). Rundungsreste gehen in den "
  "Erstattungs-Topf, so dass in jeder Epoche genau die geplante Menge entsteht. Der Erstattungs-Topf behält aber "
  "höchstens so viel, wie die kostenlosen Transaktionen von 30 Epochen im äußersten Fall kosten könnten (beim Start "
  "rund 2.800 TOY). Was darüber hinausgeht, fließt zu Beginn jeder Epoche ebenfalls in die Arbeits-Lotterie. So "
  "liegt kein TOY auf Dauer ungenutzt in einem Topf.")
FIG("abbildungen/f_split.png", "Aufteilung neuer TOY. In der Startphase wandert der Lohn der Start-Validatoren schrittweise in "
                   "die Arbeits-Lotterie, nach der Übergabe erhalten ihn alle Stake-Teilnehmer.")
H2("5.5 Es wird nie TOY vernichtet")
P("Viele Ketten „verbrennen“ Gebühren oder Strafen, Ethereum zum Beispiel einen Teil jeder Gebühr nach "
  + L("EIP-1559", U["eip1559"], "Gebühren") + ". ToyChain nicht: Strafen und Spenden fließen in den Erstattungs-Topf "
  "und kommen so den Nutzern zugute. Eine Prüfregel (I0) stellt in <b>jedem</b> Block sicher, dass die Summe aller "
  "Guthaben genau der Summe aller ausgegebenen TOY entspricht. Ein Block, der das verletzt, ist ungültig.")

# ================================================================ 6
H("Mitmachen: Stufen, Kontingent und Arbeits-Lotterie")
H2("6.1 Teilnahme-Stufen")
P("Jeder entscheidet selbst, wie viel er beiträgt. Die Wallet misst beim ersten Start das Gerät und schlägt eine "
  "passende Stufe vor. Eine höhere Stufe bietet sie nur an, wenn das Gerät sie auch schafft.")
FIG("abbildungen/f_tiers.png", "Die fünf Teilnahme-Stufen. Schon Stufe 0 braucht kein Guthaben.", 15.0)
P("Gebundenes TOY bleibt Eigentum des Nutzers und lässt sich mit 21 Epochen Wartezeit wieder lösen. Damit niemand "
  "übermächtig wird, zählt der Stake eines Teilnehmers höchstens bis 5 % (ab 100 Validatoren 1 %) des gesamten "
  "Stakes. Nach der Startphase geht ein Fünftel des Absicherungs-Anteils zu gleichen Teilen an jedes Komitee-Mitglied, "
  "der Rest linear nach Stake. Beides gibt es nur im Verhältnis zur tatsächlich geleisteten Mitarbeit: Wer 90 % der "
  "Blöcke mitunterschreibt, bekommt 90 %. Weil der Anteil nach Stake linear ist, bringt das Aufteilen auf viele Konten "
  "keinen Vorteil.")
MORE(L("Was ist Proof-of-Stake? (ethereum.org)", U["eth_pos"], "Konsens"))
H2("6.2 Kostenlose Transaktionen und Gebühren")
P("Jedes aktivierte Konto hat je Epoche ein kostenloses Kontingent: 100 Transaktionen, wenn das Netz leer ist, "
  "mindestens 10 bei Hochbetrieb. Gemessen wird, wie voll die Blöcke der vorigen Epoche waren. Für den Alltag reicht "
  "das. Ein Konto muss dafür mindestens 100 Blöcke (rund 100 Minuten) alt sein. Die Wallet stellt die Gebühr dann "
  "einfach auf null. Kostenlose Transaktionen belegen höchstens die Hälfte eines Blocks und werden aus dem "
  "Erstattungs-Topf bezahlt, in den jeder auch freiwillig spenden kann. Je Epoche gibt der Topf höchstens ein "
  "Dreißigstel seines Bestands dafür aus.")
P("Darüber zahlt man eine kleine Gebühr nach Größe der Transaktion. Beim Start sind es 0,000001 TOY je Byte, für eine "
  "normale Überweisung also rund 0,00012 TOY. Bei vollen Blöcken steigt diese Grundgebühr je Block um höchstens ein "
  "Achtel, bei leeren sinkt sie wieder, ähnlich wie bei " + L("EIP-1559", U["eip1559"]) + ". Wer viele Transaktionen "
  "in denselben Block packt, zahlt mehr: Je zehn weitere verdoppeln die Gebühr, höchstens auf das 1.024-Fache. Statt "
  "zu zahlen, kann man ein Rechenlos (Abschnitt 6.3) anhängen, dann ist die Transaktion ganz gebührenfrei. Wer mehr als "
  "1.000 Transaktionen am Tag sendet, muss mindestens die Hälfte des Überschusses mit Rechenlosen decken. So bleibt das "
  "Netz für normale Nutzer kostenlos, während Spam teuer wird. Von jeder bezahlten Gebühr gehen 40 % an eine Lotterie "
  "für aktive Teilnehmer, 40 % in den Erstattungs-Topf und 20 % an den Block-Vorschlagenden.")
H2("6.3 Die Arbeits-Lotterie")
P("Damit auch Menschen ohne TOY an neue Coins kommen, gibt es eine Lotterie für Rechenarbeit. Das Gerät berechnet Lose "
  "mit dem Verfahren " + L("Argon2id", U["argon2"], "Technik") + ", das bewusst viel Arbeitsspeicher (16 MiB) braucht. "
  "Dadurch haben Spezialhardware und Grafikkarten-Farmen kaum Vorteile gegenüber einem Handy oder PC. Je Block werden "
  "bis zu 64 Gewinner fair ausgelost, jeder Gewinn beträgt mindestens 0,2 TOY. Ein Los gilt nur für die eigene "
  "Adresse und den laufenden Tag und ist mit dem eigenen Schlüssel unterschrieben. Gezogen wird erst zwei Blöcke später, "
  "damit niemand, der einen Block baut, das Ergebnis steuern kann. Höchstens 16 Lose passen in einen Block, die "
  "Schwierigkeit passt sich täglich an. Wer mehr rechnet, hat mehr Lose, die "
  "erwartete Belohnung ist proportional zur Arbeit. Schon ein einziger Gewinn reicht, um ein Konto zu aktivieren: "
  "0,1 TOY werden als Pfand gebunden (Abschnitt 9), der Rest ist frei verfügbar. Gerechnet wird nur, wenn das Gerät es erlaubt: am Ladegerät, im "
  "WLAN und solange es kühl bleibt. Zum Vergleich: Das Bitcoin-Netz verbraucht laufend so viel Strom wie ein ganzes "
  "Land, wie der " + L("Cambridge-Stromindex", U["cbeci"], "Energie") + " zeigt.")

# ================================================================ 7
H("Einigung im Netz")
H2("7.1 So entsteht ein Block")
P("Ein Komitee aus höchstens 128 Validatoren wird je Epoche neu zusammengesetzt. Für jeden Block wird per "
  "überprüfbarem Zufall (" + L("VRF", U["vrf"], "Technik") + ") einer ausgewählt, der den Block vorschlägt. Weil es "
  "für den VRF-Standard RFC 9381 noch keine geprüfte Rust-Bibliothek gibt, nutzt das Testnetz eine VRF aus eindeutigen "
  "BLS-Unterschriften. Dasselbe Prinzip verwendet Ethereum für seinen Zufall. Vor einem Hauptnetz wird neu "
  "entschieden. Die "
  "anderen stimmen in zwei Phasen ab (Vorabstimmung und Bestätigung), nach einem bewährten Verfahren im Stil von " +
  L("Tendermint", U["tendermint"], "Konsens") + ". Sobald zwei Drittel zugestimmt haben, ist der Block "
  "<b>endgültig</b>. Er kann nicht mehr zurückgenommen werden. Bei Bitcoin wartet man dagegen üblicherweise etwa eine "
  "Stunde, bis eine Zahlung als sicher gilt. Alle Stimmen werden mit " + L("BLS-Signaturen", U["bls_wiki"], "Technik") +
  " zu einer einzigen, nur 96 Byte großen Unterschrift zusammengefasst, die auch ein Handy in Millisekunden prüfen kann.")
FIG("abbildungen/f_block.png", "Ablauf eines Blocks in fünf Schritten. Nach Schritt 5 ist die Zahlung endgültig.")
H2("7.2 Faire Reihenfolge")
P("Wer einen Block baut, kann die Reihenfolge der Transaktionen nicht frei wählen. Sie ist fest vorgegeben: nach "
  "Gebühr, bei Gleichstand nach einer Zufallszahl, die erst mit dem Block feststeht. Jeder Knoten prüft das nach. "
  "Später soll eine Aufnahme-Pflicht dafür sorgen, dass Transaktionen, die viele Validatoren sehen, auch wirklich in "
  "den nächsten Block kommen. Weil es auf ToyChain keine Tauschbörsen-Verträge gibt, entfällt das bei Ethereum "
  "bekannte " + L("„Vordrängeln“ (MEV)", U["eth_mev"], "Konsens") + " weitgehend.")
H2("7.3 Strafen, die zum Schaden passen")
P("Wer zwei widersprüchliche Stimmen für denselben Block abgibt (Doppelsignatur), verliert einen Teil seines Stakes. "
  "Das passiert oft aus Versehen, etwa wenn derselbe Schlüssel auf zwei Geräten läuft. Deshalb hängt die Strafe davon "
  "ab, wie viele es gleichzeitig tun. Die Software schützt zusätzlich vor Pannen: Sie merkt sich dauerhaft, was sie "
  "unterschrieben hat, und bleibt nach einem Gerätewechsel zunächst stumm, wenn ihr Schlüssel woanders schon aktiv "
  "ist. In der Startphase ist die Kürzung auf 5 % begrenzt.")
FIG("abbildungen/f_slashing.png", "Kürzung bei Doppelsignatur: max(1 %, 3 × gleichzeitig betroffener Anteil), höchstens 100 %.")

# ================================================================ 8
H("Die Startphase")
P("Ein fairer Start ohne Premint hat bei Proof-of-Stake ein Henne-Ei-Problem: Am Anfang hat niemand TOY, also auch "
  "niemand Stake, mit dem er das Netz absichern könnte. ToyChain löst das offen und zeitlich begrenzt:")
B(["<b>21 Start-Validatoren</b>, betrieben von Freiwilligen mit möglichst verschiedenen Betreibern und Standorten, "
   "sichern das Netz mit gleichem Gewicht. Ihre Namen und Einnahmen sind in ToyScan öffentlich.",
   "<b>Sinkender Lohn:</b> Anfangs erhalten sie zusammen die Hälfte aller neuen TOY. Der Anteil sinkt Tag für Tag "
   "gleichmäßig und ist nach 180 Tagen null. Gezählt wird dabei in <i>Planzeit</i>: Jede Pause zwischen zwei Blöcken "
   "zählt höchstens 10 Minuten, damit ein Stillstand die Uhr nicht vorstellt. Ein Deckel verhindert, dass schnellere Ausgabe oder absichtlich langsame Blöcke mehr Lohn "
   "bringen. Insgesamt sind es höchstens rund 2,22 Mio. TOY (etwa 2,2 % der Höchstmenge).",
   "<b>Gebundener Lohn:</b> Jeder Lohn bleibt 365 Epochen (rund ein Jahr) gesperrt und haftet bei Fehlverhalten mit.",
   "<b>Strenge Übergabe:</b> Die Startphase endet frühestens nach 90 Tagen und erst, wenn mindestens 21 neue "
   "Validatoren mit je 1.000 TOY seit 30 Epochen gebunden sind und zusammen zwei Drittel des wirksamen Stakes halten. "
   "Danach verlieren die Start-Validatoren jeden Sonderstatus. Gelingt die Übergabe in 180 Tagen nicht, sichern sie "
   "das Netz ohne Lohn weiter, bis es so weit ist."])
FIG("abbildungen/f_bootstrap.png", "Lohn der Start-Validatoren nach Tagen und das Zeitfenster für die Übergabe.")
P("Ehrlich gesagt: Die Startphase ist der Teil, in dem ToyChain am meisten Vertrauen in wenige Menschen braucht. Kein "
  "Protokoll kann beweisen, dass „neue“ Validatoren wirklich andere Menschen sind. Die Regeln machen eine Übernahme "
  "aber teuer und für alle sichtbar.")

# ================================================================ 9
H("Schlanke Knoten für alte Geräte")
P("Eine Bitcoin- oder Ethereum-Kette vollständig zu speichern, braucht heute hunderte Gigabyte. ToyChain setzt sich "
  "ein festes Ziel von <b>256 MB je Standard-Knoten</b>. Das gelingt, weil ein Knoten nur den aktuellen Kontostand "
  "aller aktiven Konten und ein kurzes Fenster der letzten Blöcke speichert. Ältere Blöcke bewahren freiwillige "
  "Archiv-Knoten auf. In einer späteren Phase soll die gesamte Vorgeschichte in einen kleinen kryptografischen Beweis "
  "„gefaltet“ werden, so dass auch neue Knoten nichts mehr vertrauen müssen. Bis dahin starten neue Knoten von "
  "Zwischenständen (Checkpoints), die mindestens drei voneinander unabhängige Stellen unterschrieben haben.")
FIG("abbildungen/f_budget.png", "So verteilen sich die 256 MB. Jede Speicherstruktur bekommt einen eigenen Größentest.")
P("Damit trotzdem 100 Millionen Konten und mehr Platz finden, gibt es <b>aktive und ruhende Konten</b>. Lange nicht "
  "benutzte Konten werden bei Bedarf aus dem Speicher ausgelagert, aber <b>nie gelöscht</b>. Ihr Inhalt ist durch "
  "einen " + L("Merkle-Beweis", U["merkle"]) + " von knapp 1 KB gesichert, den die Wallet selbst aufbewahrt. Mit der "
  "nächsten Transaktion wird das Konto automatisch wieder aufgeweckt, mit vollem Guthaben.")
FIG("abbildungen/f_dormant.png", "Aktive und ruhende Konten. Kein Konto geht verloren.", 15.0)
P("Damit niemand den Speicher mit Millionen leerer Konten füllt, werden beim ersten Eingang 0,1 TOY als "
  "<b>Aktivierungs-Pfand</b> gebunden. Das Pfand bleibt Eigentum des Nutzers, wird nie gekürzt und kommt beim "
  "Auflösen des Kontos vollständig zurück. Es darf später nur sinken, höchstens bis 0,01 TOY.")

# ================================================================ 10
H("Sicherheit für Nutzer")
H2("10.1 Freiwilliger Konto-Schutz")
P("Jeder kann sein eigenes Konto zusätzlich absichern. Diese Einstellungen gelten nur, weil der Eigentümer sie selbst "
  "gesetzt hat. Niemand sonst kann sie setzen.")
B(["<b>Tresor-Verzögerung:</b> Größere Abbuchungen werden erst nach einer Wartezeit wirksam und lassen sich in dieser "
   "Zeit abbrechen. Ein Dieb kann das Konto also nicht sofort leeren.",
   "<b>Notfall-Schlüssel und Sicherheitsadresse:</b> Ein zweiter, getrennt aufbewahrter Schlüssel kann nichts ausgeben, "
   "aber wartende Abbuchungen stoppen und alles an eine vorher festgelegte sichere Adresse retten.",
   "<b>Selbst-Sperre:</b> Handy verloren? Das eigene Konto lässt sich bis zu 30 Epochen sperren.",
   "<b>Erben-Adresse:</b> Bleibt ein Konto sehr lange ohne jede Aktivität (mindestens 180 Epochen), kann eine selbst "
   "gewählte Erben-Adresse es übernehmen. Der Eigentümer wird vorher mehrfach erinnert und kann noch 30 Epochen lang "
   "abbrechen."])
P("Ehrlich gesagt: Wer das Handy samt Schlüssel stiehlt, hat ohne Notfall-Schlüssel dieselben Rechte wie der "
  "Eigentümer. Die Wallet empfiehlt deshalb, einen Notfall-Schlüssel auf einem zweiten Gerät oder auf Papier anzulegen.")
FIG("abbildungen/f_vault.png", "Tresor-Verzögerung: Während der Wartezeit kann der Eigentümer eine verdächtige Abbuchung stoppen.",
    15.0)
H2("10.2 Schutzregeln mit Augenmaß")
P("Angriffe sollen früh gestoppt werden, ohne ehrliche Nutzer zu treffen. Deshalb gibt es drei Ebenen. Die Kette "
  "selbst ahndet nur <b>eindeutig nachweisbare</b> Verstöße, etwa eine Doppelsignatur oder einen gefälschten Block. "
  "Die Folge ist eine Sperrzeit, die Rechte entzieht, zum Beispiel das kostenlose Kontingent, <b>nie aber "
  "Guthaben</b>. Wiederholungen verdoppeln die Sperrzeit bis höchstens 64 Epochen, nach 30 Tagen ohne Verstoß "
  "beginnt die Zählung von vorn. Zwei Beispiele: Wer ein Rechenlos mit falscher Arbeit unterschreibt, darf für den Rest "
  "des Tages und einen weiteren Tag keine Lose einreichen. Wer an einem Tag mehr als 20 neue Konten anlegt, verliert "
  "für 7 Tage das kostenlose Kontingent. Überweisen gegen Gebühr bleibt in jedem Fall möglich. Das ist seit "
  "Meilenstein M3 für jede Regel mit einem Test belegt, jeweils für den Fall, dass sie greift, und den Fall, dass "
  "sie nicht greift.")
FIG("abbildungen/f_levels.png", "Die drei Ebenen der Schutzregeln. Nur Ebene A wirkt in der Kette, und nur auf Rechte.", 15.0)
H2("10.3 Die Verfassung")
P("Einige Regeln sind so grundlegend, dass sie fest im Code stehen und durch kein Update und keine spätere Abstimmung "
  "geändert werden dürfen: die Höchstmenge von 100 Mio. TOY, der Start ohne Guthaben, die Ausgabeformel, das Verbot, "
  "TOY zu vernichten, die Mindestdauer einer Epoche, der Verzicht auf Token und Smart Contracts und vor allem: "
  "<b>Keine Regel darf TOY von einem Konto nehmen, einfrieren oder umverteilen.</b> Ehrlich gesagt kann jeder, der "
  "eine eigene Software verbreitet, eine Kette abspalten. Wallet und ToyScan prüfen die Verfassung deshalb selbst und "
  "warnen, wenn eine Abspaltung sie bricht. Jede Programmversion soll " + L("reproduzierbar", U["repro"], "Technik") +
  " gebaut werden, damit jeder prüfen kann, dass sie wirklich aus dem veröffentlichten Code stammt.")
H2("10.4 Bewährte Kryptografie")
P("ToyChain erfindet keine eigenen Verschlüsselungsverfahren. Verwendet werden weit verbreitete, geprüfte Standards: "
  "secp256k1 und Keccak-256 wie bei Ethereum für Konten, " + L("BIP-39", U["bip39"]) + " für die Wörter-Phrase, "
  + L("BLS12-381", U["bls"], "Technik") + " für die gemeinsamen Unterschriften der Validatoren, BLAKE3 für Prüfsummen, "
  "Argon2id für die Rechenarbeit und für die Verschlüsselung der Wallet auf dem Gerät. Jede Signatur enthält die "
  "Kennung der Kette (" + L("EIP-155", U["eip155"], "Konten und Schlüssel") + "), damit sie nicht auf einer anderen "
  "Kette wiederverwendet werden kann. Alle Verfahren sitzen hinter austauschbaren Schnittstellen. Ein freiwilliger "
  "Zusatzschlüssel nach dem NIST-Standard " + L("ML-DSA", U["mldsa"], "Technik") + " soll auch "
  + L("Quantencomputern", U["pqk"], "Technik") + " standhalten. Die wichtigsten Rechenregeln werden zusätzlich mit dem "
  "Beweis-Werkzeug " + L("Kani", U["kani"], "Technik") + " mathematisch geprüft, also nicht nur an Beispielen, sondern "
  "für alle möglichen Zahlen. Bewiesen ist bereits: Eine Epoche schüttet nie mehr aus als die Restmenge, und die Summe "
  "aller TOY bleibt immer höchstens 100 Mio. (Abschnitt 17). Alle Bausteine sind mit offiziellen Testdaten der "
  "jeweiligen Standards geprüft, etwa für Phrasen, Adressen, Unterschriften und Argon2id. Ethereum-Überweisungen aus der "
  "offiziellen " + L("Testsammlung", U["ethtests"], "Technik") + " ergeben bei ToyChain exakt denselben Absender.")

# ================================================================ 11
H("TOYPAY Wallet, ToyScan und andere Ketten")
P("Die <b>TOYPAY Wallet</b> ist Geldbörse und Knoten in einem. Sie läuft auf Android ab etwa 1 GB Arbeitsspeicher, auf "
  "Windows, Linux und dem Raspberry Pi. Ein Energie-Modus regelt automatisch, wie viel das Gerät beiträgt, je nach "
  "Akku, Ladegerät, Temperatur und Netz. Die Oberfläche ist in 45 Sprachen geplant und hat vier Modi vom Einsteiger "
  "bis zum Vollprofi. Sicherheitswarnungen erscheinen in jedem Modus.")
P("Der Explorer <b>ToyScan</b> macht alles öffentlich sichtbar: wie viele TOY bereits ausgegeben sind, den aktuellen "
  "Faktor m, den Stand der Startphase und alle Sperren mit Begründung. Eine Ampel (grün, gelb, rot) fasst neun "
  "Prüfkriterien zusammen, etwa ob die Höchstmenge eingehalten wird, ob Stake zu stark konzentriert ist oder ob die "
  "Startphase zu lange dauert. ToyScan hat nie Zugriff auf Schlüssel.")
P("Weil die Adressen dieselben sind wie bei Ethereum, zeigen Wallet und ToyScan für eine Adresse auch Guthaben auf "
  "anderen EVM-Ketten wie Ethereum oder Polygon an. Bekannte Wallets wie MetaMask können ToyChain " +
  L("als eigenes Netz einbinden", U["metamask"], "Wallet") + " (Testnetz: Chain-ID 7630714) und normale "
  "Ethereum-Überweisungen senden. Wer dort den Gaspreis auf null stellt, nutzt das kostenlose Kontingent. Eine " +
  L("Brücke (Bridge)", U["eth_bridges"],
                                                                            "Andere Ketten") +
  " zwischen den Ketten gibt es bewusst nicht: 2022 entfielen laut Chainalysis rund zwei Drittel aller gestohlenen "
  "Krypto-Gelder auf " + L("Angriffe auf Brücken", U["bridgehacks"], "Andere Ketten") + ". Stattdessen ist ein "
  + L("atomarer Tausch", U["atomic"], "Andere Ketten") + " eingeplant: Zwei Menschen tauschen direkt, abgesichert "
  "durch ein " + L("Hash-Zeitschloss", U["htlc"], "Andere Ketten") + ". Entweder bekommen beide ihr Gegenstück, oder "
  "beide ihr Geld zurück.")
FIG("abbildungen/f_swap.png", "Atomarer Tausch zwischen TOY und einem Coin einer anderen Kette, ohne Vermittler.")
BOX("<b>Wichtiger Hinweis zu gleichnamigen Token:</b> Auf Polygon und Ethereum gibt es ältere Token mit dem Namen "
    "ToyCoin und dem Kürzel TOY. Sie sind <b>nicht</b> mit dem nativen TOY auf ToyChain verbunden und nicht "
    "eintauschbar. Wallet und ToyScan kennzeichnen das deutlich.")

# ================================================================ 12
H("Anwendungsmöglichkeiten")
B(["<b>Alltagszahlungen</b> zwischen Privatleuten, mit Verwendungszweck und Endgültigkeit nach etwa einer Minute, "
   "später auch als Sammelüberweisung an bis zu 32 Empfänger. Kleine Beträge sind dank des kostenlosen Kontingents ohne Gebühr möglich.",
   "<b>Mitmachen ohne Kapital:</b> Ein altes Handy oder ein Raspberry Pi wird zum Knoten und kann über die "
   "Arbeits-Lotterie erste TOY verdienen.",
   "<b>Vereine, Familien und kleine Firmen:</b> Konten mit mehreren Unterschriften, Treuhand mit Schiedsstelle, "
   "Abo-Erlaubnisse mit fester Obergrenze und Sitzungs-Schlüssel für Apps (nach dem ersten Testnetz).",
   "<b>Vorsorge:</b> Tresor-Verzögerung, Notfall-Schlüssel und Erben-Adresse schützen vor Diebstahl und Verlust.",
   "<b>Spenden für alle:</b> Jeder kann in den Erstattungs-Topf spenden, der die kostenlosen Transaktionen aller bezahlt.",
   "<b>Privatsphäre bei Bedarf:</b> Einmal-Empfangsadressen nach " + L("EIP-5564", U["eip5564"], "Konten und Schlüssel")
   + " verbergen, wer bezahlt wurde (geplant für Meilenstein M13).",
   "<b>Lernen und Forschen:</b> Offener Code, ein Testnetz mit einem Befehl und öffentliche Daten machen ToyChain zu "
   "einem Lernobjekt für Schulen, Hochschulen und Interessierte.",
   "<b>Später denkbar:</b> Händler-Anbindungen, ein Kassen-Modus, Offline-Zahlungen und die Anbindung des Spiels "
   "TOYQUEST. Diese Ideen sind ausdrücklich nicht Teil der ersten Phase."])

# ================================================================ 13
H("Ähnlichkeiten und Unterschiede")
T([["", "Bitcoin", "Ethereum", "ToyChain (geplant)"],
   ["Start", "fair, ohne Premint", "Vorverkauf und Stiftungsanteil", "fair, Genesis ohne jedes Guthaben"],
   ["Höchstmenge", "rund 21 Mio. BTC", "keine feste Obergrenze", "100 Mio. TOY, unveränderlich"],
   ["Ausgabe", "halbiert sich etwa alle 4 Jahre", "nach Bedarf, Teil der Gebühren wird verbrannt",
    "sinkt gleichmäßig, Faktor 0,5 bis 2,0, nie Verbrennen"],
   ["Sicherheit durch", "Rechenarbeit (Proof-of-Work)", "Stake (Proof-of-Stake), 32 ETH je Validator",
    "Stake in Stufen von 10, 100, 1.000 TOY plus offene Startphase"],
   ["Endgültig nach", "etwa 1 Stunde (6 Bestätigungen)", "etwa 13 Minuten", "1 Block, etwa 1 Minute"],
   ["Speicher Voll-Knoten", "rund 760 GB", "hunderte GB bis mehrere TB", "Ziel 256 MB"],
   ["Programmierbarkeit", "einfache Skripte", "Smart Contracts, Token, NFTs", "nur fest eingebaute Funktionen"],
   ["Konten", "UTXO-Modell", "Kontenmodell", "Kontenmodell, Ethereum-Adressen"],
   ["Gebühren", "immer, nach Marktlage", "immer, Grundgebühr plus Trinkgeld",
    "10 bis 100 frei je Epoche (mind. 1 Tag), dann Gebühr oder Arbeit"],
   ["Mitmachen ohne Guthaben", "nur Knoten betreiben", "nur Knoten betreiben", "ja, inklusive Arbeits-Lotterie"]],
  [3.5, 3.6, 4.2, 5.1])
P("<b>Gemeinsam mit Bitcoin</b> hat ToyChain den fairen Start, die feste Höchstmenge, die abnehmende Ausgabe über rund "
  "25 Jahre, den Verzicht auf eine zentrale Stelle und den offenen Code. <b>Gemeinsam mit Ethereum</b> hat sie "
  "Adressen, Wörter-Phrase, Signaturformat, Proof-of-Stake und die Idee einer Grundgebühr, die bei Last steigt. "
  "<b>Neu</b> ist die Kombination aus Teilnahme ohne Guthaben, kostenlosem Kontingent, Speichergrenze von 256 MB, "
  "ruhenden Konten, freiwilligem Konto-Schutz und einer Verfassung, die Eingriffe in fremde Guthaben ausschließt.")

# ================================================================ 14
H("Vorteile")
B(["<b>Fairness:</b> Niemand bekommt TOY geschenkt, alle Regeln sind öffentlich und für alle gleich.",
   "<b>Zugänglichkeit:</b> Mitmachen mit alten Geräten, ohne Guthaben und ohne Fachwissen, in 45 Sprachen.",
   "<b>Schnelle Endgültigkeit:</b> Eine Zahlung ist nach etwa einer Minute sicher.",
   "<b>Geringer Energiebedarf:</b> Kein Wettrennen um Rechenleistung. Die Arbeits-Lotterie läuft nur am Ladegerät und "
   "nur so viel, wie der Nutzer erlaubt. Mehr Rechnen bringt nur proportional mehr Lose, keinen Vorsprung.",
   "<b>Einfachheit als Sicherheit:</b> Ohne Smart Contracts gibt es keine fremden Programme, die Guthaben einfrieren "
   "oder durch Fehler verlieren können.",
   "<b>Schutz des Eigentums:</b> Sperren entziehen nur Rechte, nie Guthaben. Konten gehen nie verloren. TOY werden nie "
   "vernichtet.",
   "<b>Transparenz:</b> ToyScan, monatliche Transparenz-Berichte ab dem Testnetz und reproduzierbare Programmversionen.",
   "<b>Kompatibilität:</b> Ethereum-Adressen und bekannte Wallets funktionieren."])

# ================================================================ 15
H("Risiken und offene Fragen")
P("Ein ehrliches Whitepaper nennt auch, was schiefgehen kann. Die wichtigsten Risiken aus heutiger Sicht:")
B(["<b>Frühes Stadium:</b> ToyChain ist noch nicht gelaufen. Fehler im Konzept oder im Code sind wahrscheinlich und "
   "sollen im Testnetz gefunden werden. Zwei Prüfberichte eines KI-Prüfagenten, der an den Texten nicht beteiligt war, "
   "haben bereits Schwachstellen aufgedeckt, die behoben wurden. Eine Prüfung durch Menschen steht noch aus. Weitere Prüfungen und ein externes Audit sind vor jedem Hauptnetz Pflicht.",
   "<b>Kleines Netz, kleine Sicherheit:</b> Eine neue Kette ist anfangs verwundbar, weil wenige Teilnehmer leichter zu "
   "überstimmen sind. Sicherheit entsteht erst durch viele unabhängige Teilnehmer.",
   "<b>Vertrauen in der Startphase:</b> Die 21 Start-Validatoren tragen in den ersten Monaten viel Verantwortung und "
   "erhalten einen begrenzten Lohn. Ob die Übergabe an neue Validatoren gelingt, muss sich zeigen.",
   "<b>Speichergrenze:</b> Ob 256 MB über Jahre reichen, hängt von der Nutzung und von der späteren Beweis-Schicht ab. "
   "Reicht es nicht, wird das offen mit Zahlen vorgelegt. Größere Knoten bräuchten dann mehr Speicher.",
   "<b>Weniger erprobte Bausteine:</b> Für manche Verfahren (etwa überprüfbaren Zufall und die Konsens-Bibliothek) gibt "
   "es in Rust weniger geprüfte Bibliotheken. Für den Zufall nutzt das Testnetz deshalb eine Lösung aus der geprüften "
   "BLS-Bibliothek, die vor einem Hauptnetz erneut bewertet wird. Zwei neue Bibliotheken (für Argon2id) sind von "
   "keiner der großen Prüf-Organisationen begutachtet und als „noch nicht durchgesehen“ vermerkt.",
   "<b>Rechenaufwand bei vielen Konten:</b> Bisher wird der Kontostand für jeden Block vollständig kopiert und neu "
   "zusammengerechnet. Bei 10.000 Konten dauert ein voller Block rund 0,1 Sekunden, bei einer Million Konten wären es "
   "rund eine Sekunde, auf alten Handys mehr. Der dauerhafte Speicher aus Meilenstein M5 muss das schrittweise lösen.",
   "<b>Rechenarbeit:</b> Ob 16 MiB Speicherbedarf reichen, um Spezialhardware fernzuhalten, wird im Testnetz gemessen.",
   "<b>Recht und Regulierung:</b> Eine Kryptowährung berührt europäisches Recht (" + L("MiCA", U["mica"], "Recht") +
   "), " + L("Steuerrecht", U["bmf"], "Recht") + " und möglicherweise das Glücksspielrecht (Begriff „Lotterie“). Vor "
   "einem Hauptnetz ist eine rechtliche Prüfung Pflicht. Die Erben-Adresse ersetzt kein Erbrecht.",
   "<b>Nutzerfehler:</b> Wer seine Wörter-Phrase verliert oder preisgibt, verliert sein Geld. Konto-Schutz und "
   "Warnungen mindern das, verhindern es aber nicht ganz.",
   "<b>Wert:</b> TOY haben keinen garantierten Wert. Ob es je einen Markt gibt, ist offen (Abschnitt 16). Niemand "
   "sollte Geld einsetzen, dessen Verlust er nicht verkraften kann.",
   "<b>Verwechslung und Betrug:</b> Ältere, gleichnamige Token auf anderen Ketten können Nutzer verwirren oder für "
   "Betrug genutzt werden. Verbraucherschützer warnen allgemein vor " + L("unseriösen Krypto-Angeboten", U["test"],
                                                                            "Verbraucherschutz") + "."])
MORE(L("Verbraucherzentrale zu Kryptowährungen", U["vz"], "Verbraucherschutz"),
     L("Stiftung Warentest: Finger weg von diesen Angeboten", U["test"]))

# ================================================================ 16
H("Chancen am Kryptomarkt: eine Einschätzung")
P("Dieses Kapitel versucht eine nüchterne Einschätzung. Es ist keine Prognose und keine Anlageempfehlung. Die Zahlen "
  "zum Markt stammen von Mitte Oktober 2026 und ändern sich schnell.")
H2("16.1 Die Ausgangslage")
P("Der gesamte Kryptomarkt war am 19. Oktober 2026 rund 2,76 Billionen US-Dollar wert, wovon 59,6 % auf Bitcoin "
  "entfielen, ein Bitcoin kostete rund 82.000 US-Dollar (" + L("TokenPost, 20.10.2026", U["market2"], "Markt") + "). "
  "Die Stimmung bei kleineren Coins war schwach: Am 18. Oktober legten von den 100 größten Kryptowerten nur "
  "14 zu (" + L("Marktbericht vom 18.10.2026", U["market"], "Markt") + ", laufend aktuell bei " +
  L("CoinGecko", U["cg_charts"], "Markt") + "). Gleichzeitig ist der Markt mit neuen Projekten überschwemmt. Laut "
  "einer Auswertung von CoinGecko wurden bis Ende 2025 über 20 Millionen Kryptowerte erfasst. Allein 2025 sind mehr als "
  "11,6 Millionen gescheitert, insgesamt über die Hälfte aller jemals erfassten (" +
  L("Bericht zur CoinGecko-Studie", U["dead"], "Markt") + "). Der allergrößte Teil davon sind kurzlebige Token auf "
  "bestehenden Ketten, keine eigenen Blockchains. Die Zahlen zeigen aber, wie schwer es ist, Aufmerksamkeit und "
  "dauerhafte Nutzer zu gewinnen.")
FIG("abbildungen/f_deadcoins.png", "Gescheiterte Kryptowerte je Jahr nach CoinGecko-Daten. Die meisten Neugründungen überleben "
                       "nicht.", 15.0)
P("In der EU gilt seit dem 1. Juli 2026 die " + L("MiCA-Verordnung", U["esma"], "Recht") + " ohne Übergangsfrist: "
  "Anbieter von Krypto-Dienstleistungen brauchen eine Zulassung, sonst müssen sie ihr Geschäft mit EU-Kunden einstellen "
  "(" + L("Hinweis der französischen Aufsicht AMF", U["amf"], "Recht") + "). Für ToyChain bedeutet das zweierlei: "
  "Börsen und Wallet-Anbieter, die TOY später aufnehmen wollen, prüfen genauer. Und für das öffentliche Angebot oder "
  "die Zulassung zum Handel verlangt MiCA in der Regel ein Kryptowerte-Whitepaper nach festen Vorgaben. Ob und wie das "
  "für einen Coin ohne Herausgeber wie TOY gilt, muss vor einem Hauptnetz rechtlich geklärt werden. Das vorliegende "
  "Dokument ist <b>kein</b> solches Whitepaper.")
H2("16.2 Was für ToyChain spricht")
B(["<b>Glaubwürdige Fairness:</b> Ein Start ohne jedes Guthaben ist selten. In einem Markt, in dem viele Projekte mit "
   "großen Insider-Anteilen starten, ist das ein klares Unterscheidungsmerkmal, das sich jeder in ToyScan selbst "
   "ansehen kann.",
   "<b>Niedrige Einstiegshürde:</b> Mitmachen mit einem alten Handy, ohne Kauf und ohne Vorwissen. Das kann Menschen "
   "erreichen, die sonst nie einen Knoten betreiben würden, und schafft viele kleine Teilnehmer statt weniger großer.",
   "<b>Weniger Angriffsfläche:</b> Ohne Smart Contracts und Brücken entfallen die Ursachen vieler großer Verluste der "
   "letzten Jahre.",
   "<b>Kompatibilität:</b> Ethereum-Adressen senken die Hürde für Nutzer, die bereits eine Wallet haben.",
   "<b>Offenheit und Sprache:</b> Freie Lizenz, ein nachvollziehbares Entscheidungsprotokoll (öffentlich ab dem "
   "Testnetz) und 45 Sprachen machen das Projekt für eine internationale Gemeinschaft zugänglich. Dieses Whitepaper "
   "gibt es bereits auf Deutsch, Englisch, Französisch und Yoruba."])
H2("16.3 Was dagegen spricht")
B(["<b>Kein Startkapital:</b> Ohne Premint gibt es keine Kasse für Entwickler, Audits, Werbung oder Börsenlistungen. "
   "Das Projekt ist auf Freiwillige, Spenden und Förderungen angewiesen.",
   "<b>Netzwerkeffekte:</b> Nutzer, Händler und Börsen sammeln sich bei großen Ketten. Eine neue Kette muss einen "
   "konkreten Grund bieten, warum jemand wechselt.",
   "<b>Bewusst eingeschränkt:</b> Ohne Smart Contracts gibt es kein Ökosystem aus Apps, Token und dezentralen Börsen, "
   "das bei anderen Ketten viel Aufmerksamkeit und Kapital anzieht.",
   "<b>Starke Konkurrenz bei Zahlungen:</b> Für Alltagszahlungen nutzen viele Menschen inzwischen an Währungen "
   "gebundene Stablecoins oder etablierte Netze. Ein Coin mit schwankendem Wert ist dafür weniger attraktiv.",
   "<b>Kleines Team, frühes Stadium:</b> Bis zu einem Hauptnetz sind noch zehn Meilensteine, Audits und rechtliche "
   "Klärungen nötig.",
   "<b>Name:</b> Ältere, gleichnamige Token erschweren die Auffindbarkeit und bergen Verwechslungsgefahr."])
FIG("abbildungen/f_swot.png", "Stärken, Schwächen, Chancen und Risiken auf einen Blick.", 15.0)
H2("16.4 Szenarien")
T([["Szenario", "Beschreibung", "Einschätzung"],
   ["A · Scheitern vor dem Hauptnetz", "Das Testnetz zeigt grundlegende Probleme oder es finden sich zu wenige "
                                       "Mitwirkende.", "realistisch, wie bei den meisten Projekten"],
   ["B · Lern- und Gemeinschaftsprojekt", "Ein stabiles Testnetz und eine kleine, aktive Gemeinschaft; TOY werden kaum "
                                          "gehandelt.", "am wahrscheinlichsten, wenn die Arbeit fortgesetzt wird"],
   ["C · Kleiner, echter Markt", "Hauptnetz nach Audit, einige hundert bis tausend Knoten, Handel auf wenigen "
                                 "Plattformen, Nutzung in Nischen (Spiel, Vereine, Spenden).",
    "möglich, braucht Gemeinschaft, Audits und Rechtsklarheit"],
   ["D · Breite Nutzung", "Einer der bekannten Coins mit Millionen Nutzern.", "sehr unwahrscheinlich"]],
  [4.2, 7.6, 4.6])
H2("16.5 Was den Unterschied machen würde")
B(["<b>Unabhängige Start-Validatoren</b> aus verschiedenen Ländern, die öffentlich sichtbar mitmachen.",
   "<b>Ein externes Sicherheits-Audit</b>, finanziert über Spenden oder Förderprogramme für freie Software.",
   "<b>Ein konkreter Anwendungsfall</b>, der ohne ToyChain schlechter ginge, etwa das Spiel TOYQUEST, in dem Spieler "
   "über ihr Handy mitmachen und TOY verdienen.",
   "<b>Konsequente Transparenz:</b> monatliche Berichte, offene Messwerte, ehrliche Fehlerkultur.",
   "<b>Rechtliche Klarheit</b> vor jedem Schritt, der echte Werte berührt."])
P("<b>Fazit der Einschätzung:</b> Als Investition ist ToyChain heute nicht bewertbar und, wie fast jede neue "
  "Kryptowährung, mit sehr hohem Verlustrisiko verbunden. Als technisches und gemeinschaftliches Projekt hat ToyChain "
  "ein klares, ungewöhnliches Profil: fair, schlank, verständlich und vorsichtig. Seine realistische Chance liegt "
  "nicht im schnellen Kursanstieg, sondern darin, eine kleine, überzeugte Gemeinschaft aufzubauen, die das Netz "
  "selbst betreibt. Gelingt das, kann daraus über Jahre mehr werden. Garantiert ist es nicht.")

# ================================================================ 17
H("Stand und Fahrplan")
P("<b>Heute (20. Oktober 2026): vor der Testphase.</b> Das Konzept ist in 121 dokumentierten Entscheidungen "
  "festgehalten. Der Bau ist in 13 Meilensteine geteilt, drei davon sind fertig und laufen mit zusammen 205 "
  "automatischen Tests. Meilenstein M1 enthält die Regeln für Beträge und die Ausgabe neuer TOY. Die Rust-Umsetzung "
  "liefert auf die "
  "kleinste Einheit genau dieselben Zahlen wie die getrennt geschriebene Referenz-Simulation in Python. Die Tests "
  "laufen bei jeder Änderung automatisch unter Linux, Windows und auf ARM-Prozessoren. Mit dem Beweis-Werkzeug Kani sind "
  "drei Kernregeln mathematisch bewiesen, jeweils in unter zwei Minuten: Keine Epoche schüttet mehr aus als die "
  "Restmenge, die Summe bleibt höchstens 100 Mio. TOY, und die Aufteilung eines Blocks geht genau auf. Drei weitere "
  "Beweise zur Aufteilung mit Lohn-Anteil waren nach 30 Minuten noch nicht fertig. Diese Aufteilung prüft das Programm "
  "deshalb bei jedem Block selbst, und zusätzliche Tests decken sie ab.")
P("<b>Meilenstein M2</b> (Kryptografie und Adressen) erzeugt aus 12 oder 24 Wörtern dieselben Adressen wie Ethereum "
  "und unterschreibt mit der eigenen Netz-Kennung. Geprüft ist das mit den offiziellen Testdaten der Standards "
  "BIP-39, BIP-32, EIP-55, EIP-155 und BLS12-381. Eine Unterschrift für ein anderes Netz wird abgelehnt.")
P("<b>Meilenstein M3</b> (Konten, Transaktionen, Gebühren, Rechenlose, Schutzregeln) bringt das Kassenbuch selbst: "
  "Konten mit Pfand, beide Transaktionsformate (das eigene und das von Ethereum), Kontingent, Gebühren, die "
  "Arbeits-Lotterie und alle Sperrregeln. Ein Zufallstest über rund 2.880 Blöcke zeigt, dass die Prüfregel I0 nach "
  "jedem Block gilt, und ein absichtlich falscher Block wird abgelehnt. Alle Prüfungen laufen bei jeder Änderung "
  "automatisch auf GitHub, auch auf Windows und auf dem Prozessor des Raspberry Pi. Am 20. Oktober 2026 waren alle "
  "34 Prüfläufe grün. Zum ersten Mal gemessen: Eine Überweisung ist 112 bis 123 Byte groß, ein Konto belegt im "
  "Zustand 49 bis 90 Byte, ein Rechenlos zu prüfen dauert auf einem PC rund 10 Millisekunden.")
FIG("abbildungen/f_roadmap.png", "Fahrplan. Für ein Hauptnetz gibt es bewusst keinen festen Termin.")
T([["Phase", "Inhalt", "Stand"],
   ["Konzept", "Spezifikation v0.3 (Stand 20.10.2026), Entscheidungsprotokoll mit 121 Entscheidungen, zwei "
    "Prüfberichte", "fertig, wird laufend gepflegt"],
   ["Phase 1, M1 bis M11", "Grundtypen und Emission, Kryptografie, Konten und Transaktionen, Konsens und Startphase, "
    "Speicher, Netzwerk, Schnittstellen, Wallet, Plattformen, Testnetz, ToyScan", "M1 bis M3 fertig, M4 als Nächstes"],
   ["Testnetz 1", "Kern plus Konto-Schutz, 45 Sprachen und vier Modi; zuerst lokal, dann öffentlich", "geplant"],
   ["Phase 1, M12 und M13", "weitere Transaktionsarten, atomarer Tausch, Namen, Einmal-Adressen, "
    "Post-Quanten-Schlüssel", "geplant"],
   ["Phase 2", "Beweis-Schicht: Vorgeschichte falten, Knoten ohne Vertrauen in Checkpoints", "geplant"],
   ["Danach", "Mitbestimmung, Hauptnetz (nur nach Audit, Lasttests und rechtlicher Prüfung, Termin vorher "
    "öffentlich), mögliche Brücken und Offline-Zahlungen", "offen"]],
  [3.4, 9.4, 3.6])
P("Es gibt keinen festen Termin für ein Hauptnetz. Es startet erst, wenn das Testnetz stabil läuft, die Sicherheit "
  "geprüft ist und die rechtlichen Fragen geklärt sind. Der Starttermin wird vorher öffentlich angekündigt, damit alle "
  "gleichzeitig beginnen können.")

# ================================================================ 18
H("Fazit")
P("ToyChain will zeigen, dass eine Kryptowährung fair beginnen, mit wenig Speicher auskommen und trotzdem schnell und "
  "sicher sein kann. Jeder soll mitmachen können, auch ohne Geld und mit einem alten Gerät. Dafür verzichtet ToyChain "
  "bewusst auf vieles, was andere Ketten anbieten, und setzt auf wenige, fest eingebaute und gut prüfbare Funktionen. "
  "Ob das gelingt, entscheidet nicht dieses Papier, sondern das Testnetz. Mitwirkung, Kritik und Prüfungen sind "
  "ausdrücklich willkommen. Code, Spezifikation und Entscheidungsprotokoll liegen auf " + L("GitHub", U["repo"]) +
  " und werden spätestens mit dem öffentlichen Testnetz für alle einsehbar.")

# ================================================================ Anhang
story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Anhang A: Wichtige Startwerte", h1))
P("Alle Werte außer denen der Verfassung sind Annahmen für das Testnetz und werden dort gemessen und angepasst.")
T([["Bereich", "Startwert"],
   ["Menge und Teilbarkeit", "100.000.000 TOY, 18 Dezimalstellen (Verfassung)"],
   ["Ausgabe je Epoche", "0,0505 % der Restmenge mal m; m zwischen 0,5 und 2,0, Änderung höchstens 0,05 je Epoche"],
   ["Aufteilung neuer TOY", "50 % Konsens, 40 % Arbeits-Lotterie, 10 % Erstattungs-Topf"],
   ["Zeit", "Blockzeit 60 s, frühestens 10 s; Epoche mindestens 1.440 Blöcke und 24 Stunden"],
   ["Komitee und Blöcke", "höchstens 128 Validatoren, Block höchstens 128 KB, endgültig nach 1 Block"],
   ["Startphase", "21 Start-Validatoren, Lohn von 50 % auf 0 % in 180 Tagen, 365 Epochen gebunden, Übergabe "
                  "frühestens nach 90 Tagen"],
   ["Stake", "10 / 100 / 1.000 TOY; Lösen nach 21 Epochen; Deckel 5 %, ab 100 Validatoren 1 %"],
   ["Strafe Doppelsignatur", "max(1 %, 3 × Anteil der gleichzeitig Betroffenen), höchstens 100 %, Rolle 7 Epochen "
                             "gesperrt"],
   ["Konten", "Aktivierungs-Pfand 0,1 TOY (darf bis 0,01 TOY sinken); Auslagern ruhender Konten ab 90 % Füllung"],
   ["Kontingent", "10 bis 100 kostenlose Transaktionen je Konto und Epoche, ab einem Kontoalter von 100 Blöcken; "
                  "Auslastung aus den Bytes aller Transaktionen der Vorepoche"],
   ["Gebühren", "Grundgebühr ab 0,000001 TOY je Byte, ±1/8 je Block; Faktor höchstens 1.024; Aufteilung 40 % "
                "Gebühren-Lotterie, 40 % Erstattungs-Topf, 20 % Vorschlagender"],
   ["Erstattungs-Topf", "Erstattung höchstens 1/30 des Bestands je Epoche; Bestand höchstens 30 Epochen der "
                        "größtmöglichen Erstattung, Überschuss in die Arbeits-Lotterie"],
   ["Arbeits-Lotterie", "Argon2id mit 16 MiB, 1 Durchlauf; höchstens 16 Lose je Block, Ziel 8; Ziehung 2 Blöcke später; "
                        "bis zu 64 Gewinner je Block, Gewinn mindestens 0,2 TOY; 1 Los = 1 gebührenfreie Transaktion"],
   ["Netz-Kennung", "Chain-ID 7630713 Hauptnetz, 7630714 Testnetz, 7630715 lokales Probenetz"],
   ["Sperrzeiten", "nur bei eindeutig nachweisbaren Verstößen, Verdopplung bei Wiederholung bis höchstens 64 Epochen"],
   ["Speicher", "256 MB je Standard-Knoten: 128 MB Zustand, 48 MB Block-Fenster, 80 MB Beweise, Indizes und Reserve"]],
  [4.0, 12.4])

story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Anhang B: Linkverzeichnis", h1))
P("Die wichtigsten Erklärungen zum Weiterlesen, nach Themen geordnet. Englischsprachige Quellen sind gekennzeichnet. "
  "Stand der Links: Oktober 2026. Webseiten können sich ändern, bitte im Zweifel die Hauptseite der Quelle "
  "aufrufen. Der Link zum Projekt funktioniert erst, wenn das Repository öffentlich ist.")
link_rows = [["Thema", "Link", "Worum es geht"],
             ["Grundlagen", L("Bitcoin-Whitepaper", U["btc_pdf"]) + " (englisch)", "das Original von 2008, 9 Seiten"],
             ["Grundlagen", L("Bitcoin-Whitepaper auf Deutsch", U["btc_de"]), "Übersetzung und Einordnung"],
             ["Geldmenge", L("Controlled supply (Bitcoin Wiki)", U["btc_supply"]) + " (englisch)",
              "wie Bitcoin seine Menge halbiert"],
             ["Konsens", L("Konsensmechanismen (ethereum.org)", U["eth_consensus"]), "Überblick auf Deutsch"],
             ["Konsens", L("Proof-of-Stake (ethereum.org)", U["eth_pos"]), "Stake, Validatoren, Endgültigkeit"],
             ["Konsens", L("The latest gossip on BFT consensus", U["tendermint"]) + " (englisch)",
              "Fachartikel zum Tendermint-Verfahren"],
             ["Konsens", L("MEV (ethereum.org)", U["eth_mev"]), "was „Vordrängeln“ bei Ethereum bedeutet"],
             ["Technik", L("BLS12-381 For The Rest Of Us", U["bls"]) + " (englisch)", "gemeinsame Unterschriften"],
             ["Technik", L("Merkle-Bäume (Bitpanda Academy)", U["merkle"]), "Beweise für ruhende Konten"],
             ["Technik", L("RFC 9106: Argon2", U["argon2"]) + " (englisch)", "Standard der Rechenarbeit"],
             ["Technik", L("RFC 9381: VRF", U["vrf"]) + " (englisch)", "überprüfbarer Zufall"],
             ["Technik", L("Dandelion++", U["dandelion"]) + " (englisch)", "Schutz der Absender-IP"],
             ["Technik", L("libp2p", U["libp2p"]) + " (englisch)", "Netzwerk-Bibliothek"],
             ["Technik", L("Post-Quanten-Kryptografie (ComputerWeekly)", U["pqk"]), "Einführung auf Deutsch"],
             ["Technik", L("NIST FIPS 204 (ML-DSA)", U["mldsa"]) + " (englisch)", "Post-Quanten-Signaturen"],
             ["Technik", L("Kani (Ferrous Systems)", U["kani"]) + " (englisch)", "mathematische Prüfung von Rust-Code"],
             ["Technik", L("Reproducible Builds", U["repro"]) + " (englisch)", "nachprüfbare Programmversionen"],
             ["Konten", L("BIP-39", U["bip39"]) + " (englisch)", "Standard der 12/24-Wörter-Phrase"],
             ["Konten", L("EIP-155", U["eip155"]) + " (englisch)", "Kennung der Kette in jeder Signatur"],
             ["Konten", L("EIP-5564 erklärt (QuickNode)", U["eip5564"]) + " (englisch)", "Einmal-Empfangsadressen"],
             ["Gebühren", L("EIP-1559", U["eip1559"]) + " (englisch)", "Grundgebühr bei Ethereum"],
             ["Wallet", L("Netz in MetaMask hinzufügen", U["metamask"]) + " (englisch)", "eigene Netze einbinden"],
             ["Andere Ketten", L("Brücken (ethereum.org)", U["eth_bridges"]), "wie Brücken funktionieren, Risiken"],
             ["Andere Ketten", L("Brücken-Hacks 2022 (Decrypt)", U["bridgehacks"]) + " (englisch)",
              "Chainalysis-Zahlen zu Verlusten"],
             ["Andere Ketten", L("Atomic Swap (Blockstream)", U["atomic"]) + " (englisch)", "Tausch ohne Vermittler"],
             ["Andere Ketten", L("Hashed Timelock Contracts", U["htlc"]) + " (englisch)", "das Zeitschloss dahinter"],
             ["Energie", L("Cambridge Bitcoin Electricity Index", U["cbeci"]) + " (englisch)",
              "Stromverbrauch von Bitcoin"],
             ["Markt", L("CoinGecko Charts", U["cg_charts"]) + " (englisch)", "aktueller Marktwert"],
             ["Markt", L("Marktbericht 18.10.2026 (DailyForex)", U["market"]) + " (englisch)", "Stand zum Datum"],
             ["Markt", L("Marktwert 19.10.2026 (TokenPost)", U["market2"]) + " (englisch)",
              "Gesamtwert und Bitcoin-Anteil"],
             ["Technik", L("ethereum/tests", U["ethtests"]) + " (englisch)", "offizielle Ethereum-Testdaten"],
             ["Markt", L("CoinGecko-Studie zu gescheiterten Coins (ForkLog)", U["dead"]) + " (englisch)",
              "Zahlen 2021 bis 2025"],
             ["Recht", L("MiCA-Verordnung (EUR-Lex)", U["mica"]), "amtlicher Text auf Deutsch"],
             ["Recht", L("MiCA-Regelwerk (ESMA)", U["esma"]) + " (englisch)", "Regeln und Leitlinien"],
             ["Recht", L("Ende der MiCA-Übergangszeit (AMF)", U["amf"]) + " (englisch)", "was ab 1.7.2026 gilt"],
             ["Recht", L("BMF-Schreiben zu Kryptowerten", U["bmf"]), "Steuern in Deutschland"],
             ["Verbraucherschutz", L("Verbraucherzentrale", U["vz"]), "Bitcoin als Spekulationsobjekt"],
             ["Verbraucherschutz", L("Stiftung Warentest", U["test"]), "Warnung vor unseriösen Angeboten"],
             ["Projekt", L("github.com/Safehaigit/toycoin", U["repo"]), "Code, Spezifikation, Entscheidungen"]]
T(link_rows, [3.0, 7.6, 5.8])

story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Anhang C: Quellen und Standards", h1))
refs = [
    "S. Nakamoto: Bitcoin: A Peer-to-Peer Electronic Cash System, 2008.",
    "E. Buchman, J. Kwon, Z. Milosevic: The latest gossip on BFT consensus (Tendermint), 2018.",
    "BIP-39: Mnemonic code for generating deterministic keys. EIP-155: Simple replay attack protection. EIP-1559: Fee "
    "market change. EIP-2718: Typed transaction envelope. EIP-5564: Stealth addresses.",
    "D. Boneh, B. Lynn, H. Shacham: Short signatures from the Weil pairing (BLS), 2001; BLS12-381.",
    "RFC 9106: Argon2 memory-hard function. RFC 9381: Verifiable Random Functions (VRFs).",
    "G. Fanti et al.: Dandelion++: Lightweight cryptocurrency networking with formal anonymity guarantees, 2018.",
    "NIST FIPS 204: Module-Lattice-Based Digital Signature Standard (ML-DSA), 2024.",
    "Verordnung (EU) 2023/1114 über Märkte für Kryptowerte (MiCA).",
    "Marktdaten: TokenPost, 20.10.2026 (Stand 19.10.2026); DailyForex, Crypto Overview, 18.10.2026; CoinGecko-Daten "
    "nach ForkLog, Januar 2026; Chainalysis nach "
    "Decrypt, 2022; Spark, Bitcoin Full Node Requirements 2026.",
    "ToyChain: Technische Spezifikation v0.3, Entscheidungsprotokoll, Bauplan Phase 1, Prüfberichte 1 und 2, "
    "Referenz-Simulation m-simulation.py, Statusbericht mit Messwerten; github.com/Safehaigit/toycoin.",
    "Testdaten: ethereum/tests (RLP und Transaktionen, Commit c67e485f); RFC 9106 Argon2id-Vektor aus "
    "P-H-C/phc-winner-argon2.",
]
for i, r in enumerate(refs, 1):
    story.append(Paragraph(f"[{i}] {r}", small))
story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Anhang D: Änderungen gegenüber Entwurf 0.3", h1))
P("Entwurf 0.4 bringt den Stand nach den Meilensteinen M2 und M3 und die Entscheidungen 115 bis 121 vom 20. Oktober "
  "2026. Neu ist außerdem eine getrennte Kurzanleitung für Nutzer.")
T([["Stelle", "Änderung", "Grund"],
   ["Titel, 17, Abb. 16", "Stand: M1 bis M3 fertig, 121 Entscheidungen, 205 Tests, erste Messwerte, Fahrplan.",
    "M2 und M3 abgeschlossen (20.10.2026)"],
   ["2, Anhang A", "Netz-Kennungen 7630713, 7630714, 7630715.", "Entscheidung 116"],
   ["2, 6.2, Anhang A", "Gebühren mit Zahlen: 0,000001 TOY je Byte, ±1/8 je Block, Faktor höchstens 1.024, "
    "Aufteilung 40/40/20, Rechenlos statt Gebühr, Vielnutzer-Regel.", "Entscheidungen 117, 119, 121"],
   ["6.2", "Kontingent: Auslastung der Vorepoche, Kontoalter 100 Blöcke, Erstattung höchstens 1/30 je Epoche.",
    "Entscheidung 118 und genauere Beschreibung"],
   ["5.4, Anhang A", "Obergrenze des Erstattungs-Topfs, Überschuss in die Arbeits-Lotterie.",
    "Entscheidung 120 (löst Prüfbefund P9)"],
   ["6.3", "Lose an Adresse und Tag gebunden, unterschrieben, Ziehung 2 Blöcke später, höchstens 16 je Block.",
    "Entscheidungen 106, 119"],
   ["7.1, 15", "VRF aus eindeutigen BLS-Unterschriften statt RFC 9381, Wiedervorlage vor dem Hauptnetz.",
    "Entscheidung 115"],
   ["10.2", "Sperrzeiten mit Beispielen, Neubeginn nach 30 Tagen, Überweisen gegen Gebühr bleibt möglich.",
    "Entscheidung 121, Tests aus M3"],
   ["10.4, 11", "Offizielle Testdaten; MetaMask mit Testnetz-Kennung und Gaspreis 0.", "M2, M3, Entscheidung 121"],
   ["15", "Neue Risiken: ungeprüfte Argon2id-Bibliotheken, Rechenaufwand je Block bei vielen Konten.",
    "Messungen aus M3"],
   ["16.1, 16.3", "Marktzahlen vom 19.10.2026; noch zehn statt zwölf Meilensteine.", "neuer Stand"]],
  [2.6, 8.6, 5.2])
story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Frühere Änderungen: Entwurf 0.3 gegenüber Entwurf 0.2", h2))
P("Entwurf 0.3 ist eine Prüfung von Entwurf 0.2 auf Logik, Nachvollziehbarkeit und neue Erkenntnisse vom 18. und "
  "19. Oktober 2026. Am Konzept selbst hat sich nichts geändert.")
T([["Stelle", "Änderung", "Grund"],
   ["1, 18, Anhang B", "Das Repository ist derzeit privat; es soll spätestens mit dem öffentlichen Testnetz frei "
    "werden.", "Entwurf 0.2 nannte es fälschlich öffentlich."],
   ["2, 13", "Kostenloses Kontingent „je Epoche (mindestens ein Tag)“ statt „je Tag“.",
    "genauer: Eine Epoche kann länger als 24 Stunden dauern."],
   ["5.3", "Abweichung vom Plan: im ersten Jahr höchstens etwa 2,2 %, auf Dauer höchstens etwa 2,5 %.",
    "„höchstens etwa 2 %“ war zu knapp (Simulation: +2,23 %)."],
   ["6.1", "Belohnung nach der Startphase vollständig beschrieben: ein Fünftel als gleicher Grundbetrag je "
    "Komitee-Mitglied, der Rest linear, beides nach tatsächlicher Mitarbeit.", "fehlte, Entscheidung 112"],
   ["6.3", "Ein erster Lotteriegewinn reicht für das Aktivierungs-Pfand.",
    "beantwortet die Frage, wie man ohne TOY zum ersten Konto kommt"],
   ["7.1", "„zwei Phasen“ (Vorabstimmung und Bestätigung) statt „zwei Runden“.",
    "„Runde“ bedeutet im Verfahren etwas anderes."],
   ["8", "Planzeit erklärt; Lohn als „Hälfte aller neuen TOY“; was geschieht, wenn die Übergabe nach 180 Tagen "
    "noch aussteht.", "Abbildung 8 nutzte den Begriff ohne Erklärung; Lücke im Ablauf"],
   ["9", "Neue Knoten starten bis Phase 2 von Checkpoints mit drei unabhängigen Unterschriften.",
    "Lücke: Wem vertraut ein neuer Knoten bis dahin?"],
   ["10.1", "Abbruchfrist der Erben-Adresse (30 Epochen) und ehrlicher Hinweis zum Notfall-Schlüssel.",
    "Vollständigkeit und Ehrlichkeit"],
   ["10.4, 17", "Ergebnis der Kani-Beweise: drei Kernregeln bewiesen, drei Beweise noch offen.",
    "neue Erkenntnis vom 18.10.2026"],
   ["12", "Sammelüberweisung erst nach dem ersten Testnetz.", "Umfang von Testnetz 1 (Entscheidung 109)"],
   ["14", "Energie: Lotterie nur am Ladegerät und so viel, wie der Nutzer erlaubt.",
    "„wenn Strom ohnehin verfügbar ist“ war ungenau."],
   ["15", "Prüfberichte stammen von einem KI-Prüfagenten; eine Prüfung durch Menschen steht aus.",
    "„unabhängige Prüfberichte“ war missverständlich."],
   ["16.2", "Entscheidungsprotokoll öffentlich erst ab Testnetz; Whitepaper in vier Sprachen (Deutsch, Englisch, Französisch, Yoruba).",
    "Richtigstellung und neuer Stand"]],
  [2.4, 8.6, 5.4])
story.append(Spacer(1, 6))
story.append(Paragraph("<b>Haftungsausschluss.</b> Dieses Dokument beschreibt ein Softwareprojekt in der Entwicklung. "
                       "Es ist weder Anlage-, Steuer- noch Rechtsberatung und kein Angebot, Prospekt oder "
                       "Kryptowerte-Whitepaper im Sinne der MiCA-Verordnung. Alle Angaben sind vorläufig und können "
                       "sich ändern. Marktzahlen geben den Stand zum genannten Datum wieder.", small))


def on_page(c, doc):
    c.saveState()
    c.setFont("Sans", 7.5)
    c.setFillColor(MUTED)
    c.drawString(2.3 * cm, 1.3 * cm, "ToyChain · Vorläufiges Whitepaper · Entwurf 0.4 · vor der Testphase")
    c.drawRightString(A4[0] - 2.3 * cm, 1.3 * cm, f"Seite {doc.page}")
    c.setStrokeColor(RULE)
    c.setLineWidth(0.5)
    c.line(2.3 * cm, 1.65 * cm, A4[0] - 2.3 * cm, 1.65 * cm)
    c.restoreState()


doc = BaseDocTemplate("ToyChain-Whitepaper-v0.4.pdf", pagesize=A4, leftMargin=2.3 * cm, rightMargin=2.3 * cm,
                      topMargin=2.0 * cm, bottomMargin=2.1 * cm,
                      title="ToyChain und ToyCoin (TOY): Vorläufiges Whitepaper", author="ToyChain-Projekt",
                      subject="Entwurf 0.4, vor der Testphase", lang="de-DE")
frame = Frame(doc.leftMargin, doc.bottomMargin, doc.width, doc.height, id="f", leftPadding=0, rightPadding=0,
              topPadding=0, bottomPadding=0)
doc.addPageTemplates([PageTemplate(id="p", frames=[frame], onPage=on_page)])
doc.build(story)
print("links:", len(set(U.values())))
