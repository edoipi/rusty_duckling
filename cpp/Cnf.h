#ifndef _CNF
#define _CNF

struct Cnf {
    unsigned vc;    // var count
    unsigned cc;    // clause count
    int **clauses;    // 2-dim. array with entries same as in cnf file
    unsigned lc;    // literal count
    unsigned *cl;    // clause length
    Cnf(char *fname);

    ~Cnf();
};

#endif
