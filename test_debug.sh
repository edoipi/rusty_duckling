#!/bin/bash

rm solution
cargo build

printf "Should be SAT: " && ./target/debug/rusty_duckling.exe < test/hanoi4.cnf
printf "Should be SAT: " && ./target/debug/rusty_duckling.exe < test/hanoi5.cnf
printf "Should be SAT: " && ./target/debug/rusty_duckling.exe < test/uf250-08.cnf
printf "Should be UNSAT: " && ./target/debug/rusty_duckling.exe < test/hole6.cnf
printf "Should be UNSAT: " && ./target/debug/rusty_duckling.exe < test/hole7.cnf 
printf "Should be UNSAT: " && ./target/debug/rusty_duckling.exe < test/uuf200-013.cnf
