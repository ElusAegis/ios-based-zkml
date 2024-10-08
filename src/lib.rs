mod error;
mod gen_witness;
mod prove;
mod serialization;
mod verify;

pub use gen_witness::gen_witness;
pub use prove::{prove, prove_advanced};
pub use verify::verify;

pub(crate) use error::EZKLError as ExternalEZKLError;
pub(crate) use ezkl::EZKLError as InnerEZKLError;
pub(crate) use halo2_proofs::poly::ipa::strategy::AccumulatorStrategy as IPAAccumulatorStrategy;
pub(crate) use halo2_proofs::poly::ipa::strategy::SingleStrategy as IPASingleStrategy;
pub(crate) use halo2_proofs::poly::kzg::strategy::AccumulatorStrategy as KZGAccumulatorStrategy;
pub(crate) use halo2_proofs::poly::kzg::strategy::SingleStrategy as KZGSingleStrategy;

uniffi::setup_scaffolding!();

// This module is used for testing purposes only
pub mod testing {
    pub use crate::serialization::deserialize_params_prover;
}
