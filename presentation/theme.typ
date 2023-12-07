// modified copy of the simple theme

#import "@preview/polylux:0.3.1": *

#let footer_state = state("footer", [])
#let header_state = state("header", [])

#let theme(
  aspect-ratio: "16-9",
  footer: [],
  header: [],
  background: black,
  foreground: white,
  body
) = {
  set page(
    paper: "presentation-" + aspect-ratio,
    margin: 2em,
    header: none,
    footer: none,
    fill: background,
  )
  set text(fill: foreground, size: 25pt)
  show footnote.entry: set text(size: .6em)
  show heading.where(level: 2): set block(below: 2em)
  set outline(target: heading.where(level: 1), title: none, fill: none)
  show outline.entry: it => it.body
  show outline: it => block(inset: (x: 1em), it)

  footer_state.update(footer)
  header_state.update(header)

  body
}

#let centered-slide(body) = {
  logic.polylux-slide(align(center + horizon, body))
}

#let title-slide(body) = {
  set heading(outlined: false)
  centered-slide(body)
}

#let focus-slide(background: aqua.darken(50%), foreground: white, body) = {
  set page(fill: background)
  set text(fill: foreground, size: 1.5em)
  logic.polylux-slide(align(center + horizon, body))
}

#let slide(body) = {
  let deco-format(it) = text(size: .6em, fill: gray, it)
  set page(
    header: header_state.display(),
    footer: deco-format({
      footer_state.display(); h(1fr); logic.logical-slide.display()
    }),
    footer-descent: 1em,
    header-ascent: 1em,
  )
  logic.polylux-slide(body)
}

#let two_col_slide(title, col1, col2) = {
  slide(
    [
      #title
      #columns(2,
      [#col1 #colbreak() #col2]
      )
    ]
  )
}
