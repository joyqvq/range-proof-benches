#[macro_use]
extern crate criterion;

mod bulletproof_benches {

    use bulletproof::bulletproofs::BulletproofsRangeProof;
    use criterion::Criterion;
    use rand::{thread_rng, RngCore};

    fn proof_32(c: &mut Criterion) {
        let value = 100;
        let upper_bound = 32;
        let mut blinding = [0u8; 32];
        thread_rng().fill_bytes(&mut blinding);
        c.bench_function("Bulletproof Proving 32 bits", move |b| {
            b.iter(|| {
                let (_commitment, _range_proof) = BulletproofsRangeProof::prove_bit_length(
                    value,
                    blinding,
                    upper_bound,
                    b"MY_DOMAIN",
                )
                .unwrap();
            })
        });
    }

    fn verify_32(c: &mut Criterion) {
        let value = 100;
        let upper_bound = 32;
        let mut blinding = [0u8; 32];
        thread_rng().fill_bytes(&mut blinding);
        let (commitment, range_proof) =
            BulletproofsRangeProof::prove_bit_length(value, blinding, upper_bound, b"MY_DOMAIN")
                .unwrap();

        c.bench_function("Bulletproof Verifying 32 bits", move |b| {
            b.iter(|| {
                range_proof
                    .verify_bit_length(&commitment, upper_bound, b"MY_DOMAIN")
                    .is_ok()
            })
        });
    }

    fn proof_64(c: &mut Criterion) {
        let value = 100;
        let upper_bound = 64;
        let mut blinding = [0u8; 32];
        thread_rng().fill_bytes(&mut blinding);
        c.bench_function("Bulletproof Proving 32 bits", move |b| {
            b.iter(|| {
                let (_commitment, _range_proof) = BulletproofsRangeProof::prove_bit_length(
                    value,
                    blinding,
                    upper_bound,
                    b"MY_DOMAIN",
                )
                .unwrap();
            })
        });
    }

    fn verify_64(c: &mut Criterion) {
        let value = 100;
        let upper_bound = 64;
        let mut blinding = [0u8; 32];
        thread_rng().fill_bytes(&mut blinding);
        let (commitment, range_proof) =
            BulletproofsRangeProof::prove_bit_length(value, blinding, upper_bound, b"MY_DOMAIN")
                .unwrap();

        c.bench_function("Bulletproof Verifying 32 bits", move |b| {
            b.iter(|| {
                range_proof
                    .verify_bit_length(&commitment, upper_bound, b"MY_DOMAIN")
                    .is_ok()
            })
        });
    }

    criterion_group! {
        name = bulletproof_benches;
        config = Criterion::default().sample_size(100);
        targets = proof_32, verify_32, proof_64, verify_64
    }
}

criterion_main!(bulletproof_benches::bulletproof_benches,);
