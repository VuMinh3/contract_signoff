# contract_signoff

## Project Title
contract_signoff

## Project Description
`contract_signoff` is a Soroban smart contract that turns multi-party legal
agreements into a transparent, on-chain signoff registry. A drafter
publishes a contract bound to a 32-byte content hash and declares how many
distinct signatures are required. Each invited counterparty can sign,
withdraw, and watch signoff progress in real time. The drafter finalizes
once the threshold is met, or cancels the contract with a reason if terms
change. The on-chain state machine makes "who signed what, when" auditable
by anyone, while `require_auth` ensures that only the legitimate parties
can act on a given contract.

## Project Vision
To give small businesses, DAOs, and freelance collectives a no-trust
signoff log for legal documents — without depending on a centralized
e-signature provider. The vision is a world where every agreement
between counterparties leaves a verifiable, censorship-resistant trail
on Stellar, removing the privacy trade-offs and lock-in of closed
SaaS tools. Future iterations will compose this primitive with escrow
and milestone-based release contracts to power full on-chain legal
workflows.

## Key Features
- **Drafter-controlled lifecycle** — only the address that created the
  contract can finalize or cancel it, and only while the contract is
  still in the Pending state.
- **Multi-party signoff** — N-of-M threshold; the contract moves to
  `Finalized` only when the configured number of distinct signers have
  signed.
- **Withdrawable signatures** — signers can pull back their signature
  before finalization, so an unsigned draft is never silently locked in.
- **On-chain audit trail** — every state transition (created, signed,
  withdrawn, finalized, cancelled) is persisted in contract storage and
  can be queried by frontends or off-chain indexers.
- **Hash-bound content** — the contract binds a `BytesN<32>` hash so the
  off-chain PDF or markdown being signed is provably the one agreed upon.
- **Pure registry** — no XLM or other asset is moved, keeping fees
  minimal and the contract composable with other on-chain primitives.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** legal dApp — see `contracts/contract_signoff/src/lib.rs` for the full contract_signoff business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `<to be deployed on Stellar Testnet>`
- **Explorer template:** `https://stellar.expert/explorer/testnet`
- **Screenshot of deployed contract on Stellar Expert:**
  `_(Screenshot of the contract page on Stellar Expert will appear here after deploy.)_`


## Future Scope
- **Weighted multi-sig signoff** — let each signer carry a configurable
  weight and finalize when the cumulative weight threshold is reached.
- **Expiry windows** — contracts auto-cancel via ledger-time deadlines
  if the required signatures are not collected in time.
- **Off-chain hash anchoring** — periodically publish the contract hash
  to a public bulletin board / oracle for additional immutability.
- **Dispute / arbitration hook** — allow a designated arbiter address to
  force a transition into a Disputed state, paving the way for on-chain
  mediation.
- **Frontend dApp** — a Freighter + React UI that lets drafters upload
  a PDF, computes the hash, and walks invited signers through the
  sign / withdraw flow.
- **Audit log indexer** — an off-chain indexer that turns contract
  storage events into a human-readable signature timeline for auditors.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `contract_signoff` (legal)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
