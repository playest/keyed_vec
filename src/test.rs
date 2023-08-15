#[test]
fn test_01() {
    use crate::{IndexLike, KeyedVec};

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
    assert!(clients.len() == 0);
    let client_a = clients.push(Client { name: "A".to_string() });
    assert!(clients.len() == 1);
    let client_b = clients.push(Client { name: "B".to_string() });
    assert!(clients.len() == 2);
    let client_c = clients.push(Client { name: "C".to_string() });
    assert!(clients.len() == 3);

    let mut orders = KeyedVec::<OrderId, Order>::new();
    assert!(orders.len() == 0);
    let order_1 = orders.push(Order { address: "1".to_string() });
    assert!(orders.len() == 1);
    let order_2 = orders.push(Order { address: "2".to_string() });
    assert!(orders.len() == 2);
    let order_3 = orders.push(Order { address: "3".to_string() });
    assert!(orders.len() == 3);


    assert!(clients.get(client_a).unwrap().name == "A");
    assert!(clients.get(client_b).unwrap().name == "B");
    assert!(clients.get(client_c).unwrap().name == "C");

    assert!(orders.get(order_1).unwrap().address == "1");
    assert!(orders.get(order_2).unwrap().address == "2");
    assert!(orders.get(order_3).unwrap().address == "3");
}


/// This doctest checks (see last line) that you can't index an array with indices coming from an other array: `clients.get(order_1);`
/// 
/// ```compile_fail
/// use keyed_vec::{KeyedVec, IndexLike};
/// 
/// #[derive(Debug, Clone)]
/// struct Client {
///     name: String,
/// }
/// 
/// #[derive(Debug, Clone)]
/// struct Order {
///     address: String
/// }
/// 
/// #[derive(Debug, Clone, Copy)]
/// struct ClientId(usize);
/// impl IndexLike for ClientId {
///     fn to_index(&self) -> usize {
///         self.0
///     }
/// 
///     fn from_index(i: usize) -> Self {
///         ClientId(i)
///     }
/// }
/// 
/// #[derive(Debug, Clone, Copy)]
/// struct OrderId(usize);
/// impl IndexLike for OrderId {
///     fn to_index(&self) -> usize {
///         self.0
///     }
/// 
///     fn from_index(i: usize) -> Self {
///         OrderId(i)
///     }
/// }
/// 
/// let mut clients = KeyedVec::<ClientId, Client>::new();
/// let client_a = clients.push(Client { name: "A".to_string() });
/// 
/// let mut orders = KeyedVec::<OrderId, Order>::new();
/// let order_1 = orders.push(Order { address: "1".to_string() });
/// 
/// // mismatched types expected `ClientId`, found `OrderId`
/// clients.get(order_1);
/// ```
#[allow(dead_code)]
fn test_02_compile_fail() {
    // This is an empty function, it's here just for the compile_fail doctest above
}