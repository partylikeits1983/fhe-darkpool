// testdata.rs

use crate::common::{Order, Orders};
use rand::random;

/// Returns the orders for user one when a match should be found.
pub fn user_one_orders_match() -> Orders {
    Orders {
        order: [
            // This first buy can match a sell if price=100 and side conditions hold.
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 100,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 101,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 102,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 103,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 104,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 105,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 106,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 107,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 108,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 109,
                a_for_b: true,
            },
        ]
        .to_vec(),
    }
}

/// Returns the orders for user one when no match should be found.
pub fn user_one_orders_no_match() -> Orders {
    Orders {
        order: [
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 100,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 101,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 102,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 103,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 104,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 105,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 106,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 107,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 108,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 109,
                a_for_b: true,
            },
        ]
        .to_vec(),
    }
}

/// Returns the orders for user two when a match should be found.
pub fn user_two_orders_match() -> Orders {
    Orders {
        order: [
            // 8 buy orders:
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 200,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 201,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 202,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 203,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 204,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 205,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 206,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 207,
                a_for_b: true,
            },
            // This one is SELL at price=100 => should match user_one_orders_match()[0]
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 100,
                a_for_b: false,
            },
            // Another random buy
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 209,
                a_for_b: true,
            },
        ]
        .to_vec(),
    }
}

/// Returns the orders for user two when no match should be found.
pub fn user_two_orders_no_match() -> Orders {
    Orders {
        order: [
            // 8 buy orders:
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 200,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 201,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 202,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 203,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 204,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 205,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 206,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 207,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 105,
                a_for_b: true,
            },
            Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                price: 209,
                a_for_b: true,
            },
        ]
        .to_vec(),
    }
}

/// Creates test data for orders for two parties (A and B).
///
/// # Arguments
///
/// * `number_of_orders` - The number of orders to generate for each party.
/// * `is_match` - If true, party B will include at least one order that can match party A's first order.
///              A match is defined by: price equality and opposite side (i.e. one is buy and one is sell).
///
/// # Returns
///
/// A tuple containing the orders for party A and party B respectively.
pub fn create_order_test_data(number_of_orders: u32, is_match: bool) -> (Orders, Orders) {
    let mut orders_a = Vec::with_capacity(number_of_orders as usize);
    let mut orders_b = Vec::with_capacity(number_of_orders as usize);

    // Generate orders for party A.
    // For simplicity, party A orders will be all buys with increasing prices.
    for i in 0..number_of_orders {
        orders_a.push(Order {
            id: random::<u32>(),
            asset_a: 1,
            asset_b: 2,
            // Use price 100, 101, 102, ...
            price: 100 + i,
            a_for_b: true, // buy
        });
    }

    // Generate orders for party B.
    // If is_match is true, force at least one matching order.
    if is_match {
        // For the first order in party B, create a SELL order that matches party A's first order.
        // (The match conditions here are: equal price and opposite side.)
        orders_b.push(Order {
            id: random::<u32>(),
            asset_a: 1,
            asset_b: 2,
            price: orders_a.get(0).map_or(100, |o| o.price), // match party A's price
            a_for_b: false,                                  // sell order
        });
        // Generate the remaining orders as non-matching buy orders.
        for i in 1..number_of_orders {
            orders_b.push(Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                // set prices that don't match party A's (e.g., starting at 200)
                price: 200 + i,
                a_for_b: true, // buy
            });
        }
    } else {
        // When no match is desired, generate all orders as buys with prices that do not match party A.
        for i in 0..number_of_orders {
            orders_b.push(Order {
                id: random::<u32>(),
                asset_a: 1,
                asset_b: 2,
                // Using prices starting at 200 ensures none match party A (which starts at 100).
                price: 200 + i,
                a_for_b: true, // buy
            });
        }
    }

    (Orders { order: orders_a }, Orders { order: orders_b })
}
