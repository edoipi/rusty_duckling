#!/bin/bash

rm solution
make
./solution < test/hanoi4.cnf
./solution < test/hanoi5.cnf
./solution < test/hole6.cnf
./solution < test/hole7.cnf
./solution < test/uf250-08.cnf
./solution < test/uuf200-013.cnf
