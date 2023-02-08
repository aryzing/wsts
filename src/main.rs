use rand_core::OsRng;
use std::{env, time};

use frost::{common::test_helpers::gen_signer_ids, v1, v2};

#[allow(non_snake_case)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let N: usize = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        20
    };
    let T: usize = if args.len() > 2 {
        args[2].parse::<usize>().unwrap()
    } else {
        (N * 2) / 3
    };
    let K: usize = if args.len() > 3 {
        args[3].parse::<usize>().unwrap()
    } else {
        4
    };

    let mut rng = OsRng::default();
    let msg = "It was many and many a year ago".as_bytes();

    println!("With N={N} T={T} K={K}:");

    // v1
    {
        let signer_ids = gen_signer_ids(N, K);
        let mut signers: Vec<v1::Signer> = signer_ids
            .iter()
            .map(|ids| v1::Signer::new(ids, N, T, &mut rng))
            .collect();

        let dkg_start = time::Instant::now();
        let A = v1::test_helpers::dkg(&mut signers, &mut rng).expect("v1 dkg failed");
        let dkg_time = dkg_start.elapsed();
        let mut signers: Vec<v1::Signer> = (0..(K * 3 / 4)).map(|i| signers[i].clone()).collect();

        let mut aggregator = v1::SignatureAggregator::new(N, T, A).expect("aggregator ctor failed");

        let party_sign_start = time::Instant::now();
        let (nonces, sig_shares) = v1::test_helpers::sign(msg, &mut signers, &mut rng);
        let party_sign_time = party_sign_start.elapsed();

        let group_sign_start = time::Instant::now();
        let _sig = aggregator
            .sign(msg, &nonces, &sig_shares)
            .expect("v1 group sign failed");
        let group_sign_time = group_sign_start.elapsed();

        println!("v1 dkg time {}ms", dkg_time.as_millis());
        println!(
            "v1 party sign time {}ms ({}ms/party)",
            party_sign_time.as_millis(),
            party_sign_time.as_millis() / (N as u128)
        );
        println!("v1 group sign time {}ms", group_sign_time.as_millis());
    }

    // v2
    {
        let signer_ids = gen_signer_ids(N, K);
        let mut signers: Vec<v2::Party> = signer_ids
            .iter()
            .enumerate()
            .map(|(pid, pkids)| v2::Party::new(pid, pkids, K, N, T, &mut rng))
            .collect();

        let dkg_start = time::Instant::now();
        let A = v2::test_helpers::dkg(&mut signers, &mut rng).expect("v2 dkg failed");
        let dkg_time = dkg_start.elapsed();
        let mut signers: Vec<v2::Party> = (0..(K * 3 / 4)).map(|i| signers[i].clone()).collect();

        let mut aggregator = v2::SignatureAggregator::new(N, T, A).expect("aggregator ctor failed");

        let party_sign_start = time::Instant::now();
        let (nonces, sig_shares, key_ids) = v2::test_helpers::sign(msg, &mut signers, &mut rng);
        let party_sign_time = party_sign_start.elapsed();

        let group_sign_start = time::Instant::now();
        let _sig = aggregator
            .sign(msg, &nonces, &sig_shares, &key_ids)
            .expect("v2 group sign failed");
        let group_sign_time = group_sign_start.elapsed();

        println!("v2 dkg time {}ms", dkg_time.as_millis());
        println!(
            "v2 party sign time {}ms ({}ms/party)",
            party_sign_time.as_millis(),
            party_sign_time.as_millis() / (K as u128)
        );
        println!("v2 group sign time {}ms", group_sign_time.as_millis());
    }
}
