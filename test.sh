#!/bin/bash

rm solution
make

printf "Should be SAT: " &&  ./solution < test/hanoi4.cnf
printf "Should be SAT: " && ./solution < test/hanoi5.cnf
printf "Should be SAT: " && ./solution < test/uf250-08.cnf
printf "Should be UNSAT: " && ./solution < test/hole6.cnf
printf "Should be UNSAT: " && ./solution < test/hole7.cnf 
printf "Should be UNSAT: " && ./solution < test/uuf200-013.cnf
