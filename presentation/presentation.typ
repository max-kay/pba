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


#let header = {
  pad(0%, image("../figs/strip.png", width: 100%))
}

#show: theme.with(
  header: header
)

#title-slide[
  = Vacancy structure in prussian blue analgues
  #image("../figs/Mn[Co].png", height: 60%)
  Max Krummenacher
]

#slide[
  == Prussian blue analogues
  #columns(2,
    [
      #image("../figs/starry_night.jpg") 
      #colbreak()
      #align(center+horizon, uncover("2-", ce("M[M'(CN)_6]")))
    ]
  )
]

#two_col_slide(
  [== Crystal structure of #ce("Fe[Fe(CN)_6]")],
  [#only((until: 2))[
      #align(center)[* 18 VE rule (octahedral) *]
    ]
    #only((beginning: 3))[
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
    ],
    [
      #only("4-")[
        #align(center)[* with charge neutrality*]
      ]
    #only(4)[
      
      #v(0.8fr)
      #ce("[Fe^II (CN)_6]^-4")
      #v(1fr)
      #ce("Fe^III")
      #v(1.5fr)
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
)

#two_col_slide(
  [== Crystal structure of #ce("Mn[Co(CN)_6]")],
  [#only((until: 2))[
      #align(center)[* 18 VE rule (octahedral) *]
    ]
    #only((beginning: 3))[
      #align(center)[* without charge neutrality*]
      ]
    #only(1)[
      #table(columns: 2, stroke: none,
        ce("Co^III"), ce("6e^-"),
        ce("CN^-"), ce("2e^-"),
        ce("6(CN^-)"), ce("12e^-"),
      )
    ]
    #only((beginning: 2))[
      #align(
        center + horizon, 
        image("../figs/Mn[Co].png", width: 50%)
      )
    ]],
    [
      #only((beginning: 3))[
        #align(center)[* with charge neutrality*]
      ]
    #only(3)[
      
      #v(0.8fr)
      #ce("[Co^III (CN)_6]^-3")
      #v(1fr)
      #ce("Mn^II")
      #v(1.5fr)
      #ce("Mn^II [Co^III (CN)_6]_⅔")
    ]
    #only(4)[
      #align(
        center + horizon, 
        image("../figs/vacancies.png", width: 50%)
      )
    ]
  ]
)

#two_col_slide(
  [== Monte Carlo simulation],
  [
    *States*
    #uncover("2-")[
    - Grid with fixed #ce("Mn")-ions
    ]
    #uncover("3-")[
    - ⅔ of cyanocobaltate positions filled
    ]
  ],
  [
    *Hamiltonian*
    #uncover("4-")[
      - nearest neighbor $J_1$
    ]
    #only(4, align(center, image("../figs/nearest.png", height: 60%)))
    #uncover("5-")[
      - next nearest neighbor $J_2$
    ]
    #only(5, align(center, image("../figs/next_nearest.png", height: 60%)))
  ]
)
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

