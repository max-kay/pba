#import "@preview/whalogen:0.1.0": ce
#set heading(numbering: "1.")
#set page(
  numbering: "1",
)
#set enum(numbering: "1a)")
#set par(justify: true)
#set math.equation(numbering: "(1)")

#let M_COLOR = rgb(37%, 42%, 82%)
#let M_PRIME_COLOR = rgb(75%, 8%, 77%)
#let C_COLOR = rgb(60%, 6%, 6%)
#let N_COLOR = rgb(17%, 67%, 24%)

#let crystallo(string) = {
  let replacer(match) = {
    let buffer = ""
    for digit in match.at("captures").at(0).codepoints(){
      buffer = buffer + digit + "\u{0305}"
    }
    buffer
  }
  string = string.replace("<", sym.angle.l)
  string = string.replace(">", sym.angle.r)
  string = string.replace(regex("-(\d)\b"), replacer)
  string = string.replace(" ", "")
  string
}


#align(center, text(17pt)[
 *Vacancy structure in Prussian blue analogues*
]) 
#align(center)[Max Krummenacher]
#v(20pt)
#align(center)[Semester project HS23 \ BSc material science and engineering ETH Zürich]

#align(center)[ 
  #set par(justify: false)
  *Abstract* \
  This will be my abstract.
]

= Introduction

== Prussian Blue Analogues
For the purposes of this report we consider Prussian blue analogues (PBA) to be compounds with composition $#ce("M[M'(CN)_6]")_text(x)$ where M and M' are transition metals and x is the fill fraction of the hexacyanometalates.
These idealized structure of these compounds is the face centered cubic structure shown in @fcc.
#figure(
  image("../figs/fcc.png", width: 50%),
  caption: [The structure of #text(M_COLOR, "M") \[#text(M_PRIME_COLOR, "M'") (#text(C_COLOR, "C")#text(N_COLOR, "N"))#sub("6")\]]
)<fcc>
However this exact structure is only achievable if the M ion has the same charge as the #ce("[M'(CN)_6]") ion in other words if charge balance allows $x=1$. An example of such a PBA is #ce("Pt[Pt(CN)_6]"). If this condition is not met then some of the hexacyanometalate sites remain unoccupied.

Prussian blue itself has the composition #ce("Fe^III [Fe^II (CN)_6]_¾"). The fill fraction of ¾ allows the vacancies to be arranged in a periodic fashion in the cubic crystal structure. These vacancies aren't true vacancies in the strict sense. They don't remain empty. In the case of Prussian blue they are filled with water or depending on the production method might even be filled with alkali metal ions. This further complicates the condition of charge balance and is one of the reasons why the structure of Prussian blue has long been a mystery.


#figure(
  image("../figs/fefe_vac_w.png", width: 50%),
  caption: [Arrangement of vacancies in Prussian blue @ray_tracer]
)

In other PBAs like for example #ce("Mn^II [Co^III (CN)_6]_⅔") the fill fraction might not allow for a periodic arrangement of vacancies in a cubic crystal. This leads the crystal to be a frustrated system, as there is no singular state with lowest energy.

// write about vacancies avoiding each other.
Vacancies in the crystal can be interpreted as a micropore. If two vacancy lie next to each other they connect to form a larger micropore. Thus the structure of vacancies in the crystal is important for many transport properties of the bulk material. Prussian blue has found application in medicine against Cs poisoning as Cs ions fit within the vacancies of the crystal where they remain immobilized.
Other applications for the microporosity of PBAs include the storage of gases (eg. Hydrogen) and proton conduction.

In this paper we explore the arrangement of vacancies of PBA with fill fraction $x=⅔$ through a Monte Carlo simulation using a simple nearest and next nearest neighbor interaction approximation for the hamiltonian.


== Diffuse scattering
The Laue process is a common crystallographic method by which the Laue group of crystal and crystal parameters can be determined.
In the Laue method a beam of X-rays of a band of wavelengths is diffracted by a single crystal. This causes the beam to diffract at the atoms according to the following formula:
$ F_(h k l) = sum _j f_j exp(2 pi i mat(h, k, l;) vec(a_j, b_j, c_j)) $
Where $F_(h k l)$ is the complex structure factor at position $mat(h, k, l;)$ in the reciprocal space, $f_j$ is the atomic form factor of the atom at position $vec(a_j, b_j, c_j)$. In a perfect infinitely large crystal the structure factor is zero everywhere except at integer valued $mat(h, k, l)$.
Note that the Laue method measures the intensity $I = abs(F)^2$ such that the phase of $F_(h k l)$ is lost.

The best use case for the Laue method is on a perfect single crystal. If the periodicity of the crystal is broken the peaks measured broaden and become more diffuse. However if there is some statistical pattern in the aperiodicity of the crystal (as in PBAs) the change of the shape of the peaks is characteristic of this statistical pattern. @nature


== Monte Carlo Simulation
For a system, of which we know all possible states $sigma in Omega$ and the hamiltonian for each state $cal(H)(sigma)$, we can obtain a macroscopic view of the system by the Monte Carlo method.
Suppose the probability of a state occurring $rho (sigma)$ at some temperature $T > 0$ is proportional to $exp(- (cal(H) (sigma))/(k_b T))$. // find reference for this
We know the probabilities sum to 1.
$ 1 = sum_(sigma in Omega) (exp(-(cal(H) (sigma))/(k_b T)))/Z = 1/Z sum_(sigma in Omega) exp(- (cal(H)(sigma))/(k_b T)) $

$ => Z = sum_(sigma in Omega) exp(- (cal(H)(sigma))/(k_b T)) $
thus:
$ rho (sigma) = exp(-(cal(H) (sigma))/(k_b T))/Z $<probability>

We define macroscopic variables of state in terms of the expected value of their associated microscopic variables.
For example for the internal energy we define:
$ U = angle.l cal(H) angle.r = sum_(sigma in Omega) cal(H) (sigma) rho(sigma) $

Although each term of theses sums are simple set of states over which the sum is taken are often mathematically complex and as such very hard to solve analytically. Though these sums are computable for systems with a small number of states to obtain statistically relevant results a large system needs to be chosen and the calculation of the expectance value becomes computationally expensive.
In the Monte Carlo simulation we find an approach to calculate the expected value of these variables without iterating through all states.

The fundamental idea behind the technique is to generate a series of states $sigma_i$ drawn from the probability distribution defined by $rho (sigma)$ (@probability). Variables of state can then be calculated as the average of the microscopic variable.

For this simulation these draws are generated by the following process:
+ Start with some randomly generated state.
+ Make a small change to the state.
+ Compare the hamiltonians of the states.
  + If the hamiltonian is lower in the new state the new state is accepted.
  + If the hamiltonian is higher the new state is accepted with probability $rho = exp(- (Delta cal(H))/(k_b T))$.
+ back to step 2)


= Methods
// write about supercells

== Simulation
The simulation is written in Rust.

=== States
The structure is modeled as a three dimensional square grid containing $N$ unit cells. To represent the fcc structure of the PBA, a grid of size $2N times 2N times 2N$ is created and all positions with $i+j+k eq.triple 0 (mod 2)$ are populated with $0$ representing the fixed M sites. The other positions are populated with -1 or 1 representing a vacancy or a cyanometalate respectively. To achieve the exact fill fraction a vector containing the correct number of -1 and 1 is generated and then shuffled to achieve a random starting state. Note that this creates a three dimensional version of a checkerboard pattern where each direct neighbor to an -1/1 site is a 0 site. Additionally the grid is accessed in a modular fashion such that $e_(i,j,k) = e_(i+2N, j, k) = e_(i, j+2N, k) = e_(i, j, k+2N)$.\

=== Hamiltonian
The Hamiltonian of the system is divided into 2 terms. Only interactions between cyanometalates and vacancies are considered. For the nearest neighbor with an offset of $1/2$#crystallo("<1 1 0>") if both of them are the same (vacancies or metalates) their contribution to the total energy is $J_1$ otherwise it is $-J_1$. Similarly for next nearest neighbors along #crystallo("<1 0 0>") directions, if both are the same their contribution is $J_2$, $-J_2$ otherwise.\
#figure(
  grid(columns: (1fr, 2fr, 2fr, 1fr),
    [],
    image("../figs/nearest_w.png", width: 100%), image("../figs/next_nearest_w.png", width: 100%),
    [],
  ),
  caption: [Images showing the nearest and next nearest neighbors@ray_tracer\ of #text("M'", M_PRIME_COLOR) around #text("M", M_COLOR) left, right respectively]
)<neighbor>
Let $I = {(i, j, k) | i+j+k eq.triple 1 (mod 2), 0 <= i, j, k, < 2N}$ be the set of all indexes of possible cyanometalate sites and $sigma_(i, j, k)$ the value at index $(i, j, k)$. With this notation we can express the hamiltonian in the following way:
$
cal(H) (sigma)&= sum_(arrow(r)_0 in I) (
  1/2 sum_(arrow(r) in#crystallo("<1 1 0>")) J_1 sigma_(arrow(r)_0) sigma_(arrow(r)_0 + arrow(r))
  + 1/2 sum_(arrow(r) in 2#crystallo("<1 0 0>")) J_2 sigma_(arrow(r)_0) sigma_(arrow(r)_0 + arrow(r)))\
  &= J_1 (1/2 sum_(arrow(r)_0 in I) sum_(arrow(r) in#crystallo("<1 1 0>")) sigma_(arrow(r)_0) sigma_(arrow(r)_0 + arrow(r)))
  + J_2 (1/2 sum_(arrow(r)_0 in I) sum_(arrow(r) in 2#crystallo("<1 0 0>")) sigma_(arrow(r)_0) sigma_(arrow(r)_0 + arrow(r)))\
  &= J_1 s_1(sigma) + J_2 s_2(sigma)
$
Where the factors of $1/2$ correct for counting each neighbor interaction twice and the functions $s_1(sigma)$ and $s_2(sigma)$ correspond to the sum over nearest neighbor and next nearest neighbor respectively. (Note that here #crystallo("<1 1 0>") and #crystallo("<1 0 0>") correspond to offsets of the indexes into the array and not crystallographic vectors)\
Instead of calculating the hamiltonian directly the sums $s_1$ and $s_2$ are computed and updated in each step. As these sums are of integer value they allow avoiding float inaccuracies. \

=== Parameters
For each Monte Carlo step two of the vacancy/cyanometalate sites, which aren't the same are chosen uniformly from the whole grid. To calculate the difference in energy, if a swap was to be performed is calculated by calculating how the sums over nearest neighbors and next nearest neighbors. If this difference in energy is smaller than zero the swap is accepted, otherwise the swap is allowed with a probability $p=exp(-(Delta E)/(k_b T))$.\
As by the consideration in the appendix @parameters only two new parameters $J' = J_1/J_2$ and $T' = (k_b T)/J_2$ need to be explored to get an overview over the whole parameter space.
For the simulation an ensemble of 20 models with different J' terms is simulated. Each model is tempered from some initial temperature $T_0$ to $T_(n-1)$. The temperature steps are chosen such that $ln(T_i)$ is a linear function of i. At each temperature the model is left to equilibrate for 500 epochs before any measurements are taken from another 500 epochs. Where an epoch refers to $8 S$ Monte Carlo steps with the number of vacancy/cyanometalate sites $S$.

=== Output
At the end of each temperature step a `.mmcif` file is generated from the model, which is a crystallographic file format which contains information about positions of atoms in the supercell. All values from the measurements are logged to a `.csv` file.


=== Optimizations
For the compiler to be able to fully optimize the program, where possible the size of list of values, like the grid is give at compile time through the use of `[i8; N]` instead of `Vec<i8>`. Though the Monte Carlo method itself is not parallelizable many systems of different $J'$s can be run at the same time. For this the crate `rayon` was used.

== Data Analysis
On the `.mmcif` files produced by the simulation the command line tool `gemmi sfcalc` is run. This program calculates the structure factors using the fast Fourier transform for the crystal provided in the input file. `gemmi` is called from a python script which then converts the data into the `Yell` format.
#pagebreak()

= Results
#align(center)[
  #figure(
    image("../figs/2023-11-16_17-01.svg", width: 80%),
    caption: "Internal Energy of the system and the natural log of its derivative with respect to temperature"
  )<e_plot>
]

#align(center)[
  #figure(
    grid(columns: 3, row-gutter: 2mm, column-gutter: 1mm,
      image("../figs/examples/a.png", width: 60%), image("../figs/examples/b.png", width: 60%), image("../figs/examples/c.png", width: 60%),
      "a)", "b)", "c)",
      image("../figs/examples/d.png", width: 60%), image("../figs/examples/e.png", width: 60%), image("../figs/examples/f.png", width: 60%),
      "d)", "e)", "f)"
    ),
    caption: [Diffraction patterns at the positions labeled in @e_plot]
  )<patterns>
]

@e_plot shows the energy calculated during the simulation. In the plot of the derivative of the internal energy invalid values resulting from the log were replaced by the minimum values.


= Discussion
In @e_plot phase transitions can be clearly recognized. In the plot of the derivative of the internal energy with respect to temperature we can see the limitations of the Monte Carlo method at lower temperatures. This is to be expected as the at lower temperatures more ordered structures are expected. and the structure might be in a local minimum of energy which cannot be left because of the low thermal energy.
The change of the structures can be seen in the diffraction pattern in @patterns. As expected we see very diffuse scattering at higher temperatures, while the diffraction patterns become more ordered at lower temperatures. The most surprising diffraction pattern was a)

If further work on the simulation was to be made measurements of more different characteristic of the system might show more phase transition which cannot be interpreted from only the measurement of the energy of the system.

Furthermore the simulation could be generalized to include more distant neighbor interactions to create a better approximation of the real crystals.



#bibliography("sources.bib")

#pagebreak()

= Appendix

== Considerations about the parameter space

Lets consider the Boltzmann factor of a system where the hamiltonian $cal(H)$ can be described by the sum of two energies multiplied by two functions $s_1$ and $s_2$ on the state of the system $sigma$.
$ cal(H) (sigma) = J_1 s_1(sigma) + J_2 s_2(sigma) $
Lets define two new variables $J'$ and $T'$ such that $J_2 J' = J_1$ and $J_2 T'= k_b T$.
$ exp( - (cal(H)(sigma))/(k_b T)) 
    &= exp( - (J_1 s_1(sigma) + J_2 s_2(sigma))/(k_b T)) \
    &= exp( - (cancel(J_2)J' s_1(sigma) + cancel(J_2) s_2(sigma))/(cancel(J_2)T')) \
    &= exp( - (J' s_1(sigma) + s_2(sigma))/(T')) $<parameters>
Thus we can describe the whole parameter space using only $T'$ and $J'$.