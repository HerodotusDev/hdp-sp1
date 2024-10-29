# Compliance

We divide 5 levels of sanction level.
initially, in level 5 there will be few listed address (e.g tornado cash etc).

Whenever Account X is related to Y that is in sanction level N, it will be accumulated in level N-1. (note that level 1 is exception.)

We will use SMT(Sparse Merkle Tree) to handle this accumulation and inclusion checking + update and also commit to onchain so that we could ensure the integrity.

- Key-Only Storage: SMTs efficiently store the presence of keys without associated values.
- Single Root Hash: Represents the entire state, suitable for on-chain commitment.
- Efficient Operations: Optimized for inclusion checks, insertions, and deletions.
- Scalability: Handles large and dynamic datasets efficiently


- Initiate: initiate SMT via most sanctioned addresses.
  - (key, value) schema will be (address, level)
  - `SMT::new(vec![(addr1, 5)..(addr10, 5)])`
- Each block: This checks should happen every new block committed via execution hook
  - We can translate transactions as list of (send, receiv) tuples which sometimes address is null.
  - First we generate hash map `HashMap<unique add, (relevant addresses)>`, which relevatn address is relevant via one trnasaction either by as sender or receiver.
  - loop unique addresses via hash map
    => tree.get = [undefined/int], and if value is defined (level), `tree.set(relevant addresses, level-1)` in tree, if level is lowest ignore.
- commit : SMT.root
  - this is part to ensure integrity. (???)
