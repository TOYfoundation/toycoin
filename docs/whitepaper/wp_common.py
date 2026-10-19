"""Shared layout helpers for the English and French whitepaper builds (build_en.py, build_fr.py)."""
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
WORDS = {"figure": "Figure", "more": "More:"}
FILTER = [lambda t: t]


def set_filter(fn):
    FILTER[0] = fn


def _f(t):
    return FILTER[0](t) if isinstance(t, str) else t


def set_words(figure, more):
    WORDS["figure"] = figure
    WORDS["more"] = more



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
    story.append(Paragraph(_f(text), style))


def MORE(*links):
    P(f"<b>{WORDS['more']}</b> " + " · ".join(links), more)


def B(items, style=body):
    story.append(ListFlowable([ListItem(Paragraph(_f(t), style), leftIndent=12, value="•") for t in items],
                              bulletType="bullet", start="•", leftIndent=12, bulletFontName="Serif",
                              bulletFontSize=8, spaceAfter=4))


def T(rows, widths, header=True):
    data = [[Paragraph(_f(c), cellb if (header and i == 0) else cell) for c in r] for i, r in enumerate(rows)]
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
    t = Table([[Paragraph(_f(text), ParagraphStyle("bx", parent=body, fontName="Sans", fontSize=8.8, leading=12.4,
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
                               Paragraph(_f(f"{WORDS['figure']} {fig_no[0]}: {text}"), caption)]))




def toc_table(items, heading):
    half = (len(items) + 1) // 2
    rows = [[Paragraph(a, toc), Paragraph(b, toc)] for a, b in
            zip(items[:half], items[half:] + [""] * (2 * half - len(items)))]
    tt = Table(rows, colWidths=[8.2 * cm, 8.2 * cm])
    tt.setStyle(TableStyle([("TOPPADDING", (0, 0), (-1, -1), 0.5), ("BOTTOMPADDING", (0, 0), (-1, -1), 0.5),
                            ("LEFTPADDING", (0, 0), (-1, -1), 1.1 * cm)]))
    story.append(Paragraph(heading, ParagraphStyle("ti", parent=h2, leftIndent=1.1 * cm)))
    story.append(tt)


def finish(filename, footer, page_word, title_text, subject, lang):
    def on_page(c, doc):
        c.saveState()
        c.setFont("Sans", 7.5)
        c.setFillColor(MUTED)
        c.drawString(2.3 * cm, 1.3 * cm, footer)
        c.drawRightString(A4[0] - 2.3 * cm, 1.3 * cm, f"{page_word} {doc.page}")
        c.setStrokeColor(RULE)
        c.setLineWidth(0.5)
        c.line(2.3 * cm, 1.65 * cm, A4[0] - 2.3 * cm, 1.65 * cm)
        c.restoreState()

    doc = BaseDocTemplate(filename, pagesize=A4, leftMargin=2.3 * cm, rightMargin=2.3 * cm, topMargin=2.0 * cm,
                          bottomMargin=2.1 * cm, title=title_text, author="ToyChain project", subject=subject,
                          lang=lang)
    frame = Frame(doc.leftMargin, doc.bottomMargin, doc.width, doc.height, id="f", leftPadding=0, rightPadding=0,
                  topPadding=0, bottomPadding=0)
    doc.addPageTemplates([PageTemplate(id="p", frames=[frame], onPage=on_page)])
    doc.build(story)
