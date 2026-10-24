"""ToyChain short user guide (German, draft 0.4): how a user will set up ToyChain once the testnet runs.

Run: python3 build_kurzanleitung.py  (same layout as the whitepaper, see wp_common.py)
"""
from reportlab.lib import colors
from reportlab.lib.units import cm
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.enums import TA_CENTER, TA_LEFT
from reportlab.platypus import Paragraph, Spacer, Table, TableStyle, KeepTogether

from wp_common import (story, title, subtitle, center, body, small, h1, h2, cell, INK, MUTED, RULE, ACCENT,
                       L, H, H2, P, B, T, BOX, set_words, finish)

set_words("Abbildung", "Mehr dazu:")

U = dict(
    repo="https://github.com/Safehaigit/toycoin",
    metamask="https://support.metamask.io/configure/networks/how-to-add-a-custom-network-rpc",
    bip39="https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki",
    vz="https://www.verbraucherzentrale.de/wissen/geld-versicherungen/sparen-und-anlegen/bitcoin-vom-alternativen-zahlungsmittel-zum-spekulationsobjekt-11641",
)

num_style = ParagraphStyle("num", fontName="Sans-B", fontSize=15, leading=18, alignment=TA_CENTER,
                           textColor=colors.white)
step_title = ParagraphStyle("stt", fontName="Sans-B", fontSize=10, leading=13, textColor=INK, spaceAfter=2)
step_body = ParagraphStyle("stb", parent=body, fontSize=9.2, leading=13, alignment=TA_LEFT, spaceAfter=0)


def STEP(n, heading, text):
    """A numbered step: blue number on the left, heading and text on the right."""
    right = [Paragraph(heading, step_title), Paragraph(text, step_body)]
    t = Table([[Paragraph(str(n), num_style), right]], colWidths=[1.1 * cm, 15.3 * cm])
    t.setStyle(TableStyle([("VALIGN", (0, 0), (-1, -1), "TOP"),
                           ("BACKGROUND", (0, 0), (0, 0), ACCENT),
                           ("TOPPADDING", (0, 0), (0, 0), 5), ("BOTTOMPADDING", (0, 0), (0, 0), 5),
                           ("LEFTPADDING", (1, 0), (1, 0), 9), ("TOPPADDING", (1, 0), (1, 0), 1),
                           ("LINEBELOW", (0, 0), (-1, 0), 0.5, RULE)]))
    story.append(KeepTogether([t, Spacer(1, 7)]))


# ================================================================ Titel
story.append(Spacer(1, 0.2 * cm))
story.append(Paragraph("ToyChain einrichten", title))
story.append(Spacer(1, 4))
story.append(Paragraph("Kurzanleitung für Nutzerinnen und Nutzer", subtitle))
story.append(Spacer(1, 8))
story.append(Paragraph("Begleitheft zum Whitepaper · Entwurf 0.4 · 20. Oktober 2026", center))
story.append(Spacer(1, 10))
BOX("<b>Heute gibt es noch nichts zu installieren.</b> Diese Anleitung beschreibt, wie das Einrichten aussehen soll, "
    "sobald die TOYPAY Wallet und das Testnetz fertig sind (Meilensteine M8 bis M10). Bildschirme und Namen von Knöpfen "
    "können sich bis dahin noch ändern. Im Testnetz sind TOY <b>nichts wert</b> und werden nie in ein späteres "
    "Hauptnetz übernommen. Wer heute TOY zum Kauf angeboten bekommt, hat es mit einem Betrug oder mit einem "
    "gleichnamigen, fremden Token zu tun.")

P("<b>Kurz gesagt:</b> App holen, Sprache wählen, 12 Wörter auf Papier schreiben, Gerät mitmachen lassen. Mehr braucht "
  "es nicht. Sie brauchen dafür <b>kein Geld</b>, kein Fachwissen und kein neues Gerät. Das alles dauert etwa zehn "
  "Minuten. Alles Weitere ist freiwillig.")

# ================================================================ 1
H("Was Sie brauchen")
T([["Gerät", "Geeignet für", "Hinweis"],
   ["Android-Handy oder Tablet, ab etwa 1 GB Arbeitsspeicher", "Wallet und Mitmachen ohne Guthaben (Stufe 0 und 1)",
    "auch ältere Geräte; gerechnet wird nur am Ladegerät und im WLAN"],
   ["PC oder Laptop mit Windows oder Linux", "alle Stufen", "für höhere Stufen sollte das Gerät oft eingeschaltet sein"],
   ["Raspberry Pi 4 oder 5", "Dauerbetrieb als Knoten, auch ohne Bildschirm",
    "mit SSD für einen Voll-Knoten (Stufe 3) oder Validator (Stufe 4)"]],
  [5.6, 5.0, 5.8])
B(["<b>Papier und Stift</b> für Ihre 12 Wiederherstellungs-Wörter. Kein Foto, keine Cloud, kein Notizprogramm.",
   "<b>Internet</b>, am besten WLAN. Im Mobilnetz lädt die App höchstens etwa 20 MB am Tag.",
   "<b>Optional:</b> ein zweites Gerät oder ein zweites Blatt Papier für einen Notfall-Schlüssel (Schritt 7)."])

# ================================================================ 2
H("In fünf Schritten startklar")
STEP(1, "App aus der offiziellen Quelle holen",
     "Laden Sie die <b>TOYPAY Wallet</b> nur über den Link auf der Projektseite " + L("github.com/Safehaigit/toycoin",
                                                                                    U["repo"]) +
     " (öffentlich spätestens mit dem Testnetz). Es gibt sie für Android, Windows, Linux und den Raspberry Pi. Jede "
     "Version wird nachprüfbar aus dem offenen Code gebaut, und die App prüft vor jedem Update selbst, ob die neue "
     "Version echt ist. Apps mit ähnlichem Namen aus anderen Quellen: <b>nicht installieren</b>.")
STEP(2, "Sprache und Modus wählen",
     "Die App startet in der Sprache Ihres Geräts, 45 Sprachen sind geplant. Danach wählen Sie einen von vier Modi. "
     "Für den Anfang empfohlen ist <b>Einsteiger</b>: Jeder Schritt wird erklärt, und es gibt einen Übungs-Modus mit "
     "Spiel-TOY. Sie können jederzeit wechseln. Sicherheitswarnungen erscheinen in jedem Modus gleich.")
STEP(3, "Wallet anlegen und die 12 Wörter sichern",
     "Die App erzeugt Ihre Wiederherstellungs-Phrase aus 12 Wörtern (wahlweise 24, nach dem Standard " +
     L("BIP-39", U["bip39"]) + "). <b>Schreiben Sie sie auf Papier</b> und bewahren Sie es sicher auf. Wer diese Wörter "
     "kennt, kann über Ihr Geld verfügen, und wer sie verliert, kommt bei Verlust des Geräts nicht mehr an sein Geld. "
     "Ein kurzes Quiz fragt einige Wörter ab. Bis Sie es bestanden haben, sperrt die App große Beträge. Zusätzlich "
     "legen Sie ein App-Passwort fest. Die Wörter funktionieren auch in anderen Wallets wie MetaMask, denn die Adressen "
     "sind dieselben wie bei Ethereum.")
STEP(4, "Stufe und Energie-Modus bestätigen",
     "Die App misst kurz Ihr Gerät und schlägt eine Stufe vor. Für den Anfang ist das meist <b>Stufe 0 „Mitmachen ohne "
     "Guthaben“</b>: Ihr Gerät prüft die Kette mit und kann an der Arbeits-Lotterie teilnehmen. Beim Energie-Modus ist "
     "<b>Ausgewogen</b> voreingestellt: Gerechnet wird nur am Ladegerät, im WLAN, mit einem Prozessorkern und solange "
     "das Gerät kühl bleibt. Wird es warm oder benutzen Sie es gerade, pausiert die Arbeit von selbst.")
STEP(5, "Die ersten TOY bekommen",
     "Es gibt drei Wege. <b>Arbeits-Lotterie:</b> Ihr Gerät rechnet Lose, je Block werden Gewinner gezogen, jeder Gewinn "
     "ist mindestens 0,2 TOY. Wie lange das dauert, hängt von der Zahl der Mitspieler ab, die App zeigt eine ehrliche "
     "Schätzung. <b>Testnetz-Kran:</b> Im Testnetz gibt eine Ausgabestelle des Projekts kleine Mengen Test-TOY zum "
     "Ausprobieren aus, begrenzt je Adresse und Tag. <b>Geschenkt:</b> Jemand schickt Ihnen mindestens 0,1 TOY. "
     "Beim ersten Eingang werden 0,1 TOY als <b>Aktivierungs-Pfand</b> gebunden. Sie gehören weiter Ihnen und kommen "
     "beim Auflösen des Kontos vollständig zurück. Ab jetzt ist Ihr Konto aktiv.")

# ================================================================ 3
H("Senden und empfangen")
STEP(6, "Bezahlen und bezahlt werden",
     "<b>Empfangen:</b> Tippen Sie auf „Empfangen“ und zeigen Sie den QR-Code oder geben Sie die Adresse weiter "
     "(beginnt mit 0x). <b>Senden:</b> QR-Code scannen oder Adresse aus dem Adressbuch wählen, Betrag eingeben, "
     "optional einen Verwendungszweck. Vor dem Unterschreiben zeigt die App eine Vorschau mit Guthaben vorher und "
     "nachher, Gebühr und Hinweis, falls der Empfänger neu ist. Nach etwa <b>einer Minute</b> ist die Zahlung "
     "endgültig.")
T([["Was", "Wie es läuft"],
   ["Kosten", "Die ersten 10 bis 100 Transaktionen am Tag sind <b>kostenlos</b>, sobald Ihr Konto etwa 100 Minuten alt "
              "ist. Danach kostet eine Überweisung beim Start rund 0,00012 TOY. Die App wählt das selbst."],
   ["Neue Empfänger", "An eine Adresse, die noch nie TOY hatte, müssen mindestens 0,1 TOY gehen (das Pfand). Die App "
                      "prüft das. Bei der ersten Zahlung über 10 TOY an eine neue Adresse schlägt sie einen kleinen "
                      "Probebetrag vor."],
   ["Verwechslung", "Die App warnt, wenn eine Adresse einer bekannten nur ähnlich sieht oder wenn die Adresse in der "
                    "Zwischenablage heimlich ausgetauscht wurde. Prüfen Sie trotzdem Anfang und Ende der Adresse."],
   ["Hängende Zahlung", "Jede Zahlung gilt nur eine Zeit lang (Standard rund 100 Minuten). Sie lässt sich bis dahin "
                        "mit einem Tipp ersetzen oder abbrechen."]],
  [3.4, 13.0])

# ================================================================ 4
H("Konto schützen (empfohlen)")
STEP(7, "Sicherheits-Check durchgehen",
     "Die App zeigt eine Ampel: Wörter gesichert? Notfall-Schlüssel eingerichtet? Version aktuell? Gerät in Ordnung? "
     "Jeder Punkt hat einen Tipp zum Beheben. Ein Assistent richtet den freiwilligen Konto-Schutz ein:")
B(["<b>Notfall-Schlüssel und Sicherheitsadresse:</b> ein zweiter Schlüssel auf einem anderen Gerät oder auf Papier. "
   "Er kann nichts ausgeben, aber im Notfall alles an eine vorher festgelegte, sichere Adresse retten.",
   "<b>Tresor-Verzögerung:</b> Größere Abbuchungen werden erst nach einer Wartezeit wirksam, zum Beispiel 24 Stunden. "
   "In dieser Zeit können Sie eine verdächtige Abbuchung stoppen.",
   "<b>Selbst-Sperre:</b> Handy verloren? Mit dem Notfall-Schlüssel oder einem zweiten Gerät sperren Sie Ihr Konto "
   "für bis zu 30 Tage.",
   "<b>Erben-Adresse:</b> Bleibt Ihr Konto sehr lange unbenutzt (mindestens 180 Tage), kann eine selbst gewählte "
   "Adresse es übernehmen. Die App erinnert Sie vorher mehrfach. Das ersetzt kein Testament."])
P("Diese Einstellungen gelten nur, weil Sie sie selbst gesetzt haben. Niemand sonst, auch nicht das Projekt, kann sie "
  "setzen oder Ihr Guthaben sperren. Änderungen am Schutz werden frühestens nach zwei Tagen wirksam, damit ein Dieb "
  "sie nicht sofort umstellen kann.")

# ================================================================ 5
H("Mehr beitragen (freiwillig)")
P("Wer TOY als Pfand bindet (Stake), bekommt mehr Aufgaben und einen Anteil an den Belohnungen. Gebundene TOY bleiben "
  "Ihr Eigentum und lassen sich mit 21 Tagen Wartezeit wieder lösen. Die App bietet eine Stufe nur an, wenn Ihr Gerät "
  "sie schafft, und nennt vorher ehrlich Kosten und Risiken. Eine Rendite wird nicht versprochen.")
T([["Stufe", "Voraussetzung", "Was Ihr Gerät tut"],
   ["0 · Leicht", "nichts", "Kette mitprüfen, Transaktionen weiterleiten, Lose rechnen"],
   ["1 · Konto", "aktiviertes Konto (0,1 TOY Pfand)", "zusätzlich senden mit kostenlosem Kontingent"],
   ["2 · Attestierer", "10 TOY Stake", "Blöcke mitunterschreiben, Anteil an Belohnung und Gebühren-Lotterie"],
   ["3 · Voll", "100 TOY Stake, 256 MB Daten, gut erreichbar", "Zustand halten und andere Knoten beliefern"],
   ["4 · Validator", "1.000 TOY Stake, dauerhaft eingeschaltet", "Blöcke vorschlagen und im Komitee abstimmen"]],
  [3.0, 5.6, 7.8])
H2("Raspberry Pi oder PC als Dauer-Knoten")
P("Für Stufe 3 und 4 gibt es den <b>ToyNode</b> ohne Oberfläche. Er läuft als Dienst, wird über eine lokale "
  "Weboberfläche oder die Kommandozeile (<font face='Sans'>toy-cli</font>) bedient und startet nach einem Stromausfall "
  "von selbst. Wichtig: Schalten Sie das Mitunterschreiben nur auf <b>einem</b> Gerät ein. Die App erzeugt dafür einen "
  "eigenen Signier-Schlüssel und hilft beim Umzug auf ein neues Gerät. Läuft derselbe Schlüssel auf zwei Geräten, "
  "droht eine Strafe für Doppelsignatur. Für Validatoren gilt: Ein Gerät, das wegen Akku oder Hitze ausfällt, verliert "
  "Belohnung und wird pausiert.")

# ================================================================ 6
H("MetaMask oder eine andere Wallet nutzen (freiwillig)")
P("Weil ToyChain dieselben Adressen wie Ethereum nutzt, können Sie Ihr Konto auch in " +
  L("MetaMask als eigenes Netz eintragen", U["metamask"]) + ". Später geht das mit einem Klick auf der Projektseite. "
  "Von Hand sind es diese Werte:")
T([["Feld", "Wert für das Testnetz"],
   ["Netzwerkname", "ToyChain Testnet"],
   ["Chain-ID", "7630714 (Hauptnetz später: 7630713)"],
   ["Währungssymbol", "TOY"],
   ["RPC-Adresse", "wird mit dem öffentlichen Testnetz auf der Projektseite veröffentlicht. Oder Ihr eigener ToyNode"]],
  [4.0, 12.4])
P("In MetaMask können Sie TOY senden und empfangen. Wenn Sie den Gaspreis auf 0 stellen, nutzen Sie Ihr kostenloses "
  "Kontingent, sonst zahlen Sie die kleine Gebühr. Stake, Konto-Schutz, Spenden und die Arbeits-Lotterie gibt es nur "
  "in der TOYPAY Wallet. Hardware-Wallets wie Ledger oder Trezor können Überweisungen unterschreiben.")

# ================================================================ 7
H("Fünf goldene Regeln")
BOX("<b>1. Ihre 12 Wörter gehören nur Ihnen.</b> Niemand vom Projekt, kein Support und keine App fragt jemals danach. "
    "Wer danach fragt, will Sie bestehlen.<br/>"
    "<b>2. Gleichnamige Token sind kein TOY.</b> Auf Polygon und Ethereum gibt es ältere Token namens ToyCoin mit dem "
    "Kürzel TOY. Sie haben mit ToyChain nichts zu tun und lassen sich nicht eintauschen.<br/>"
    "<b>3. Testnetz-TOY sind Spielgeld.</b> Sie werden nie in ein Hauptnetz übernommen. Wer sie verkauft oder kauft, "
    "handelt mit nichts.<br/>"
    "<b>4. Nur die offizielle App.</b> Nur über den Link auf der Projektseite installieren und Updates nur aus der App "
    "selbst annehmen.<br/>"
    "<b>5. Erst prüfen, dann senden.</b> Eine endgültige Zahlung lässt sich nicht zurückholen. Im Zweifel erst einen "
    "kleinen Betrag schicken.")
P("Allgemeine Hinweise zu Risiken von Kryptowerten gibt die " + L("Verbraucherzentrale", U["vz"]) + ".", small)

# ================================================================ 8
H("Häufige Fragen")
faq = [
    ("Kostet das Mitmachen etwas?",
     "Nein. Stufe 0 braucht kein Guthaben. Ihr Gerät verbraucht etwas Strom, rechnet aber im Standard-Modus nur am "
     "Ladegerät."),
    ("Was passiert, wenn mein Handy kaputt ist oder gestohlen wird?",
     "Mit Ihren 12 Wörtern stellen Sie die Wallet auf einem neuen Gerät wieder her. Gegen einen Dieb hilft am besten "
     "ein vorher eingerichteter Notfall-Schlüssel: Damit sperren Sie das Konto oder retten alles an Ihre "
     "Sicherheitsadresse."),
    ("Kann mein Konto verschwinden?",
     "Nein. Lange unbenutzte Konten werden nur aus dem Speicher der Knoten ausgelagert. Mit Ihrer nächsten Zahlung "
     "weckt die App das Konto automatisch mit vollem Guthaben wieder auf."),
    ("Kann jemand mein Guthaben sperren oder wegnehmen?",
     "Nein. Das verbietet die Verfassung von ToyChain, auch für spätere Updates. Sperren nach Regelverstößen nehmen nur "
     "Rechte weg, zum Beispiel das kostenlose Kontingent, nie Guthaben. Überweisen gegen Gebühr geht immer."),
    ("Wofür ist das Pfand von 0,1 TOY?",
     "Es verhindert, dass jemand die Knoten mit Millionen leerer Konten füllt. Es bleibt Ihres und kommt beim "
     "Auflösen des Kontos zurück. Es darf später nur sinken, nie steigen."),
    ("Wann geht es los?",
     "Wallet und App entstehen in den Meilensteinen M8 und M9, das Testnetz mit Ausgabestelle in M10, zuerst lokal, "
     "dann öffentlich. Einen festen Termin gibt es nicht. Ein Hauptnetz kommt erst nach Sicherheitsprüfung, "
     "Lasttests und rechtlicher Prüfung, und sein Start wird vorher öffentlich angekündigt."),
]
for q, a in faq:
    story.append(KeepTogether([Paragraph(q, h2), Paragraph(a, body)]))

story.append(Spacer(1, 6))
P("<b>Haftungsausschluss.</b> Diese Anleitung beschreibt geplante Software, die noch nicht veröffentlicht ist. "
  "Abläufe, Namen und Zahlen können sich ändern. Sie ist keine Anlage-, Steuer- oder Rechtsberatung. Maßgeblich sind "
  "die technische Spezifikation und das Whitepaper (Entwurf 0.4) im Projekt-Repository.", small)

finish("ToyChain-Kurzanleitung-v0.4.pdf", "ToyChain · Kurzanleitung für Nutzer · Entwurf 0.4 · vor der Testphase",
       "Seite", "ToyChain einrichten: Kurzanleitung für Nutzer", "Entwurf 0.4, vor der Testphase", "de-DE")
print("ok")
