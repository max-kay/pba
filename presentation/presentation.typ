#import "@preview/polylux:0.3.1": *
#import "theme.typ": *
#import "@preview/whalogen:0.1.0": ce

#let header = {
  pad(0%, image("../figs/strip.png", width: 100%))
}

#show: theme.with(
  header: header
)

#title-slide[
  = Vacancy structure in manganese hexakiscyanocobaltate
  #image("../figs/Mn[Co].png", height: 60%)
  Max Krummenacher
]
#slide[
  == Crystal structure of Mn[Co]
  #columns(2)[
    #only((until: 2))[
      #align(center)[* 18 VE rule (octahedral symmetry)*]
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
    ]
    #colbreak()
    #only((beginning: 3))[
      #align(center)[* with charge neutrality*]
      ]
    #only(3)[
      
      #v(0.8fr)
      #ce("Co^III")
      #v(1fr)
      #ce("CN^-")
      #v(1fr)
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
]
#slide[
== Monte Carlo simulation
]
#slide[
== Results
]