#![no_std]

//! # contract_signoff
//!
//! A Soroban smart contract that implements a multi-party contract signoff
//! registry. A drafter publishes a contract bound to a 32-byte content hash
//! and declares how many distinct signatures are required. Designated
//! counterparties sign the contract, can withdraw their signature before
//! finalization, and the drafter finalizes (or cancels with a reason) once
//! the threshold is met.
//!
//! The contract is intentionally a pure registry: it never moves XLM or any
//! other token. All authorization is enforced through `require_auth`.

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, Symbol};

/// Lifecycle status of a contract signoff entry.
const STATUS_PENDING: u32 = 0;
const STATUS_FINALIZED: u32 = 1;
const STATUS_CANCELLED: u32 = 2;

/// Persisted state of a single signoff request.
#[contracttype]
#[derive(Clone)]
pub struct ContractData {
    /// The address that registered the contract; the only address allowed to
    /// finalize or cancel it.
    pub drafter: Address,
    /// SHA-256 (or any 32-byte) hash of the off-chain contract content.
    pub contract_hash: BytesN<32>,
    /// Number of distinct signatures required before the contract can be
    /// finalized.
    pub required_signers: u32,
    /// Number of distinct signatures currently collected.
    pub signature_count: u32,
    /// Current status (pending / finalized / cancelled).
    pub status: u32,
    /// Short reason symbol recorded when the drafter cancels the contract.
    pub reason: Symbol,
}

/// Storage keys used by the contract.
#[contracttype]
pub enum DataKey {
    /// Per-contract metadata.
    Contract(BytesN<32>),
    /// Per-(contract, signer) marker — present iff `signer` has signed.
    Signer(BytesN<32>, Address),
}

#[contract]
pub struct ContractSignoff;

#[contractimpl]
impl ContractSignoff {
    /// Create a new contract signoff request. The drafter's authorization is
    /// verified via `require_auth`. The contract is bound to a 32-byte
    /// content hash and a positive threshold of required signatures.
    /// Re-creating an existing `contract_id` is rejected.
    pub fn create_contract(
        env: Env,
        drafter: Address,
        contract_id: BytesN<32>,
        contract_hash: BytesN<32>,
        required_signers: u32,
    ) {
        drafter.require_auth();

        let key = DataKey::Contract(contract_id);
        if env.storage().instance().has(&key) {
            panic!("contract already exists");
        }
        if required_signers == 0 {
            panic!("must require at least 1 signer");
        }

        let data = ContractData {
            drafter: drafter.clone(),
            contract_hash,
            required_signers,
            signature_count: 0,
            status: STATUS_PENDING,
            reason: Symbol::new(&env, ""),
        };

        env.storage().instance().set(&key, &data);
    }

    /// Add the caller's signature to a pending contract. The signer's
    /// authorization is verified via `require_auth`. A signer cannot sign
    /// the same contract twice, and signing is only allowed while the
    /// contract is still in the Pending state. Returns the new total
    /// signature count.
    pub fn sign(env: Env, signer: Address, contract_id: BytesN<32>) -> u32 {
        signer.require_auth();

        let key = DataKey::Contract(contract_id.clone());
        let mut data: ContractData = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("contract not found"));

        if data.status != STATUS_PENDING {
            panic!("contract is not pending");
        }

        let signer_key = DataKey::Signer(contract_id, signer);
        if env.storage().instance().has(&signer_key) {
            panic!("already signed");
        }

        env.storage().instance().set(&signer_key, &true);

        data.signature_count += 1;
        env.storage().instance().set(&key, &data);

        data.signature_count
    }

    /// Withdraw a previously submitted signature. Only allowed while the
    /// contract is still in the Pending state and only by the original
    /// signer. Decrements the recorded signature count.
    pub fn withdraw_signature(env: Env, signer: Address, contract_id: BytesN<32>) {
        signer.require_auth();

        let key = DataKey::Contract(contract_id.clone());
        let mut data: ContractData = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("contract not found"));

        if data.status != STATUS_PENDING {
            panic!("contract is not pending");
        }

        let signer_key = DataKey::Signer(contract_id, signer);
        if !env.storage().instance().has(&signer_key) {
            panic!("not signed");
        }

        env.storage().instance().remove(&signer_key);
        data.signature_count -= 1;
        env.storage().instance().set(&key, &data);
    }

    /// Finalize the contract. Only the original drafter can finalize, and
    /// only after the required number of distinct signatures has been
    /// collected. Once finalized, no further state transitions are allowed.
    pub fn finalize(env: Env, drafter: Address, contract_id: BytesN<32>) {
        drafter.require_auth();

        let key = DataKey::Contract(contract_id);
        let mut data: ContractData = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("contract not found"));

        if data.drafter != drafter {
            panic!("not the drafter");
        }
        if data.status != STATUS_PENDING {
            panic!("contract is not pending");
        }
        if data.signature_count < data.required_signers {
            panic!("not enough signatures");
        }

        data.status = STATUS_FINALIZED;
        env.storage().instance().set(&key, &data);
    }

    /// Cancel a pending contract. Only the drafter can cancel, and only
    /// while the contract is still in the Pending state. A short `reason`
    /// symbol is recorded on-chain for downstream readers and auditors.
    pub fn cancel(env: Env, drafter: Address, contract_id: BytesN<32>, reason: Symbol) {
        drafter.require_auth();

        let key = DataKey::Contract(contract_id);
        let mut data: ContractData = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("contract not found"));

        if data.drafter != drafter {
            panic!("not the drafter");
        }
        if data.status != STATUS_PENDING {
            panic!("contract is not pending");
        }

        data.status = STATUS_CANCELLED;
        data.reason = reason;
        env.storage().instance().set(&key, &data);
    }

    /// View the current signature count for a contract. Useful for
    /// frontends and off-chain watchers that need to render real-time
    /// signoff progress without re-reading the full state.
    pub fn get_signature_count(env: Env, contract_id: BytesN<32>) -> u32 {
        let data: ContractData = env
            .storage()
            .instance()
            .get(&DataKey::Contract(contract_id))
            .unwrap_or_else(|| panic!("contract not found"));
        data.signature_count
    }
}
