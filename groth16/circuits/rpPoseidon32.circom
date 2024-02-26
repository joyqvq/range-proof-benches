pragma circom 2.1.5;

include "../node_modules/circomlib/circuits/poseidon.circom";
include "../node_modules/circomlib/circuits/comparators.circom";

template rpPoseidon(N) {
    signal input x;
    signal input r;

    signal output c;
    signal input ul;

    component CPos = Poseidon(2);
    CPos.inputs[0] <== x;
    CPos.inputs[1] <== r;
    CPos.out ==> c;

    component rangeCheck = Num2Bits(N);
    rangeCheck.in <== x;

    component CLess = LessThan(N);
    CLess.in[0] <== x;
    CLess.in[1] <== ul;
    CLess.out === 1;
    log(CLess.out);
 }

component main{public [ul]} = rpPoseidon(32);

/* INPUT = {
    "x": "5",
    "r": "77",
    "ul": "7"
} */
