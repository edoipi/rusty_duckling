#ifndef _SAT_SOLVER
#define _SAT_SOLVER

#include "CnfManager.h"

struct Luby {            // restart scheduler as proposed in
    vector<unsigned> seq;    // Optimal Speedup of Las Vegas Algorithms
    unsigned index;    // Michael Luby et al, 1993
    unsigned k;

    Luby() : index(0), k(1) { }

    unsigned next() {
        if (++index == (unsigned) ((1 << k) - 1))
            seq.push_back(1 << (k++ - 1));
        else
            seq.push_back(seq[index - (1 << (k - 1))]);
        return seq.back();
    }
};

class SatSolver : public CnfManager {
    unsigned nVars;        // num of variables in varOrder
    Luby luby;        // restart scheduler
    unsigned lubyUnit;    // unit run length for Luby's
    unsigned nextDecay;    // next score decay point
    unsigned nextRestart;    // next restart point

    int selectLiteral();

    bool verifySolution();

public:
    SatSolver() { };

    SatSolver(Cnf &cnf);

    bool run();

    void printStats();

    void printSolution(FILE *);
};

#endif
