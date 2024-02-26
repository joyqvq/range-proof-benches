pragma circom 2.1.5;

include "../node_modules/circomlib/circuits/pedersen.circom";
include "../node_modules/circomlib/circuits/comparators.circom";

template rpPedersen(N) {
    signal input x; // N bits
    signal input r; // 128 bits

    signal output c0;
    signal output c1;
    signal input ul;

    component bitifyX = Num2Bits(N);
    bitifyX.in <== x;

    component bitifyR = Num2Bits(128);
    bitifyR.in <== r;

    component CPed = Pedersen(N + 128);
    for (var i = 0; i < N; i++) {
        CPed.in[i] <== bitifyX.out[i];
    }
    for (var i = 0; i < 128; i++) {
        CPed.in[N + i] <== bitifyR.out[i];
    }

    CPed.out[0] ==> c0;
    CPed.out[1] ==> c1;

    component CLess = LessThan(N);
    CLess.in[0] <== x;
    CLess.in[1] <== ul;
    CLess.out === 1;

    log(CLess.out);
 }

component main{public [ul]} = rpPedersen(64);

/* INPUT = {
    "x": "5",
    "r": "77",
    "ul": "6"
} */
