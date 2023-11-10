= Introduction

== Prussian Blue Analogues

== Monte Carlo Method
In order to get a macroscopic view of a system, of which we know all possible states and the hamiltonian of each state $cal(H)(sigma)$, we make the following observation. For this report we start from the observation that the probability of a state occuring $rho (sigma)$ at some temperature $T$ is proportional to $exp(- (cal(H) (sigma))/(k_b T))$ // find reference for this
Additional we know that the sum of all probabilities must be 1.
$ 1 = sum_(sigma in Omega) (exp(- (cal(H) (sigma))/(k_b T)))/Z = 1/Z sum_(sigma in Omega) exp(- (cal(H)(sigma))/(k_b T)) $

$ => Z = sum_(sigma in Omega) exp(- (cal(H)(sigma))/(k_b T)) $


= Methods



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