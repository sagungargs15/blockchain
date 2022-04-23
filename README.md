# blockchain
Blockchain in Rust

# What is a blockchain?
A blockchain is a distributed and decentralized way of storing data, such as transactions publicly shared across multiple nodes. As the name suggests, Block-Chain, in a blockchain multiple such blocks of transactions are linked(chained) together using hashes. The hash of a “Block” is dependent on the hash of the previous block in the chain.

# What is a Block?
An implementation of “Block”(block.rs) will store the transactions, a “proof of work” and a hash from the previously mined “Block”. So, each block in the blockchain will have an index, a timestamp, list of transactions, a proof, and a hash.

# What is a Chain & What is Mining ?
Sequence of Blocks stitched together is called a Chain. The first block of the chain is called the “Genesis Block”. Mining, in the context of blockchain technology, is the process of adding transactions i.e new Blocks to the Chain after the first block i.e “Genesis Block” to further build the chain. chain.rs creates a new transaction and appends it to the list of existing transactions

# What is “distributed”? 
Suppose you have five kids and they want to eat ice-cream. Instead of giving all money to the oldest one, you give 50 cents to each of them.

# What is “decentralized”? 
Now what you did above was not just sharing the money. You also shared the responsibility among them equally. This shared responsibility is the process of decentralizing. Now, trying to understand this in terms of blockchain. Each member in the system has a copy of the chain. Not everyone can lose the chain all at once. Thus, by not storing data at one central place, the blockchain eliminates several risks.
