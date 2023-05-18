use std::fs;
use halo2_proofs::SerdeFormat;
use types::eth::BlockTrace;
use zkevm::circuit::AGG_DEGREE;
use zkevm::utils::{load_params, load_seed};
use zkevm::{circuit::DEGREE, prover::Prover};

fn main() {
    let traces_vec = fs::read("./multiple.json").unwrap();
    let trace = serde_json::from_slice::<BlockTrace>(&traces_vec).unwrap();

    let params = load_params("./test_params", *DEGREE, SerdeFormat::RawBytesUnchecked).unwrap();
    let agg_params = load_params("./test_params", *AGG_DEGREE, SerdeFormat::RawBytesUnchecked).unwrap();
    let seed = load_seed("./test_seed").unwrap();
    let mut p = Prover::from_params_and_seed(params, agg_params, seed);
    let _ = p
        .create_agg_circuit_proof_batch(&[trace])
        .unwrap();
}
