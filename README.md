# blockchain
Blockchain in Rust

# Project Structure

Our final project structure shall look something like this.

```
|- Cargo.toml
|- Cargo.lock
|____src
| |____server
| | |____server.rs
| | |____mod.rs
| |____blockchain
| | |____transaction.rs
| | |____chain.rs
| | |____mod.rs
| | |____block.rs
| |____sha
| | |____mod.rs
| | |____sha.rs
| |____main.rs
```

# What is a blockchain?
A blockchain is a distributed and decentralized way of storing data, such as transactions publicly shared across multiple nodes. As the name suggests, Block-Chain, in a blockchain multiple such blocks of transactions are linked(chained) together using hashes. The hash of a “Block” is dependent on the hash of the previous block in the chain.

# What is a Block?
An implementation of “Block”(block.rs) will store the transactions, a “proof of work” and a hash from the previously mined “Block”. So, each block in the blockchain will have an index, a timestamp, list of transactions, a proof, and a hash.

# What is a Chain? 
Sequence of Blocks stitched together is called a Chain. The first block of the chain is called the “Genesis Block”. 

# What is Mining ?
Mining, in the context of blockchain technology, is the process of adding transactions i.e new Blocks to the Chain after the first block i.e “Genesis Block” to further build the chain. chain.rs creates a new transaction and appends it to the list of existing transactions.

# Proof of Work (Algorithm): Simple One
Our PoW algorithm is quite simple for the sake of understanding the concept. Please note Bitcoin uses the Hashcash proof of work system. 

# Building the Blockchain
The `Blockchain` will store the `chain` and the `transactions` as well. We will be implementing methods to add transactions, mine a block, find proof of work, hash and a few others as well.   

**"Find a number p such that, when it is hashed with the previous block’s solution, a new hash with 4 leading 0s is produced."** 

 - Find a number p' such that hash(pp') contains leading 4 zeroes, where p is the previous p'
 - p is the previous proof, and p' is the new proof
 - Parameter: last_proof: u64
 - returns: u64

# What is “distributed”? 
Suppose you have five kids and they want to eat ice-cream. Instead of giving all money to the oldest one, you give 50 cents to each of them.

# What is “decentralized”? 
Now what you did above was not just sharing the money. You also shared the responsibility among them equally. This shared responsibility is the process of decentralizing. Now, trying to understand this in terms of blockchain. Each member in the system has a copy of the chain. Not everyone can lose the chain all at once. Thus, by not storing data at one central place, the blockchain eliminates several risks.

### Simple blockchain implementation in Rust programming language ###

### Requirements
* [Rust](https://www.rust-lang.org/)

### Dependencies
1. [chrono](https://crates.io/crates/chrono) Date and time library for Rust.
2. [serde](https://crates.io/crates/serde) A generic serialization/deserialization framework.
3. [serde_json](https://crates.io/crates/serde_json)  A JSON serialization file format.
4. [rocket](https://crates.io/crates/rocket) Web framework for nightly with a focus on ease-of-use, expressibility, and speed. It is used for handling the incoming HTTP requests
5. [reqwest](https://crates.io/crates/reqwest) higher level HTTP client library. It is used for for making HTTP requests
6. [rocket_contrib](https://crates.io/crates/rocket_contrib) Community contributed libraries for the Rocket web framework. 

### Run the Project
You can run your project by simply executing `cargo run` from project directory. The server should now be up and running at `localhost:5000`.  

# API Endpoints
1. Create new transactions................../transactions/new
2. Mine new blocks........................../mine
3. See the blockchain......................./chain
4. Register new nodes in the network......../nodes/register
5. See existing nodes in the network......../nodes
6. Verify the valid chain.................../nodes/resolve 

Using `localhost:5000` you can see the blockchain at


# See the blockchain

```
curl -X GET \
 http://localhost:5000/chain \
```

With the response as below

```
[
    {
        "index": 1,
        "previous_hash": "1",
        "proof": 100,
        "timestamp": "2021-03-09T07:22:39.503836Z",
        "transactions": []
    }
]
```

# Add a new Transaction
```
curl -X POST \
  http://localhost:5000/transactions/new \
  -H 'content-type: application/json' \
  -d '{
 "sender": "d4ee26eee15148ee92c6cd394edd974e",
 "recipient": "4as226eee1514743e92c6cd394edd974f",
 "amount": 5
}'
```
With the response as below

```
{
    "message": "Transaction added to the block",
    "transaction": {
        "amount": 5,
        "recipient": "4as226eee1514743e92c6cd394edd974f",
        "sender": "d4ee26eee15148ee92c6cd394edd974e"
    }
}
```
# Mine a block
```
curl -X GET \
  http://localhost:5000/mine \
```

With the response as below

```
{
    "message": "New block forged",
    "proof": 33575,
    "previous_hash": "0201d60f8d9227e99b52465254dceaf3487d5f829e9a4c72633b327684aaa308",
    "transactions": [
        {
            "amount": 5,
            "recipient": "4as226eee1514743e92c6cd394edd974f",
            "sender": "d4ee26eee15148ee92c6cd394edd974e"
        },
        {
            "amount": 1,
            "recipient": "792DC096423E4D289CE73510A66751872631800001087BEA56B12",
            "sender": "0"
        }
    ],
    "index": 2
}
```

# Verify the Blockchain

```
curl -X GET \
  http://localhost:5000/nodes/resolve \
```

With the response as below

```
{
    "chain": [
        {
            "index": 1,
            "previous_hash": "1",
            "proof": 100,
            "timestamp": "2021-03-09T07:22:39.503836Z",
            "transactions": []
        },
        {
            "index": 2,
            "previous_hash": "8e3cccb856e49ec262b822e5200786a6af48393799f30e7735b62c906e71528d",
            "proof": 33575,
            "timestamp": "2021-03-09T07:24:11.504608Z",
            "transactions": [
                {
                    "amount": 5,
                    "recipient": "4as226eee1514743e92c6cd394edd974f",
                    "sender": "d4ee26eee15148ee92c6cd394edd974e"
                }
            ]
        }
    ],
    "message": "Our chain is authoritative"
}
```

# Register a new Node in the Network

```
curl -X POST \
  http://localhost:5000/nodes/register \
  -H 'content-type: application/json' \
  -d '{
 "nodes": ["http://127.0.0.104:5000"]
}'
```
With the response as below

```
{
    "message": "New nodes have been added",
    "total_nodes": [
        "127.0.0.104"
    ]
}
```