A Vector type where the key is a custom object to avoid mixing indices between vectors.

# Example

```rust
use keyed_vec::{KeyedVec, IndexLike};

#[derive(Debug, Clone)]
struct Client {
    name: String,
}

#[derive(Debug, Clone)]
struct Order {
    address: String
}

#[derive(Debug, Clone, Copy)]
struct ClientId(usize);
impl IndexLike for ClientId {
    fn to_index(&self) -> usize {
        self.0
    }

    fn from_index(i: usize) -> Self {
        ClientId(i)
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderId(usize);
impl IndexLike for OrderId {
    fn to_index(&self) -> usize {
        self.0
    }

    fn from_index(i: usize) -> Self {
        OrderId(i)
    }
}

let mut clients = KeyedVec::<ClientId, Client>::new();
let client_a = clients.push(Client { name: "A".to_string() });

let mut orders = KeyedVec::<OrderId, Order>::new();
let order_1 = orders.push(Order { address: "1".to_string() });

// mismatched types expected `ClientId`, found `OrderId`
clients.get(order_1);
```