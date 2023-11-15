#import "@preview/whalogen:0.1.0": ce
= Abstract


= Introduction

== Prussian Blue Analogues
Prussian blue analogues are a class of materials derived from prussian blue. Their composition can be described as #ce("M^II [M^III (CN)_6]_(x)")


== Monte Carlo Method
In order to get a macroscopic view of a system, of which we know all possible states and the hamiltonian of each state $cal(H)(sigma)$, we make the following observation. For this report we start from the observation that the probability of a state occurring $rho (sigma)$ at some temperature $T$ is proportional to $exp(- (cal(H) (sigma))/(k_b T))$. // find reference for this
Additional we know the probabilities sum to 1.
$ 1 = sum_(sigma in Omega) (exp(-(cal(H) (sigma))/(k_b T)))/Z = 1/Z sum_(sigma in Omega) exp(- (cal(H)(sigma))/(k_b T)) $

$ => Z = sum_(sigma in Omega) exp(- (cal(H)(sigma))/(k_b T)) $

The Hamiltonian of the system is divided into 2 terms. Only interactions between cyanometalates and vacancies are considered. For the nearest neighbor with an offset of $1/2$< 110 > if both of them are the same (vacancies or metalates) their contribution to the total energy is $J_1$ otherwise it is $-J_1$. Similarly for next nearest neighbors along < 100 > directions, if both are the same their contribution is $J_2$ and $-J_2$ otherwise.

= Methods

== Simulation
The structure is modeled as a three dimensional square grid containing $N$ unit cells. To represent the fcc structure of the PBA, a grid of size $2N times 2N times 2N$ is created and all positions with $i+j+k eq.triple 0 (mod 2)$ are populated with $0$ representing the fixed M sites. The other positions are populated with -1 or 1 representing a vacancy or a cyanometalate respectively. To achieve the desired fill fraction a random number generator is used to generate boolean with the desired distribution. Note that this creates a three dimensional version of a checkerboard pattern where each direct neighbor to an -1/1 site is a 0 site. Additionally the grid is accessed in a modular fashion such that $e_(i,j,k) = e_(i+2N, j, k) = e_(i, j+2N, k) = e_(i, j, k+2N)$.\
Instead of calculating the Hamiltonian as described in the introduction the sums over the nearest neighbors and next nearest neighbors are calculated separately. This allows for the use of integer arithmetic, thus omitting all floating point in accuracies.\
For each Monte Carlo step two of the vacancy/cyanometalate sites, which aren't the same are chosen uniformly from the whole grid. To calculate the difference in energy, if a swap was to be performed is calculated by calculating how the sums over nearest neighbors and next nearest neighbors. If this difference in energy is smaller than zero the swap is accepted, otherwise the swap is allowed with a probability $p=exp(-(Delta E)/(k_b T))$.\
As by the consideration in the appendix only two new parameters $J' = J_1/J_2$ and $T' = (k_b T)/J_2$ need to be explored to get an overview over the whole parameter space.
For the simulation an ensemble of 20 models with different J' terms is simulated. Each model is tempered from some initial temperature $T_0$ to $T_(n-1)$. The temperature steps are chosen such that $ln(T_i)$ is a linear function of i. At each temperature the model is left to equilibrate for 500 epochs before any measurements are taken from another 500 epochs. Where an epoch refers to $8 S$ steps with S the number of vacancy/cyanometalate sites.
At the end of each temperature step a `.cif` file is generated from the model. All values from the measurements are logged to a `.csv` file.

=== Optimizations
For the compiler to be able to fully optimize the program, where possible the size of list of values, like the grid is give at compile time through the use of `[i8; N]` instead of `Vec<i8>`

== Diffraction
On the `.cif` files produced by the command line tool `gemmi sfcalc` is run. This program calculates the electron density in the structure and calculated the structure factors into a `.sf` file.


= Results

= Discussion

= Appendix

== Considerations about the parameter space

From statistical mechanics we know that the partition function $Z$ describes the system completely.
Lets consider a system where the hamiltonian $cal(H)$ can be described by the sum of two energies multiplied by the functions $c_1$ and $c_2$ on the state of the system $sigma$.
$ cal(H) (sigma) = J_1 c_1(sigma) + J_2 c_2(sigma) $
Lets define two new variables $J'$ and $T'$ such that $J_2 J' = J_1$ and $J_2 T'= k_b T$.
$ Z &= sum_(sigma) exp( - (cal(H)(sigma))/(k_b T)) \
    &= sum_(sigma) exp( - (J_1 c_1(sigma) + J_2 c_2(sigma))/(k_b T)) \
    &= sum_(sigma) exp( - (cancel(J_2)J' c_1(sigma) + cancel(J_2) c_2(sigma))/(cancel(J_2)T')) \
    &= sum_(sigma) exp( - (J' c_1(sigma) + c_2(sigma))/(T')) $
Thus we see that the system only depends on two parameters.