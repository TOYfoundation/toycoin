from reportlab.lib.pagesizes import A4
from reportlab.lib.units import cm
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.enums import TA_JUSTIFY, TA_CENTER, TA_LEFT
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (BaseDocTemplate, Frame, PageTemplate, Paragraph, Spacer,
                                Table, TableStyle, Image, KeepTogether, ListFlowable, ListItem)

D = "/usr/share/fonts/truetype/dejavu/"
pdfmetrics.registerFont(TTFont("Serif", D + "DejaVuSerif.ttf"))
pdfmetrics.registerFont(TTFont("Serif-B", D + "DejaVuSerif-Bold.ttf"))
pdfmetrics.registerFont(TTFont("Serif-I", D + "DejaVuSerif-Italic.ttf"))
pdfmetrics.registerFont(TTFont("Serif-BI", D + "DejaVuSerif-BoldItalic.ttf"))
pdfmetrics.registerFont(TTFont("Sans", D + "DejaVuSans.ttf"))
pdfmetrics.registerFont(TTFont("Sans-B", D + "DejaVuSans-Bold.ttf"))
pdfmetrics.registerFontFamily("Serif", normal="Serif", bold="Serif-B", italic="Serif-I", boldItalic="Serif-BI")
pdfmetrics.registerFontFamily("Sans", normal="Sans", bold="Sans-B", italic="Sans", boldItalic="Sans-B")

INK = colors.HexColor("#1b1b1a")
MUTED = colors.HexColor("#52514e")
RULE = colors.HexColor("#c9c8c2")
TINT = colors.HexColor("#f3f2ee")
ACCENT = colors.HexColor("#2a78d6")

body = ParagraphStyle("body", fontName="Serif", fontSize=9.6, leading=13.6, alignment=TA_JUSTIFY,
                      textColor=INK, spaceAfter=5.5)
small = ParagraphStyle("small", parent=body, fontSize=8.2, leading=11, spaceAfter=3)
cell = ParagraphStyle("cell", fontName="Sans", fontSize=7.9, leading=10.2, textColor=INK, alignment=TA_LEFT)
cellb = ParagraphStyle("cellb", parent=cell, fontName="Sans-B")
h1 = ParagraphStyle("h1", fontName="Sans-B", fontSize=12, leading=15, spaceBefore=11, spaceAfter=5, textColor=INK)
h2 = ParagraphStyle("h2", fontName="Sans-B", fontSize=10, leading=13, spaceBefore=6, spaceAfter=3, textColor=INK)
title = ParagraphStyle("title", fontName="Serif-B", fontSize=19, leading=24, alignment=TA_CENTER, textColor=INK)
subtitle = ParagraphStyle("sub", fontName="Serif", fontSize=11, leading=15, alignment=TA_CENTER, textColor=MUTED)
center = ParagraphStyle("center", parent=body, alignment=TA_CENTER, fontSize=9.4)
abstract = ParagraphStyle("abs", parent=body, fontSize=9.2, leading=13, leftIndent=1.1 * cm, rightIndent=1.1 * cm)
caption = ParagraphStyle("cap", parent=small, alignment=TA_CENTER, textColor=MUTED, fontName="Serif-I")

story = []
sec = [0]


def H(text):
    sec[0] += 1
    story.append(Paragraph(f"{sec[0]}.&nbsp;&nbsp;{text}", h1))


def H2(text):
    story.append(Paragraph(text, h2))


def P(text, style=body):
    story.append(Paragraph(text, style))


def B(items, style=body):
    story.append(ListFlowable([ListItem(Paragraph(t, style), leftIndent=12, value="•") for t in items],
                              bulletType="bullet", start="•", leftIndent=12, bulletFontName="Serif",
                              bulletFontSize=8, spaceAfter=4))


def T(rows, widths, header=True, zebra=True):
    data = []
    for i, r in enumerate(rows):
        data.append([Paragraph(c, cellb if (header and i == 0) else cell) for c in r])
    t = Table(data, colWidths=[w * cm for w in widths], repeatRows=1 if header else 0)
    st = [("VALIGN", (0, 0), (-1, -1), "TOP"),
          ("LINEBELOW", (0, 0), (-1, 0), 0.8, INK),
          ("LINEBELOW", (0, -1), (-1, -1), 0.6, RULE),
          ("TOPPADDING", (0, 0), (-1, -1), 3), ("BOTTOMPADDING", (0, 0), (-1, -1), 3),
          ("LEFTPADDING", (0, 0), (-1, -1), 4), ("RIGHTPADDING", (0, 0), (-1, -1), 4)]
    if zebra:
        for i in range(1, len(rows)):
            if i % 2 == 0:
                st.append(("BACKGROUND", (0, i), (-1, i), TINT))
    t.setStyle(TableStyle(st))
    story.append(Spacer(1, 2))
    story.append(t)
    story.append(Spacer(1, 7))


def BOX(text):
    t = Table([[Paragraph(text, ParagraphStyle("bx", parent=body, fontName="Sans", fontSize=8.8,
                                               leading=12.4, alignment=TA_LEFT, spaceAfter=0))]],
              colWidths=[16.4 * cm])
    t.setStyle(TableStyle([("BOX", (0, 0), (-1, -1), 1.1, ACCENT),
                           ("BACKGROUND", (0, 0), (-1, -1), colors.HexColor("#eef4fc")),
                           ("TOPPADDING", (0, 0), (-1, -1), 7), ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
                           ("LEFTPADDING", (0, 0), (-1, -1), 9), ("RIGHTPADDING", (0, 0), (-1, -1), 9)]))
    story.append(t)
    story.append(Spacer(1, 8))


# ---------------------------------------------------------------- Titel
story.append(Spacer(1, 0.4 * cm))
story.append(Paragraph("ToyChain und ToyCoin (TOY)", title))
story.append(Spacer(1, 4))
story.append(Paragraph("Ein schlankes, fair gestartetes Zahlungsnetz,<br/>an dem jeder ohne Guthaben teilnehmen kann", subtitle))
story.append(Spacer(1, 10))
story.append(Paragraph("ToyChain-Projekt · github.com/Safehaigit/toycoin", center))
story.append(Paragraph("Vorläufiges Whitepaper · Entwurf 0.1 · 18. Oktober 2026", center))
story.append(Spacer(1, 12))
BOX("<b>Stand: vor der Testphase.</b> ToyChain existiert bisher als Konzept und als erster Programmbaustein. "
    "Fertig und getestet sind die Grundtypen und die Regeln der Geldentstehung (Meilenstein M1 von 13). "
    "Es gibt noch <b>kein Testnetz, kein Hauptnetz und keine handelbaren TOY</b>. Alle Zahlen sind Startwerte, die im "
    "Testnetz gemessen und angepasst werden. Dieses Dokument ist keine Anlageberatung und kein Angebot zum Kauf von "
    "irgendetwas. Guthaben aus dem Testnetz haben keinen Wert und werden nie in ein späteres Hauptnetz übernommen.")

P("<b>Zusammenfassung.</b> ToyChain ist eine eigenständige Blockchain mit der Währung ToyCoin (TOY), höchstens "
  "100 Millionen Stück. Sie startet völlig fair: Der erste Block enthält kein einziges TOY, es gibt keine Reserve für "
  "Gründer, keinen Vorverkauf und keine Sonderzuteilung. Alle TOY entstehen nach und nach durch eine feste, öffentlich "
  "nachprüfbare Regel, ähnlich wie bei Bitcoin. Anders als bei Bitcoin entsteht Sicherheit nicht durch Stromverbrauch, "
  "sondern durch Teilnehmer, die TOY als Pfand binden und Blöcke gemeinsam unterschreiben. Ein Block ist nach etwa einer "
  "Minute endgültig. Jeder Knoten kommt mit 256 MB Daten aus und läuft auch auf alten Handys und auf dem Raspberry Pi. "
  "Mitmachen ist ohne Guthaben möglich, wer mehr beiträgt, bekommt schrittweise mehr Rechte. Bewusst verzichtet ToyChain "
  "auf Smart Contracts, Token und NFTs: Jede Funktion ist fest eingebaut, einfach prüfbar und kann keine fremden "
  "Guthaben antasten. Konten-Adressen sind dieselben wie bei Ethereum, so dass bekannte Wallets weiter funktionieren.", abstract)

# ---------------------------------------------------------------- 1
H("Einleitung")
P("Bitcoin hat 2008 gezeigt, dass Menschen sich ohne Bank darauf einigen können, wem welches Geld gehört. Seitdem "
  "sind viele Kryptowährungen entstanden. Bei vielen davon haben Gründer und frühe Geldgeber einen großen Teil der "
  "Coins schon vor dem Start erhalten (Premint). Andere brauchen große Rechenzentren, viel Strom oder Festplatten mit "
  "hunderten Gigabyte, so dass normale Menschen das Netz kaum selbst prüfen können. Und viele Ketten sind mit "
  "Smart Contracts so vielseitig geworden, dass Fehler in fremdem Code regelmäßig zu Verlusten führen.")
P("ToyChain versucht, die Grundidee von Bitcoin, also feste Menge, fairer Start und offene Regeln, mit heutigen "
  "Verfahren schlank und alltagstauglich umzusetzen. Die Leitfragen sind: Kann jeder mit einem alten Gerät mitmachen? "
  "Kann jeder die Regeln selbst nachprüfen? Bleibt das eigene Guthaben unter allen Umständen das eigene? Dieses "
  "Whitepaper erklärt die Ideen in einfacher Sprache. Die genauen Regeln stehen in der technischen Spezifikation "
  "(Version 0.3) im öffentlichen Repository.")

# ---------------------------------------------------------------- 2
H("ToyChain auf einen Blick")
T([["Eigenschaft", "ToyChain (geplante Startwerte)"],
   ["Währung", "ToyCoin, Kürzel TOY, teilbar in 18 Dezimalstellen"],
   ["Höchstmenge", "100.000.000 TOY, im Code als unveränderliche Regel festgeschrieben"],
   ["Start", "Totaler Fair Launch: erster Block ohne Guthaben, kein Premint, keine Reserve"],
   ["Neue TOY", "je Epoche (mindestens 24 Stunden) 0,0505 % der Restmenge, angepasst mit Faktor m zwischen 0,5 und 2,0; "
                "erstes Jahr rund 16,84 Mio., Hälfte nach rund 3,76 Jahren, 99 % nach rund 25 Jahren"],
   ["Einigung (Konsens)", "Proof-of-Stake mit Komitee von höchstens 128 Validatoren, gemeinsame Unterschrift, "
                          "endgültig nach 1 Block"],
   ["Blockzeit", "60 Sekunden, bei vollem Wartebereich früher, frühestens nach 10 Sekunden"],
   ["Speicher je Knoten", "Ziel 256 MB, auch auf alten Android-Geräten und dem Raspberry Pi"],
   ["Konten", "wie Ethereum: gleiche Adressen, gleiche 12- oder 24-Wörter-Phrase"],
   ["Funktionen", "nur fest eingebaute Transaktionsarten, keine Smart Contracts, keine Token, keine NFTs"],
   ["Gebühren", "10 bis 100 kostenlose Transaktionen je Konto und Tag, darüber kleine Gebühr oder Rechenarbeit"],
   ["Lizenz", "MIT (freie Software)"]],
  [4.0, 12.4])

# ---------------------------------------------------------------- 3
H("Grundbegriffe einfach erklärt")
P("Wer noch nie mit Kryptowährungen zu tun hatte, findet hier die wichtigsten Begriffe. Die folgenden Abschnitte "
  "bauen darauf auf.")
T([["Begriff", "Einfach gesagt"],
   ["Blockchain", "Ein gemeinsames Kassenbuch, das viele Computer gleichzeitig führen. Neue Einträge werden in Paketen "
                  "(Blöcken) angehängt, jeder Block verweist fälschungssicher auf den vorigen."],
   ["Knoten", "Ein Gerät mit der ToyChain-Software. Es prüft selbst, ob alle Einträge den Regeln entsprechen, und "
              "muss dafür niemandem vertrauen."],
   ["Wallet", "Die App, die Ihre Schlüssel verwahrt und Überweisungen unterschreibt. Bei ToyChain ist die Wallet "
              "TOYPAY zugleich ein Knoten."],
   ["Schlüssel und Phrase", "Ein geheimer Schlüssel beweist, dass Ihnen ein Konto gehört. Er lässt sich aus 12 oder "
                            "24 Wörtern wiederherstellen. Wer die Wörter kennt, kann über das Geld verfügen."],
   ["Signatur", "Eine digitale Unterschrift. Jeder kann prüfen, dass sie echt ist, aber niemand kann sie ohne den "
                "Schlüssel fälschen."],
   ["Konsens", "Das Verfahren, mit dem sich alle Knoten auf denselben nächsten Block einigen."],
   ["Stake", "TOY, die man freiwillig als Pfand bindet. Wer sich an die Regeln hält, bekommt Belohnungen. Wer "
             "nachweislich betrügt, verliert einen Teil des Pfands."],
   ["Validator", "Ein Knoten mit Stake, der Blöcke vorschlägt und mitunterschreibt."],
   ["Epoche", "Ein Abschnitt der Kette von mindestens 1.440 Blöcken und mindestens 24 Stunden. Viele Regeln zählen "
              "in Epochen, etwa die Ausgabe neuer TOY."],
   ["Testnetz", "Eine Probe-Kette, deren Coins nichts wert sind. Dort werden Fehler gesucht, bevor es ernst wird."]],
  [3.6, 12.8])

# ---------------------------------------------------------------- 4
H("Fairer Start und Geldmenge")
H2("4.1 Kein einziges TOY im ersten Block")
P("Der erste Block (Genesis-Block) enthält bei ToyChain <b>keinerlei Guthaben</b>. Niemand, auch nicht die Entwickler, "
  "besitzt beim Start TOY. Es gibt keinen Vorverkauf, keine Reserve und keinen Gründeranteil. Das ist strenger als bei "
  "den meisten neuen Projekten und entspricht dem Geist von Bitcoin. Diese Regel gehört zur „Verfassung“ (Abschnitt 9.3) "
  "und kann durch kein Update geändert werden.")
H2("4.2 Wie neue TOY entstehen")
P("In jeder Epoche entsteht ein fester Bruchteil der noch nicht ausgegebenen Menge: 0,0505 % der Restmenge. In der "
  "ersten Epoche sind das genau 50.500 TOY. Weil die Restmenge schrumpft, sinkt die Ausgabe gleichmäßig, statt wie "
  "bei Bitcoin alle vier Jahre sprunghaft auf die Hälfte zu fallen. Nach rund 3,76 Jahren ist die Hälfte ausgegeben, "
  "nach rund 25 Jahren etwa 99 %. Die Höchstmenge wird nie erreicht und nie überschritten. Das ist bereits durch Tests "
  "belegt: Selbst mit doppelter Ausgabegeschwindigkeit über 100 Jahre bleibt die Summe darunter.")
story.append(KeepTogether([Image("emission.png", width=15.6 * cm, height=7.3 * cm),
                           Paragraph("Abbildung 1: Ausgegebener Anteil der Höchstmenge über die Zeit. ToyChain sinkt "
                                     "gleichmäßig, Bitcoin halbiert in Stufen. Am Ende sind beide ähnlich.", caption)]))
H2("4.3 Der Faktor m: angepasst, aber gezügelt")
P("Die Ausgabe wird mit einem Faktor m zwischen 0,5 und 2,0 multipliziert. Er steigt leicht, wenn das Netz "
  "<i>wirklich</i> genutzt wird (bezahlte oder durch Rechenarbeit gedeckte Transaktionen) und wenn viele Validatoren "
  "zuverlässig mitunterschreiben. Kostenlose Transaktionen zählen nicht, sonst ließe sich die Ausgabe mit Spam "
  "hochtreiben. Ein Zeitplan-Anteil holt Abweichungen wieder ein, und m darf sich je Epoche nur um 0,05 ändern. In "
  "Simulationen weicht die Ausgabe im ersten Jahr höchstens um etwa 2 % vom Plan ab. m verschiebt also nur, <i>wann</i> "
  "TOY entstehen, nicht <i>wie viele</i> insgesamt.")
H2("4.4 Wer die neuen TOY bekommt")
P("Jede Ausgabe wird fest aufgeteilt: <b>50 %</b> an alle, die das Netz mit Stake absichern, <b>40 %</b> an eine "
  "Arbeits-Lotterie, an der jedes Gerät ohne Guthaben teilnehmen kann (Abschnitt 5.3), und <b>10 %</b> an einen "
  "Erstattungs-Topf, der die kostenlosen Transaktionen aller bezahlt. Gerechnet wird ausschließlich mit ganzen Zahlen "
  "in der kleinsten Einheit (10<super>-18</super> TOY). Rundungsreste gehen in den Erstattungs-Topf, so dass in jeder "
  "Epoche genau die geplante Menge entsteht.")
H2("4.5 Es wird nie TOY vernichtet")
P("Viele Ketten „verbrennen“ Gebühren oder Strafen. ToyChain nicht: Strafen und Spenden fließen in den "
  "Erstattungs-Topf und kommen so den Nutzern zugute. Eine Prüfregel (I0) stellt in <b>jedem</b> Block sicher, dass die "
  "Summe aller Guthaben genau der Summe aller ausgegebenen TOY entspricht. Ein Block, der das verletzt, ist ungültig.")

# ---------------------------------------------------------------- 5
H("Mitmachen: Stufen, Kontingent und Arbeits-Lotterie")
H2("5.1 Teilnahme-Stufen")
P("Jeder entscheidet selbst, wie viel er beiträgt. Die Wallet misst beim ersten Start das Gerät und schlägt eine "
  "passende Stufe vor.")
T([["Stufe", "Voraussetzung", "Was das Gerät tut", "Was es bekommt"],
   ["0 · Leicht", "nichts", "prüft Blöcke, leitet weiter, rechnet Arbeit", "Lose in der Arbeits-Lotterie"],
   ["1 · Konto", "0,1 TOY Aktivierungs-Pfand", "senden und empfangen", "10 bis 100 kostenlose Transaktionen je Tag"],
   ["2 · Attestierer", "10 TOY Stake", "unterschreibt in Gruppen mit", "Anteil an der Konsens-Belohnung"],
   ["3 · Voll", "100 TOY Stake", "hält alle aktuellen Daten, versorgt andere", "höherer Anteil für Verfügbarkeit"],
   ["4 · Validator", "1.000 TOY Stake", "schlägt Blöcke vor, stimmt ab", "größter Anteil, 20 % der Blockgebühren"]],
  [2.6, 3.5, 5.4, 4.9])
P("Gebundenes TOY bleibt Eigentum des Nutzers und lässt sich mit 21 Epochen Wartezeit wieder lösen. Damit niemand "
  "übermächtig wird, zählt der Stake eines Teilnehmers höchstens bis 5 % (später 1 %) des gesamten Stakes. Die "
  "Belohnung steigt linear mit dem Stake, dazu kommt ein gleicher Grundbetrag für jeden aktiven Validator. Das "
  "Aufteilen auf viele Konten bringt deshalb keinen Vorteil.")
H2("5.2 Kostenlose Transaktionen und Gebühren")
P("Jedes aktivierte Konto hat je Epoche ein kostenloses Kontingent: 100 Transaktionen, wenn das Netz leer ist, "
  "mindestens 10 bei Hochbetrieb. Für den Alltag reicht das. Darüber zahlt man eine kleine Gebühr, die bei vielen "
  "Transaktionen im selben Block schnell steigt, oder man legt einen Arbeitsnachweis bei. So bleibt das Netz für normale "
  "Nutzer kostenlos, während Spam teuer wird. Kostenlose Transaktionen belegen höchstens die Hälfte eines Blocks.")
H2("5.3 Die Arbeits-Lotterie")
P("Damit auch Menschen ohne TOY an neue Coins kommen, gibt es eine Lotterie für Rechenarbeit. Das Gerät berechnet "
  "Lose mit dem Verfahren Argon2id, das bewusst viel Arbeitsspeicher (16 MiB) braucht. Dadurch haben Spezialhardware "
  "und Grafikkarten-Farmen kaum Vorteile gegenüber einem Handy oder PC. Je Block werden bis zu 64 Gewinner fair "
  "ausgelost, jeder Gewinn beträgt mindestens 0,2 TOY. Wer mehr rechnet, hat mehr Lose, die erwartete Belohnung ist "
  "proportional zur Arbeit. Gerechnet wird nur, wenn das Gerät es erlaubt: am Ladegerät, im WLAN und solange es kühl bleibt.")

# ---------------------------------------------------------------- 6
H("Einigung im Netz")
H2("6.1 So entsteht ein Block")
P("Ein Komitee aus höchstens 128 Validatoren wird je Epoche neu zusammengesetzt. Für jeden Block wird per "
  "überprüfbarem Zufall (VRF) einer ausgewählt, der den Block vorschlägt. Die anderen stimmen in zwei Runden ab, nach "
  "einem bewährten Verfahren im Stil von Tendermint. Sobald zwei Drittel zugestimmt haben, ist der Block "
  "<b>endgültig</b>. Er kann nicht mehr zurückgenommen werden. Bei Bitcoin wartet man dagegen üblicherweise etwa eine "
  "Stunde, bis eine Zahlung als sicher gilt. Alle Stimmen werden mit BLS-Signaturen zu einer einzigen, nur 96 Byte "
  "großen Unterschrift zusammengefasst, die auch ein Handy in Millisekunden prüfen kann.")
H2("6.2 Faire Reihenfolge")
P("Wer einen Block baut, kann die Reihenfolge der Transaktionen nicht frei wählen. Sie ist fest vorgegeben: nach "
  "Gebühr, bei Gleichstand nach einer Zufallszahl, die erst mit dem Block feststeht. Jeder Knoten prüft das nach. Später "
  "soll eine Aufnahme-Pflicht dafür sorgen, dass Transaktionen, die viele Validatoren sehen, auch wirklich in den "
  "nächsten Block kommen. Weil es auf ToyChain keine Tauschbörsen-Verträge gibt, entfällt das bei Ethereum bekannte "
  "„Vordrängeln“ (MEV) weitgehend.")
H2("6.3 Strafen, die zum Schaden passen")
P("Wer zwei widersprüchliche Stimmen für denselben Block abgibt (Doppelsignatur), verliert einen Teil seines Stakes. "
  "Das passiert oft aus Versehen, etwa wenn derselbe Schlüssel auf zwei Geräten läuft. Deshalb hängt die Strafe davon "
  "ab, wie viele es gleichzeitig tun: Ein einzelner Ausrutscher kostet 1 %, ein gemeinsamer Angriff von einem Drittel "
  "des Stakes 100 %. Die Software schützt zusätzlich vor Pannen: Sie merkt sich dauerhaft, was sie unterschrieben hat, "
  "und bleibt nach einem Gerätewechsel zunächst stumm, wenn ihr Schlüssel woanders schon aktiv ist. Gekürzter Stake wird "
  "nicht vernichtet, sondern geht in den Erstattungs-Topf.")

# ---------------------------------------------------------------- 7
H("Die Startphase")
P("Ein fairer Start ohne Premint hat bei Proof-of-Stake ein Henne-Ei-Problem: Am Anfang hat niemand TOY, also auch "
  "niemand Stake, mit dem er das Netz absichern könnte. ToyChain löst das offen und zeitlich begrenzt:")
B(["<b>21 Start-Validatoren</b>, betrieben von Freiwilligen mit möglichst verschiedenen Betreibern und Standorten, "
   "sichern das Netz mit gleichem Gewicht. Ihre Namen und Einnahmen sind in ToyScan öffentlich.",
   "<b>Sinkender Lohn:</b> Anfangs erhalten sie zusammen den halben Anteil der Ausgabe. Er sinkt nach Tagen "
   "linear und ist nach 180 Tagen null. Ein Deckel verhindert, dass schnellere Ausgabe oder absichtlich langsame "
   "Blöcke mehr Lohn bringen. Insgesamt sind es höchstens rund 2,22 Mio. TOY (etwa 2,2 % der Höchstmenge).",
   "<b>Gebundener Lohn:</b> Jeder Lohn bleibt 365 Epochen (rund ein Jahr) gesperrt und haftet bei Fehlverhalten mit.",
   "<b>Strenge Übergabe:</b> Die Startphase endet frühestens nach 90 Tagen und erst, wenn mindestens 21 neue "
   "Validatoren mit je 1.000 TOY seit 30 Epochen gebunden sind und zusammen zwei Drittel des wirksamen Stakes halten. "
   "Danach verlieren die Start-Validatoren jeden Sonderstatus."])
P("Ehrlich gesagt: Die Startphase ist der Teil, in dem ToyChain am meisten Vertrauen in wenige Menschen braucht. Kein "
  "Protokoll kann beweisen, dass „neue“ Validatoren wirklich andere Menschen sind. Die Regeln machen eine Übernahme aber "
  "teuer und für alle sichtbar.")

# ---------------------------------------------------------------- 8
H("Schlanke Knoten für alte Geräte")
P("Eine Bitcoin- oder Ethereum-Kette vollständig zu speichern, braucht heute hunderte Gigabyte. ToyChain setzt sich "
  "ein festes Ziel von <b>256 MB je Standard-Knoten</b>. Das gelingt, weil ein Knoten nur den aktuellen Kontostand aller "
  "aktiven Konten und ein kurzes Fenster der letzten Blöcke speichert. Ältere Blöcke bewahren freiwillige Archiv-Knoten "
  "auf. In einer späteren Phase soll die gesamte Vorgeschichte in einen kleinen kryptografischen Beweis „gefaltet“ werden, "
  "so dass auch neue Knoten nichts mehr vertrauen müssen.")
P("Damit trotzdem 100 Millionen Konten und mehr Platz finden, gibt es <b>aktive und ruhende Konten</b>. Lange nicht "
  "benutzte Konten werden bei Bedarf aus dem Speicher ausgelagert, aber <b>nie gelöscht</b>. Ihr Inhalt ist durch "
  "einen Merkle-Beweis von knapp 1 KB gesichert, den die Wallet selbst aufbewahrt. Mit der nächsten Transaktion wird das "
  "Konto automatisch wieder aufgeweckt, mit vollem Guthaben. Damit niemand den Speicher mit Millionen leerer Konten "
  "füllt, werden beim ersten Eingang 0,1 TOY als <b>Aktivierungs-Pfand</b> gebunden. Das Pfand bleibt Eigentum des "
  "Nutzers, wird nie gekürzt und kommt beim Auflösen des Kontos vollständig zurück. Es darf später nur sinken, "
  "höchstens bis 0,01 TOY.")

# ---------------------------------------------------------------- 9
H("Sicherheit für Nutzer")
H2("9.1 Freiwilliger Konto-Schutz")
P("Jeder kann sein eigenes Konto zusätzlich absichern. Diese Einstellungen gelten nur, weil der Eigentümer sie "
  "selbst gesetzt hat. Niemand sonst kann sie setzen.")
B(["<b>Tresor-Verzögerung:</b> Größere Abbuchungen werden erst nach einer Wartezeit wirksam und lassen sich in dieser "
   "Zeit abbrechen. Ein Dieb kann das Konto also nicht sofort leeren.",
   "<b>Notfall-Schlüssel und Sicherheitsadresse:</b> Ein zweiter, getrennt aufbewahrter Schlüssel kann nichts ausgeben, "
   "aber wartende Abbuchungen stoppen und alles an eine vorher festgelegte sichere Adresse retten.",
   "<b>Selbst-Sperre:</b> Handy verloren? Das eigene Konto lässt sich bis zu 30 Epochen sperren.",
   "<b>Erben-Adresse:</b> Bleibt ein Konto sehr lange ohne jede Aktivität (mindestens 180 Epochen), kann eine selbst "
   "gewählte Erben-Adresse es übernehmen. Der Eigentümer wird vorher mehrfach erinnert und kann jederzeit abbrechen."])
H2("9.2 Schutzregeln mit Augenmaß")
P("Angriffe sollen früh gestoppt werden, ohne ehrliche Nutzer zu treffen. Deshalb gibt es drei Ebenen. Die Kette "
  "selbst ahndet nur <b>eindeutig nachweisbare</b> Verstöße, etwa eine Doppelsignatur oder einen gefälschten Block. Die "
  "Folge ist eine Sperrzeit, die Rechte entzieht (zum Beispiel das kostenlose Kontingent), <b>nie aber Guthaben</b>. "
  "Jeder Knoten schützt sich zusätzlich selbst vor Überlastung. Auffällige Muster, die nur geschätzt werden können, zeigt "
  "der Explorer ToyScan als Warnung an, sie führen aber nie automatisch zu einer Sperre.")
H2("9.3 Die Verfassung")
P("Einige Regeln sind so grundlegend, dass sie fest im Code stehen und durch kein Update und keine spätere Abstimmung "
  "geändert werden dürfen: die Höchstmenge von 100 Mio. TOY, der Start ohne Guthaben, die Ausgabeformel, das Verbot, TOY "
  "zu vernichten, die Mindestdauer einer Epoche, der Verzicht auf Token und Smart Contracts und vor allem: <b>Keine Regel "
  "darf TOY von einem Konto nehmen, einfrieren oder umverteilen.</b> Ehrlich gesagt kann jeder, der eine eigene "
  "Software verbreitet, eine Kette abspalten. Wallet und ToyScan prüfen die Verfassung deshalb selbst und warnen, wenn "
  "eine Abspaltung sie bricht.")
H2("9.4 Bewährte Kryptografie")
P("ToyChain erfindet keine eigenen Verschlüsselungsverfahren. Verwendet werden weit verbreitete, geprüfte Standards: "
  "secp256k1 und Keccak-256 wie bei Ethereum für Konten, BIP-39 für die Wörter-Phrase, BLS12-381 für die gemeinsamen "
  "Unterschriften der Validatoren, BLAKE3 für Prüfsummen, Argon2id für die Rechenarbeit und für die Verschlüsselung "
  "der Wallet auf dem Gerät. Alle Verfahren sitzen hinter austauschbaren Schnittstellen, damit sie später ersetzt "
  "werden können, etwa durch Verfahren, die auch Quantencomputern standhalten. Ein freiwilliger Zusatzschlüssel nach "
  "dem NIST-Standard ML-DSA ist bereits eingeplant.")

# ---------------------------------------------------------------- 10
H("TOYPAY Wallet, ToyScan und andere Ketten")
P("Die <b>TOYPAY Wallet</b> ist Geldbörse und Knoten in einem. Sie läuft auf Android ab etwa 1 GB Arbeitsspeicher, "
  "auf Windows, Linux und dem Raspberry Pi. Ein Energie-Modus regelt automatisch, wie viel das Gerät beiträgt, je nach "
  "Akku, Ladegerät, Temperatur und Netz. Die Oberfläche ist in 45 Sprachen geplant und hat vier Modi vom Einsteiger bis "
  "zum Vollprofi. Sicherheitswarnungen erscheinen in jedem Modus.")
P("Der Explorer <b>ToyScan</b> macht alles öffentlich sichtbar: wie viele TOY bereits ausgegeben sind, den aktuellen "
  "Faktor m, den Stand der Startphase und alle Sperren mit Begründung. Eine Ampel (grün, gelb, rot) fasst neun "
  "Prüfkriterien zusammen, etwa ob die Höchstmenge eingehalten wird, ob Stake zu stark konzentriert ist oder ob die "
  "Startphase zu lange dauert. ToyScan hat nie Zugriff auf Schlüssel.")
P("Weil die Adressen dieselben sind wie bei Ethereum, zeigen Wallet und ToyScan für eine Adresse auch Guthaben auf "
  "anderen EVM-Ketten wie Ethereum oder Polygon an. Bekannte Wallets wie MetaMask können ToyChain als eigenes Netz "
  "einbinden. Eine Brücke (Bridge) zwischen den Ketten gibt es bewusst nicht, weil Brücken in der Vergangenheit häufig "
  "Ziel großer Angriffe waren. Stattdessen ist ein <b>atomarer Tausch</b> eingeplant: Zwei Menschen tauschen direkt, "
  "abgesichert durch ein Hash-Zeitschloss. Entweder bekommen beide ihr Gegenstück, oder beide ihr Geld zurück.")
BOX("<b>Wichtiger Hinweis zu gleichnamigen Token:</b> Auf Polygon und Ethereum gibt es ältere Token mit dem Namen "
    "ToyCoin und dem Kürzel TOY. Sie sind <b>nicht</b> mit dem nativen TOY auf ToyChain verbunden und nicht eintauschbar. "
    "Wallet und ToyScan kennzeichnen das deutlich.")

# ---------------------------------------------------------------- 11
H("Anwendungsmöglichkeiten")
B(["<b>Alltagszahlungen</b> zwischen Privatleuten, mit Verwendungszweck, Sammelüberweisung an bis zu 32 Empfänger und "
   "Endgültigkeit nach etwa einer Minute. Kleine Beträge sind dank des kostenlosen Kontingents ohne Gebühr möglich.",
   "<b>Mitmachen ohne Kapital:</b> Ein altes Handy oder ein Raspberry Pi wird zum Knoten und kann über die Arbeits-Lotterie "
   "erste TOY verdienen.",
   "<b>Vereine, Familien und kleine Firmen:</b> Konten mit mehreren Unterschriften, Treuhand mit Schiedsstelle, "
   "Abo-Erlaubnisse mit fester Obergrenze und Sitzungs-Schlüssel für Apps (nach dem ersten Testnetz).",
   "<b>Vorsorge:</b> Tresor-Verzögerung, Notfall-Schlüssel und Erben-Adresse schützen vor Diebstahl und Verlust.",
   "<b>Spenden für alle:</b> Jeder kann in den Erstattungs-Topf spenden, der die kostenlosen Transaktionen aller bezahlt.",
   "<b>Lernen und Forschen:</b> Offener Code, ein Testnetz mit einem Befehl und öffentliche Daten machen ToyChain zu einem "
   "Lernobjekt für Schulen, Hochschulen und Interessierte.",
   "<b>Später denkbar:</b> Händler-Anbindungen, ein Kassen-Modus, Offline-Zahlungen und die Anbindung des Spiels "
   "TOYQUEST. Diese Ideen sind ausdrücklich nicht Teil der ersten Phase."])

# ---------------------------------------------------------------- 12
H("Ähnlichkeiten und Unterschiede")
T([["", "Bitcoin", "Ethereum", "ToyChain (geplant)"],
   ["Start", "fair, ohne Premint", "Vorverkauf und Stiftungsanteil", "fair, Genesis ohne jedes Guthaben"],
   ["Höchstmenge", "rund 21 Mio. BTC", "keine feste Obergrenze", "100 Mio. TOY, unveränderlich"],
   ["Ausgabe", "halbiert sich etwa alle 4 Jahre", "nach Bedarf, Teil der Gebühren wird verbrannt",
    "sinkt gleichmäßig, Faktor 0,5 bis 2,0, nie Verbrennen"],
   ["Sicherheit durch", "Rechenarbeit (Proof-of-Work)", "Stake (Proof-of-Stake), 32 ETH je Validator",
    "Stake in Stufen von 10, 100, 1.000 TOY plus offene Startphase"],
   ["Endgültig nach", "etwa 1 Stunde (6 Bestätigungen)", "etwa 13 Minuten", "1 Block, etwa 1 Minute"],
   ["Speicher Voll-Knoten", "hunderte GB", "hunderte GB bis mehrere TB", "Ziel 256 MB"],
   ["Programmierbarkeit", "einfache Skripte", "Smart Contracts, Token, NFTs", "nur fest eingebaute Funktionen"],
   ["Konten", "UTXO-Modell", "Kontenmodell", "Kontenmodell, Ethereum-Adressen"],
   ["Gebühren", "immer, nach Marktlage", "immer, Grundgebühr plus Trinkgeld",
    "10 bis 100 frei je Tag, dann Gebühr oder Arbeit"],
   ["Mitmachen ohne Guthaben", "nur Knoten betreiben", "nur Knoten betreiben", "ja, inklusive Arbeits-Lotterie"]],
  [3.5, 3.6, 4.2, 5.1])
P("<b>Gemeinsam mit Bitcoin</b> hat ToyChain den fairen Start, die feste Höchstmenge, die abnehmende Ausgabe über rund "
  "25 Jahre, den Verzicht auf eine zentrale Stelle und den offenen Code. <b>Gemeinsam mit Ethereum</b> hat sie Adressen, "
  "Wörter-Phrase, Signaturformat, Proof-of-Stake und die Idee einer Grundgebühr, die bei Last steigt. <b>Neu</b> ist die "
  "Kombination aus Teilnahme ohne Guthaben, kostenlosem Kontingent, Speichergrenze von 256 MB, ruhenden Konten, "
  "freiwilligem Konto-Schutz und einer Verfassung, die Eingriffe in fremde Guthaben ausschließt.")

# ---------------------------------------------------------------- 13
H("Vorteile")
B(["<b>Fairness:</b> Niemand bekommt TOY geschenkt, alle Regeln sind öffentlich und für alle gleich.",
   "<b>Zugänglichkeit:</b> Mitmachen mit alten Geräten, ohne Guthaben und ohne Fachwissen, in 45 Sprachen.",
   "<b>Schnelle Endgültigkeit:</b> Eine Zahlung ist nach etwa einer Minute sicher.",
   "<b>Geringer Energiebedarf:</b> Kein Wettrennen um Rechenleistung. Die Arbeits-Lotterie läuft nur, wenn Strom ohnehin "
   "verfügbar ist, und belohnt Arbeit nur proportional.",
   "<b>Einfachheit als Sicherheit:</b> Ohne Smart Contracts gibt es keine fremden Programme, die Guthaben einfrieren oder "
   "durch Fehler verlieren können.",
   "<b>Schutz des Eigentums:</b> Sperren entziehen nur Rechte, nie Guthaben. Konten gehen nie verloren. TOY werden nie "
   "vernichtet.",
   "<b>Transparenz:</b> ToyScan, monatliche Transparenz-Berichte ab dem Testnetz und reproduzierbare Programmversionen.",
   "<b>Kompatibilität:</b> Ethereum-Adressen und bekannte Wallets funktionieren."])

# ---------------------------------------------------------------- 14
H("Risiken und offene Fragen")
P("Ein ehrliches Whitepaper nennt auch, was schiefgehen kann. Die wichtigsten Risiken aus heutiger Sicht:")
B(["<b>Frühes Stadium:</b> ToyChain ist noch nicht gelaufen. Fehler im Konzept oder im Code sind wahrscheinlich und "
   "sollen im Testnetz gefunden werden. Zwei unabhängige Prüfberichte haben bereits Schwachstellen aufgedeckt, die "
   "behoben wurden. Weitere Prüfungen und ein externes Audit sind vor jedem Hauptnetz Pflicht.",
   "<b>Kleines Netz, kleine Sicherheit:</b> Eine neue Kette ist anfangs verwundbar, weil wenige Teilnehmer leichter zu "
   "überstimmen sind. Sicherheit entsteht erst durch viele unabhängige Teilnehmer.",
   "<b>Vertrauen in der Startphase:</b> Die 21 Start-Validatoren tragen in den ersten Monaten viel Verantwortung und "
   "erhalten einen begrenzten Lohn. Ob die Übergabe an neue Validatoren gelingt, muss sich zeigen.",
   "<b>Speichergrenze:</b> Ob 256 MB über Jahre reichen, hängt von der Nutzung und von der späteren Beweis-Schicht ab. "
   "Reicht es nicht, wird das offen mit Zahlen vorgelegt. Größere Knoten bräuchten dann mehr Speicher.",
   "<b>Weniger erprobte Bausteine:</b> Für manche Verfahren (etwa überprüfbaren Zufall und die Konsens-Bibliothek) gibt es "
   "in Rust weniger geprüfte Bibliotheken. Sie werden streng nach Standard umgesetzt und extern geprüft.",
   "<b>Rechenarbeit:</b> Ob 16 MiB Speicherbedarf reichen, um Spezialhardware fernzuhalten, wird im Testnetz gemessen.",
   "<b>Recht und Regulierung:</b> Eine Kryptowährung berührt europäisches Recht (zum Beispiel MiCA), Steuerrecht und "
   "möglicherweise das Glücksspielrecht (Begriff „Lotterie“). Vor einem Hauptnetz ist eine rechtliche Prüfung Pflicht. Die "
   "Erben-Adresse ersetzt kein Erbrecht.",
   "<b>Nutzerfehler:</b> Wer seine Wörter-Phrase verliert oder preisgibt, verliert sein Geld. Konto-Schutz und Warnungen "
   "mindern das, verhindern es aber nicht ganz.",
   "<b>Wert:</b> TOY haben keinen garantierten Wert. Ob es je einen Markt gibt, ist offen. Niemand sollte Geld einsetzen, "
   "dessen Verlust er nicht verkraften kann.",
   "<b>Verwechslung:</b> Ältere, gleichnamige Token auf anderen Ketten können Nutzer verwirren oder für Betrug genutzt "
   "werden."])

# ---------------------------------------------------------------- 15
H("Stand und Fahrplan")
P("<b>Heute (18. Oktober 2026): vor der Testphase.</b> Das Konzept ist in 113 dokumentierten Entscheidungen festgehalten. "
  "Der Bau ist in 13 Meilensteine geteilt. Meilenstein M1 ist fertig: Die Regeln für Beträge und die Ausgabe neuer TOY "
  "sind programmiert und mit 45 automatischen Tests geprüft. Die Rust-Umsetzung liefert auf die kleinste Einheit genau "
  "dieselben Zahlen wie die unabhängige Referenz-Simulation. Die Tests laufen automatisch unter Linux, Windows und auf "
  "ARM-Prozessoren.")
T([["Phase", "Inhalt", "Stand"],
   ["Konzept", "Spezifikation v0.3, Entscheidungsprotokoll, zwei Prüfberichte", "fertig, wird laufend gepflegt"],
   ["Phase 1, M1 bis M11", "Grundtypen und Emission, Kryptografie, Konten und Transaktionen, Konsens und Startphase, "
    "Speicher, Netzwerk, Schnittstellen, Wallet, Plattformen, Testnetz, ToyScan", "M1 fertig, M2 als Nächstes"],
   ["Testnetz 1", "Kern plus Konto-Schutz, 45 Sprachen und vier Modi; zuerst lokal, dann öffentlich", "geplant"],
   ["Phase 1, M12 und M13", "weitere Transaktionsarten, atomarer Tausch, Namen, Einmal-Adressen, "
    "Post-Quanten-Schlüssel", "geplant"],
   ["Phase 2", "Beweis-Schicht: Vorgeschichte falten, Knoten ohne Vertrauen in Checkpoints", "geplant"],
   ["Danach", "Mitbestimmung, Hauptnetz (nur nach Audit, Lasttests und rechtlicher Prüfung, Termin vorher "
    "öffentlich), mögliche Brücken und Offline-Zahlungen", "offen"]],
  [3.4, 9.4, 3.6])
P("Es gibt keinen festen Termin für ein Hauptnetz. Es startet erst, wenn das Testnetz stabil läuft, die Sicherheit "
  "geprüft ist und die rechtlichen Fragen geklärt sind. Der Starttermin wird vorher öffentlich angekündigt, damit "
  "alle gleichzeitig beginnen können.")

# ---------------------------------------------------------------- 16
H("Fazit")
P("ToyChain will zeigen, dass eine Kryptowährung fair beginnen, mit wenig Speicher auskommen und trotzdem schnell "
  "und sicher sein kann. Jeder soll mitmachen können, auch ohne Geld und mit einem alten Gerät. Dafür verzichtet "
  "ToyChain bewusst auf vieles, was andere Ketten anbieten, und setzt auf wenige, fest eingebaute und gut prüfbare "
  "Funktionen. Ob das gelingt, entscheidet nicht dieses Papier, sondern das Testnetz. Mitwirkung, Kritik und "
  "Prüfungen sind ausdrücklich willkommen.")

# ---------------------------------------------------------------- Anhang
story.append(Paragraph("Anhang: Wichtige Startwerte", h1))
P("Alle Werte außer denen der Verfassung sind Annahmen für das Testnetz und werden dort gemessen und angepasst.")
T([["Bereich", "Startwert"],
   ["Menge und Teilbarkeit", "100.000.000 TOY, 18 Dezimalstellen (Verfassung)"],
   ["Ausgabe je Epoche", "0,0505 % der Restmenge mal m; m zwischen 0,5 und 2,0, Änderung höchstens 0,05 je Epoche"],
   ["Aufteilung neuer TOY", "50 % Konsens, 40 % Arbeits-Lotterie, 10 % Erstattungs-Topf"],
   ["Zeit", "Blockzeit 60 s, frühestens 10 s; Epoche mindestens 1.440 Blöcke und 24 Stunden"],
   ["Komitee und Blöcke", "höchstens 128 Validatoren, Block höchstens 128 KB, endgültig nach 1 Block"],
   ["Startphase", "21 Start-Validatoren, Lohn von 50 % auf 0 % in 180 Tagen, 365 Epochen gebunden, "
                  "Übergabe frühestens nach 90 Tagen"],
   ["Stake", "10 / 100 / 1.000 TOY; Lösen nach 21 Epochen; Deckel 5 %, ab 100 Validatoren 1 %"],
   ["Strafe Doppelsignatur", "max(1 %, 3 × Anteil der gleichzeitig Betroffenen), höchstens 100 %, Rolle 7 Epochen gesperrt"],
   ["Konten", "Aktivierungs-Pfand 0,1 TOY (darf bis 0,01 TOY sinken); Auslagern ruhender Konten ab 90 % Füllung"],
   ["Kontingent", "10 bis 100 kostenlose Transaktionen je Konto und Epoche, ab einem Kontoalter von 100 Blöcken"],
   ["Arbeits-Lotterie", "Argon2id mit 16 MiB; bis zu 64 Gewinner je Block, Gewinn mindestens 0,2 TOY"],
   ["Sperrzeiten", "nur bei eindeutig nachweisbaren Verstößen, Verdopplung bei Wiederholung bis höchstens 64 Epochen"],
   ["Speicher", "256 MB je Standard-Knoten: 128 MB Zustand, 48 MB Block-Fenster, 80 MB Beweise, Indizes und Reserve"]],
  [4.0, 12.4])
H2("Mitwirken")
P("ToyChain ist freie Software unter der MIT-Lizenz. Code, Spezifikation, Entscheidungsprotokoll und Prüfberichte "
  "liegen öffentlich auf GitHub. Gesucht werden Menschen, die den Code prüfen, im Testnetz einen Knoten betreiben, "
  "als Start-Validator mitwirken, Texte in eine der 45 Sprachen übersetzen oder Schwachstellen melden. Sicherheitslücken "
  "bitte nicht öffentlich, sondern über die private Meldefunktion von GitHub melden.")

# ---------------------------------------------------------------- Quellen
story.append(Paragraph("Quellen und Standards", h1))
refs = [
    "S. Nakamoto: Bitcoin: A Peer-to-Peer Electronic Cash System, 2008.",
    "E. Buchman, J. Kwon, Z. Milosevic: The latest gossip on BFT consensus (Tendermint), 2018.",
    "BIP-39: Mnemonic code for generating deterministic keys. EIP-155: Simple replay attack protection. "
    "EIP-1559: Fee market change. EIP-2718: Typed transaction envelope. EIP-5564: Stealth addresses.",
    "D. Boneh, B. Lynn, H. Shacham: Short signatures from the Weil pairing (BLS), 2001; BLS12-381.",
    "RFC 9106: Argon2 memory-hard function. RFC 9381: Verifiable Random Functions (VRFs).",
    "G. Fanti et al.: Dandelion++: Lightweight cryptocurrency networking with formal anonymity guarantees, 2018.",
    "NIST FIPS 204: Module-Lattice-Based Digital Signature Standard (ML-DSA), 2024.",
    "ToyChain: Technische Spezifikation v0.3, Entscheidungsprotokoll, Bauplan Phase 1, Prüfberichte 1 und 2, "
    "Referenz-Simulation m-simulation.py; github.com/Safehaigit/toycoin.",
]
for i, r in enumerate(refs, 1):
    story.append(Paragraph(f"[{i}] {r}", small))
story.append(Spacer(1, 6))
story.append(Paragraph("<b>Haftungsausschluss.</b> Dieses Dokument beschreibt ein Softwareprojekt in der Entwicklung. "
                       "Es ist weder Anlage-, Steuer- noch Rechtsberatung und kein Angebot oder Prospekt. Alle Angaben "
                       "sind vorläufig und können sich ändern.", small))


def on_page(c, doc):
    c.saveState()
    c.setFont("Sans", 7.5)
    c.setFillColor(MUTED)
    c.drawString(2.3 * cm, 1.3 * cm, "ToyChain · Vorläufiges Whitepaper · Entwurf 0.1 · vor der Testphase")
    c.drawRightString(A4[0] - 2.3 * cm, 1.3 * cm, f"Seite {doc.page}")
    c.setStrokeColor(RULE)
    c.setLineWidth(0.5)
    c.line(2.3 * cm, 1.65 * cm, A4[0] - 2.3 * cm, 1.65 * cm)
    c.restoreState()


doc = BaseDocTemplate("ToyChain-Whitepaper-v0.1.pdf", pagesize=A4, leftMargin=2.3 * cm, rightMargin=2.3 * cm,
                      topMargin=2.0 * cm, bottomMargin=2.1 * cm, title="ToyChain und ToyCoin (TOY): Vorläufiges Whitepaper",
                      author="ToyChain-Projekt", subject="Entwurf 0.1, vor der Testphase", lang="de-DE")
frame = Frame(doc.leftMargin, doc.bottomMargin, doc.width, doc.height, id="f", leftPadding=0, rightPadding=0,
              topPadding=0, bottomPadding=0)
doc.addPageTemplates([PageTemplate(id="p", frames=[frame], onPage=on_page)])
doc.build(story)
