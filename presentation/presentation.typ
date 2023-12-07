#import "@preview/polylux:0.3.1": *
#import "theme.typ": *
#import "@preview/whalogen:0.1.0": ce


#let display_diffraction(img, j, t) = [
  #grid(
    columns: 2,
    align(center, image(img, width: 60%)),
    align(horizon, 
      grid(
        columns: 1,
        gutter: 20pt,
        align(center, $J_1/J_2 = #j$),
        align(center, t),
      )
    )
  )
]

#let legend(color, letter) = text(color)[*#letter*]

#let M_COLOR = rgb(37%, 42%, 82%)
#let M_PRIME_COLOR = rgb(75%, 8%, 77%)
#let C_COLOR = rgb(60%, 6%, 6%)
#let N_COLOR = rgb(17%, 67%, 24%)


#let header = {
  pad(0%, image("../figs/strip.png", width: 100%))
}

#show: theme.with(
  header: header
)

#title-slide[
  = Vacancy structure in Prussian blue analgues
  #image("../figs/Mn[Co].png", height: 60%)
  Max Krummenacher
]

#slide[
  == Prussian blue analogues
  #columns(2,
    [
      #image("../figs/starry_night.jpg") 
      #colbreak()
      #v(1fr)
      #align(center, uncover("2-", ce("M[M'(CN)_6]")))
      #v(1fr)
    ]
  )
]

#slide[
  == Crystal structure of #ce("Fe[Fe(CN)_6]_¾")
  #columns(2)[#only("-2")[
      #align(center)[* 18 VE rule (octahedral) *]
    ]
    #only("3-")[
      #align(center)[* without charge neutrality*]
      ]
    #only(1)[
      #table(columns: 2, stroke: none,
        ce("Fe^II"), ce("6e^-"),
        ce("CN^-"), ce("2e^-"),
        ce("6(CN^-)"), ce("12e^-"),
      )
    ]
    #only(2)[
      #align(
        center + horizon, 
        image("../figs/cyanometallate.png", width: 50%)
      )
    ]
    #only("3-")[
      #align(
        center + horizon,
        image("../figs/fefe.png", width: 50%)
      )
    ]
    #colbreak()
      #only("4-")[
        #align(center)[* with charge neutrality*]
      ]
    #only(4)[
      #v(10pt)
      #ce("[Fe^II (CN)_6]^-4")
      #v(20pt)
      #ce("Fe^III")
      #v(30pt)
      #ce("Fe^III_4 [Fe^II (CN)_6]_3")
    ]
    #only(5)[
      #align(
        center + horizon, 
        image("../figs/fefe_vac_v1.png", width: 50%)
      )
    ]
    #only(6)[
      #align(
        center + horizon, 
        // image("../figs/vacancies.png", width: 50%)
        image("../figs/fefe_vac_v3.png", width: 50%)
      )
    ]
  ]
  #h(0.3fr)
  #only("2-")[
    #legend(M_COLOR, ce("Fe^III")) #h(1fr)
    #legend(M_PRIME_COLOR, ce("Fe^II")) #h(1fr)
    #legend(C_COLOR, ce("C")) #h(1fr)
    #legend(N_COLOR, ce("N"))
  ]
  #only(6)[
    #h(1fr)
    #legend(rgb(80%, 80%, 80%), ce("Vacancy"))
  ]
  #h(0.3fr)
  
]

#slide[
  == Crystal structure of #ce("Mn[Co(CN)_6]_⅔")
  #columns(2)[#only((until: 2))[
      #align(center)[* 18 VE rule (octahedral) *]
    ]
    #only("3-")[
      #align(center)[* without charge neutrality*]
      ]
    #only(1)[
      #table(columns: 2, stroke: none,
        ce("Co^III"), ce("6e^-"),
        ce("CN^-"), ce("2e^-"),
        ce("6(CN^-)"), ce("12e^-"),
      )
    ]
    #only("2-")[
      #align(
        center + horizon, 
        image("../figs/Mn[Co].png", width: 50%)
      )
    ]
    #colbreak()
      #only("3-")[
        #align(center)[* with charge neutrality*]
      ]
    #only(3)[
      #v(10pt)
      #ce("[Co^III (CN)_6]^-3")
      #v(20pt)
      #ce("Mn^II")
      #v(30pt)
      #ce("Mn^II [Co^III (CN)_6]_⅔")
    ]
    #only(4)[
      #align(
        center + horizon, 
        image("../figs/vacancies.png", width: 50%)
      )
    ]
  ]
  #only("2-")[
    #h(0.3fr)
    #legend(M_COLOR, ce("Mn")) #h(1fr)
    #legend(M_PRIME_COLOR, ce("Co")) #h(1fr)
    #legend(C_COLOR, ce("C")) #h(1fr)
    #legend(N_COLOR, ce("N")) #h(0.3fr)
  ]
]

#slide[
  == Monte Carlo simulation
  #columns(2)[
  *States*
  #uncover("2-")[
  - Grid with fixed #ce("Mn")-ions
  ]
  #uncover("3-")[
  - ⅔ of cyanocobaltate positions filled
  ]
  #colbreak()
  *Hamiltonian*
  #uncover("4-")[
    - nearest neighbor $J_1$
  ]
  #only(4, align(center, image("../figs/nearest.png", height: 45%)))
  #uncover("5-")[
    - next nearest neighbor $J_2$
  ]
  #only(5, align(center, image("../figs/next_nearest.png", height: 45%)))
  #only("4-")[
    #h(1fr)
    #legend(M_PRIME_COLOR, ce("Co"))
    #h(1fr)
    #legend(M_COLOR, ce("Mn"))
    #h(1fr)
  ]
]
]
#slide[
  == Diffraction
  #grid(
    columns: (1fr, 1fr),
    rows: (1fr, 1fr),
    display_diffraction("../figs/j_0.4000001_t_54.59815.png", 0.4, [high $T$]),
    display_diffraction("../figs/j_4.8_t_54.59815.png", 4.8, [high $T$]),
    uncover("2-", display_diffraction("../figs/j_0.4000001_t_0.13533528.png", 0.4, [low $T$])),
    uncover("2-", display_diffraction("../figs/j_4.8_t_0.13533528.png", 4.8, [low $T$])),
  )

]
#slide[
#align(center)[== Thank you for your attention]

  #align(center + horizon, image("../figs/Mn[Co].png", height: 70%))
]