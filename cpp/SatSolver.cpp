#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <algorithm>
#include <functional>
#include "SatSolver.h"

#define HALFLIFE 128
#define _DT 32        // RSAT phase selection threshold

SatSolver::SatSolver(Cnf &cnf) : CnfManager(cnf) {
    // initialize parameters
    nextRestart = luby.next() * (lubyUnit = 512);
    nextDecay = HALFLIFE;

    // assertUnitClauses has failed
    if (dLevel == 0) return;

    // assert pure literals
    for (int i = 1; i <= (int) vc; i++)
        if (vars[i].value == _FREE) {
            if (vars[i].activity[_POSI] == 0 && vars[i].activity[_NEGA] > 0)
                // ante is NULL, as opposed to empty clause for implied literals
                assertLiteral(-i, NULL);
            else if (vars[i].activity[_NEGA] == 0 && vars[i].activity[_POSI] > 0)
                assertLiteral(i, NULL);
        }

    // initialize varOrder
    nVars = 0;
    for (unsigned i = 1; i <= vc; i++)
        if (vars[i].value == _FREE && SCORE(i) > 0) {
            varOrder[nVars++] = i;
            vars[i].phase = (vars[i].activity[_POSI] > vars[i].activity[_NEGA]) ? _POSI : _NEGA;
        }
    sort(varOrder, varOrder + nVars, [&](unsigned a, unsigned b) -> bool { return SCORE(a) > SCORE(b); });
    for (unsigned i = 0; i < nVars; i++) varPosition[varOrder[i]] = i;
    nextVar = 0;
    printf("next clause: %d\n", clauses.size() - 1);
    nextClause = clauses.size() - 1;
}

int SatSolver::selectLiteral() {
    unsigned x = 0;

    // pick best var in unsatisfied conflict clause nearest to top of stack
    // but only search 256 clauses
    int lastClause = nextClause > 256 ? (nextClause - 256) : 0;
    for (int i = nextClause; i >= lastClause; i--) {
        int *p = clauses[nextClause = i];

        // skip satisfied clauses
        bool sat = false;
        for (; (*p); p++)
            if (SET(*p)) {
                sat = true;
                break;
            }
        if (sat) continue;

        // traverse again, find best variable of clause
        int score = -1;
        for (p = clauses[i]; (*p); p++)
            if (FREE(*p) && ((int) SCORE(VAR(*p))) > score) {
                x = VAR(*p);
                score = SCORE(x);
            }

        // RSAT phase selection
        int d = vars[x].activity[_POSI] - vars[x].activity[_NEGA];
        if (d > _DT) return x; else if (-d > _DT) return -x;
        else return (vars[x].phase == _POSI) ? (x) : -(int) (x);
    }

    // fall back to VSIDS
    for (unsigned i = nextVar; i < nVars; i++) {
        if (vars[varOrder[i]].value == _FREE) {
            x = varOrder[i];
            nextVar = i + 1;

            // RSAT phase selection
            int d = vars[x].activity[_POSI] - vars[x].activity[_NEGA];
            if (d > _DT) return x; else if (-d > _DT) return -x;
            else return (vars[x].phase == _POSI) ? (x) : -(int) (x);
        }
    }
    return 0;
}

bool SatSolver::run() {
    if (dLevel == 0) return false;        // assertUnitClauses has failed
    for (int lit; (lit = selectLiteral());) { // pick decision literal
        if (!decide(lit))
            do {        // decision/conflict
                // conflict has occurred in dLevel 1, unsat
                if (aLevel == 0) return false;

                // score decay
                if (nConflicts == nextDecay) {
                    nextDecay += HALFLIFE;
                    scoreDecay();
                }

                // rewind to top of clause stack
                nextClause = clauses.size() - 1;

                // restart at dLevel 1
                if (nConflicts == nextRestart) {
                    nRestarts++;
                    nextRestart += luby.next() * lubyUnit;
                    backtrack(1);
                    if (dLevel != aLevel) break;

                    // partial restart at aLevel
                } else backtrack(aLevel);
            } while (!assertCL());        // assert conflict literal
    }
    if (!verifySolution()) {
        printf("s UNKOWN\n");
        exit(0);
    }
    return true;
}

bool SatSolver::verifySolution() {
    int lit, *pool = litPools[0];
    for (unsigned i = 0; i < litPoolSizeOrig;) {
        bool satisfied = false;
        while ((lit = pool[i++]))
            if (SET(lit)) {
                satisfied = true;
                while (pool[i++]);
                break;
            }
        if (!satisfied) return false;
    }
    return true;
}

void SatSolver::printSolution(FILE *ofp) {
    for (unsigned i = 1; i <= vc; i++)
        if (vars[i].value == _POSI) fprintf(ofp, "%d ", i);
        else if (vars[i].value == _NEGA) fprintf(ofp, "-%d ", i);
    fprintf(ofp, "0\n");
}

void SatSolver::printStats() {
    printf("c %d decisions, %d conflicts, %d restarts\n", nDecisions, nConflicts, nRestarts);
}
