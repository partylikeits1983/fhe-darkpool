use std::time::{Duration, Instant};

use tfhe::prelude::*;
use tfhe::{ConfigBuilder, FheBool, FheUint32, ServerKey, generate_keys, set_server_key};

use fhe_darkpool_poc::common::{safe_deserialize_item, safe_serialize_item};
use fhe_darkpool_poc::test_data::{
    create_order_test_data, user_one_orders_match, user_one_orders_no_match, user_two_orders_match,
    user_two_orders_no_match,
};

/// This test sets up 10 orders for each user that CANNOT match.
#[tokio::test]
async fn test_no_match() -> Result<(), Box<dyn std::error::Error>> {
    //
    // 1) Define the "no-match" orders for each user:
    //    - user_one: all "buy" orders for the same asset pair, same price.
    //    - user_two: also all "buy" orders => never a buy/sell opposite side => no match possible.
    //
    let user_one_orders = user_one_orders_no_match();
    let user_two_orders = user_two_orders_no_match();

    //
    // 2) user_one: Generate keys, encrypt user_one's 10 orders.
    //
    let config_one = ConfigBuilder::default().build();
    let (client_key_one, server_key_one) = generate_keys(config_one);

    let mut enc_asset_a_one: Vec<FheUint32> = Vec::new();
    let mut enc_asset_b_one: Vec<FheUint32> = Vec::new();
    let mut enc_price_one: Vec<FheUint32> = Vec::new();
    let mut enc_side_one: Vec<FheBool> = Vec::new();

    for order in &user_one_orders.order {
        enc_asset_a_one.push(FheUint32::try_encrypt(order.asset_a, &client_key_one)?);
        enc_asset_b_one.push(FheUint32::try_encrypt(order.asset_b, &client_key_one)?);
        enc_price_one.push(FheUint32::try_encrypt(order.price, &client_key_one)?);
        enc_side_one.push(FheBool::try_encrypt(order.a_for_b, &client_key_one)?);
    }

    // Serialize them individually
    let ser_enc_asset_a_one: Vec<Vec<u8>> = enc_asset_a_one
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_asset_b_one: Vec<Vec<u8>> = enc_asset_b_one
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_price_one: Vec<Vec<u8>> = enc_price_one
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_side_one: Vec<Vec<u8>> = enc_side_one
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();

    // Serialize user_one's server key
    let ser_server_key_one = bincode::serialize(&server_key_one)?;

    //
    // 3) user_two: De-serialize user_one's orders + key, then compare to user_two's plaintext orders.
    //    We'll do a 10x10 cross product to see if any match occurs.
    //
    let dec_enc_asset_a_one: Vec<FheUint32> = ser_enc_asset_a_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_asset_b_one: Vec<FheUint32> = ser_enc_asset_b_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_price_one: Vec<FheUint32> = ser_enc_price_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_side_one: Vec<FheBool> = ser_enc_side_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let server_key_one_for_two: ServerKey = bincode::deserialize(&ser_server_key_one)?;
    set_server_key(server_key_one_for_two);

    // "match" = (asset_a == asset_a) & (asset_b == asset_b) & (price == price) & (side != side).
    // Since all are buys for both, we expect no match.
    let mut match_ciphertexts_two: Vec<FheBool> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let eq_asset_a = dec_enc_asset_a_one[i].eq(user_two_orders.order[j].asset_a);
            let eq_asset_b = dec_enc_asset_b_one[i].eq(user_two_orders.order[j].asset_b);
            let eq_price = dec_enc_price_one[i].eq(user_two_orders.order[j].price);

            // side != side => (side == side).eq(false)
            let eq_side = dec_enc_side_one[i].eq(user_two_orders.order[j].a_for_b);
            let side_opposite = eq_side.eq(false);

            let is_match = eq_asset_a & eq_asset_b & eq_price & side_opposite;
            match_ciphertexts_two.push(is_match);
        }
    }

    // Serialize user_two's match results
    let ser_match_two: Vec<Vec<u8>> = match_ciphertexts_two
        .iter()
        .map(|m| safe_serialize_item(m).unwrap())
        .collect();

    //
    // 4) user_one decrypts user_two's results.
    //
    let match_ciphertexts_for_one: Vec<FheBool> = ser_match_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let mut any_match_for_one = false;
    for mc in &match_ciphertexts_for_one {
        let is_match: bool = mc.decrypt(&client_key_one);
        if is_match {
            any_match_for_one = true;
            println!("user_one sees a match found by user_two!");
        }
    }
    println!(
        "Result from user_two->user_one check: any_match_for_one = {}",
        any_match_for_one
    );
    assert!(!any_match_for_one, "We expected no match, but got one");

    //
    // 5) Reverse flow: user_two encrypts, user_one compares, user_two decrypts.
    //    Just to illustrate two-way private matching.
    //
    let config_two = ConfigBuilder::default().build();
    let (client_key_two, server_key_two) = generate_keys(config_two);

    let mut enc_asset_a_two: Vec<FheUint32> = Vec::new();
    let mut enc_asset_b_two: Vec<FheUint32> = Vec::new();
    let mut enc_price_two: Vec<FheUint32> = Vec::new();
    let mut enc_side_two: Vec<FheBool> = Vec::new();

    for order in &user_two_orders.order {
        enc_asset_a_two.push(FheUint32::try_encrypt(order.asset_a, &client_key_two)?);
        enc_asset_b_two.push(FheUint32::try_encrypt(order.asset_b, &client_key_two)?);
        enc_price_two.push(FheUint32::try_encrypt(order.price, &client_key_two)?);
        enc_side_two.push(FheBool::try_encrypt(order.a_for_b, &client_key_two)?);
    }

    let ser_enc_asset_a_two: Vec<Vec<u8>> = enc_asset_a_two
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_asset_b_two: Vec<Vec<u8>> = enc_asset_b_two
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_price_two: Vec<Vec<u8>> = enc_price_two
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_side_two: Vec<Vec<u8>> = enc_side_two
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();

    let ser_server_key_two = bincode::serialize(&server_key_two)?;

    // user_one compares (decrypts user_two ciphertext, compares to user_one plaintext)
    let dec_enc_asset_a_two: Vec<FheUint32> = ser_enc_asset_a_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_asset_b_two: Vec<FheUint32> = ser_enc_asset_b_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_price_two: Vec<FheUint32> = ser_enc_price_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_side_two: Vec<FheBool> = ser_enc_side_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let server_key_two_for_one: ServerKey = bincode::deserialize(&ser_server_key_two)?;
    set_server_key(server_key_two_for_one);

    let mut match_ciphertexts_one: Vec<FheBool> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let eq_asset_a = dec_enc_asset_a_two[j].eq(user_one_orders.order[i].asset_a);
            let eq_asset_b = dec_enc_asset_b_two[j].eq(user_one_orders.order[i].asset_b);
            let eq_price = dec_enc_price_two[j].eq(user_one_orders.order[i].price);

            // side != side
            let eq_side = dec_enc_side_two[j].eq(user_one_orders.order[i].a_for_b);
            let side_opposite = eq_side.eq(false);

            let is_match = eq_asset_a & eq_asset_b & eq_price & side_opposite;
            match_ciphertexts_one.push(is_match);
        }
    }

    let ser_match_one: Vec<Vec<u8>> = match_ciphertexts_one
        .iter()
        .map(|m| safe_serialize_item(m).unwrap())
        .collect();

    // user_two decrypts final match results
    let match_ciphertexts_for_two: Vec<FheBool> = ser_match_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let mut any_match_for_two = false;
    for mc in &match_ciphertexts_for_two {
        let is_match: bool = mc.decrypt(&client_key_two);
        if is_match {
            any_match_for_two = true;
            println!("user_two sees a match found by user_one!");
        }
    }

    println!(
        "Result from user_one->user_two check: any_match_for_two = {}",
        any_match_for_two
    );
    assert!(!any_match_for_two, "We expected no match, but got one");

    Ok(())
}
/// This test sets up 10 orders for each user where at least ONE buy/sell pair matches.
/// Now, the FHE matching only considers the price and the side (a_for_b).
#[tokio::test]
async fn test_match() -> Result<(), Box<dyn std::error::Error>> {
    //
    // 1) user_one has 10 buy orders. user_two has mostly buy orders but includes
    //    one SELL that exactly matches user_one's first buy (price=100).
    //
    let user_one_orders = user_one_orders_match();
    let user_two_orders = user_two_orders_match();

    //
    // 2) user_one => encrypt orders
    //
    let config_one = ConfigBuilder::default().build();
    let (client_key_one, server_key_one) = generate_keys(config_one);

    // Although we encrypt asset_a and asset_b as well, they won't be used in matching.
    let mut enc_asset_a_one: Vec<FheUint32> = Vec::new();
    let mut enc_asset_b_one: Vec<FheUint32> = Vec::new();
    let mut enc_price_one: Vec<FheUint32> = Vec::new();
    let mut enc_side_one: Vec<FheBool> = Vec::new();

    for order in &user_one_orders.order {
        enc_asset_a_one.push(FheUint32::try_encrypt(order.asset_a, &client_key_one)?);
        enc_asset_b_one.push(FheUint32::try_encrypt(order.asset_b, &client_key_one)?);
        enc_price_one.push(FheUint32::try_encrypt(order.price, &client_key_one)?);
        enc_side_one.push(FheBool::try_encrypt(order.a_for_b, &client_key_one)?);
    }

    let ser_enc_price_one: Vec<Vec<u8>> = enc_price_one
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_side_one: Vec<Vec<u8>> = enc_side_one
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();

    let ser_server_key_one = bincode::serialize(&server_key_one)?;

    //
    // 3) user_two => decrypt user_one's ciphertexts and compare to plaintext orders.
    // Now the matching is done only on price and side.
    //
    let dec_enc_price_one: Vec<FheUint32> = ser_enc_price_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_side_one: Vec<FheBool> = ser_enc_side_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let server_key_one_for_two: ServerKey = bincode::deserialize(&ser_server_key_one)?;
    set_server_key(server_key_one_for_two);

    // ---- TIMING for homomorphic comparisons ----
    let start_fhe = Instant::now();
    let mut match_ciphertexts_two: Vec<FheBool> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            // Now only compare price...
            let eq_price = dec_enc_price_one[i].eq(user_two_orders.order[j].price);

            // ...and side (a_for_b): compute "side != side" via (side == side).eq(false)
            let eq_side = dec_enc_side_one[i].eq(user_two_orders.order[j].a_for_b);
            let side_opposite = eq_side.eq(false);

            // Overall match criteria using only the price and side.
            let is_match = eq_price & side_opposite;
            match_ciphertexts_two.push(is_match);
        }
    }
    let fhe_duration = start_fhe.elapsed();
    println!("(user_two) Homomorphic comparison took: {:?}", fhe_duration);

    let ser_match_two: Vec<Vec<u8>> = match_ciphertexts_two
        .iter()
        .map(|m| safe_serialize_item(m).unwrap())
        .collect();

    // ---- TIMING for decrypting results ----
    let start_decrypt = Instant::now();
    let match_ciphertexts_for_one: Vec<FheBool> = ser_match_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let mut any_match_for_one = false;
    for mc in &match_ciphertexts_for_one {
        let is_match: bool = mc.decrypt(&client_key_one);
        if is_match {
            any_match_for_one = true;
            println!("user_one sees a match found by user_two!");
        }
    }
    let decrypt_duration = start_decrypt.elapsed();
    println!(
        "(user_one) Decryption of user_two's results took: {:?}",
        decrypt_duration
    );

    println!(
        "Result from user_two->user_one check: any_match_for_one = {}",
        any_match_for_one
    );
    // Expect at least one match
    assert!(any_match_for_one, "Expected a match, found none");

    //
    // 4) user_two => encrypt orders, send to user_one for comparison.
    //
    let config_two = ConfigBuilder::default().build();
    let (client_key_two, server_key_two) = generate_keys(config_two);

    let mut enc_asset_a_two: Vec<FheUint32> = Vec::new();
    let mut enc_asset_b_two: Vec<FheUint32> = Vec::new();
    let mut enc_price_two: Vec<FheUint32> = Vec::new();
    let mut enc_side_two: Vec<FheBool> = Vec::new();

    for order in &user_two_orders.order {
        enc_asset_a_two.push(FheUint32::try_encrypt(order.asset_a, &client_key_two)?);
        enc_asset_b_two.push(FheUint32::try_encrypt(order.asset_b, &client_key_two)?);
        enc_price_two.push(FheUint32::try_encrypt(order.price, &client_key_two)?);
        enc_side_two.push(FheBool::try_encrypt(order.a_for_b, &client_key_two)?);
    }

    let ser_enc_price_two: Vec<Vec<u8>> = enc_price_two
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_side_two: Vec<Vec<u8>> = enc_side_two
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();

    let ser_server_key_two = bincode::serialize(&server_key_two)?;

    // user_one => compare the received ciphertexts with its plaintext orders,
    // using only price and side.
    let dec_enc_price_two: Vec<FheUint32> = ser_enc_price_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_side_two: Vec<FheBool> = ser_enc_side_two
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let server_key_two_for_one: ServerKey = bincode::deserialize(&ser_server_key_two)?;
    set_server_key(server_key_two_for_one);

    // ---- TIMING for homomorphic comparisons (second round) ----
    let start_fhe_two = Instant::now();
    let mut match_ciphertexts_one: Vec<FheBool> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let eq_price = dec_enc_price_two[j].eq(user_one_orders.order[i].price);
            let eq_side = dec_enc_side_two[j].eq(user_one_orders.order[i].a_for_b);
            let side_opposite = eq_side.eq(false);

            let is_match = eq_price & side_opposite;
            match_ciphertexts_one.push(is_match);
        }
    }
    let fhe_duration_two = start_fhe_two.elapsed();
    println!(
        "(user_one) Homomorphic comparison (2nd round) took: {:?}",
        fhe_duration_two
    );

    let ser_match_one: Vec<Vec<u8>> = match_ciphertexts_one
        .iter()
        .map(|m| safe_serialize_item(m).unwrap())
        .collect();

    // ---- TIMING for decryption (second round) ----
    let start_decrypt_two = Instant::now();
    let match_ciphertexts_for_two: Vec<FheBool> = ser_match_one
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let mut any_match_for_two = false;
    for mc in &match_ciphertexts_for_two {
        let is_match: bool = mc.decrypt(&client_key_two);
        if is_match {
            any_match_for_two = true;
            println!("user_two sees a match found by user_one!");
        }
    }
    let decrypt_duration_two = start_decrypt_two.elapsed();
    println!(
        "(user_two) Decryption of user_one's results took: {:?}",
        decrypt_duration_two
    );

    println!(
        "Result from user_one->user_two check: any_match_for_two = {}",
        any_match_for_two
    );
    // Expect at least one match
    assert!(any_match_for_two, "Expected a match, found none");

    Ok(())
}

#[tokio::test]
async fn test_match_with_timing() -> Result<(), Box<dyn std::error::Error>> {
    // Generate test data with 10 orders for each party and force at least one match.
    let (orders_a, orders_b) = create_order_test_data(10, true);

    // --- Party A (user_one) encrypts its orders. ---
    let config_one = ConfigBuilder::default().build();
    let (client_key_one, server_key_one) = generate_keys(config_one);

    // Encrypt the fields used in the matching (price and side).
    let mut enc_price_a: Vec<FheUint32> = Vec::with_capacity(orders_a.order.len());
    let mut enc_side_a: Vec<FheBool> = Vec::with_capacity(orders_a.order.len());

    for order in &orders_a.order {
        enc_price_a.push(FheUint32::try_encrypt(order.price, &client_key_one)?);
        enc_side_a.push(FheBool::try_encrypt(order.a_for_b, &client_key_one)?);
    }

    // Serialize the encrypted price and side for party A
    let ser_enc_price_a: Vec<Vec<u8>> = enc_price_a
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();
    let ser_enc_side_a: Vec<Vec<u8>> = enc_side_a
        .iter()
        .map(|ct| safe_serialize_item(ct).unwrap())
        .collect();

    // Serialize Party A's server key so that Party B can use it.
    let ser_server_key_one = bincode::serialize(&server_key_one)?;

    // --- Party B (user_two) decrypts Party A's ciphertexts and compares with its plaintext orders ---
    // For matching, we only consider the price and the side.
    let dec_enc_price_a: Vec<FheUint32> = ser_enc_price_a
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let dec_enc_side_a: Vec<FheBool> = ser_enc_side_a
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();

    let server_key_one_for_two: ServerKey = bincode::deserialize(&ser_server_key_one)?;
    set_server_key(server_key_one_for_two);

    // Time each individual FHE matching computation.
    let mut match_results: Vec<FheBool> = Vec::new();

    let start = Instant::now();
    // For each order from Party A (encrypted) and each order from Party B (plaintext), do a match check.
    for (i, _) in orders_a.order.iter().enumerate() {
        for (_, order_b) in orders_b.order.iter().enumerate() {
            // Compare the price using homomorphic encryption.
            let eq_price = dec_enc_price_a[i].eq(order_b.price);

            // Compare the side: We want a "not equal" check.
            // Compute (encrypted side == plaintext side) then invert it.
            let eq_side = dec_enc_side_a[i].eq(order_b.a_for_b);
            let side_opposite = eq_side.eq(false);

            // Overall match: price equal and side opposite.
            let is_match = eq_price & side_opposite;
            match_results.push(is_match);
        }
    }

    // For reporting, compute statistics for the individual computation times.
    let total: Duration = start.elapsed();
    let count = orders_a.order.len() as u32;
    let average = total / count;
    println!(
        "Performed {} individual FHE comparisons; total time: {:?}, average per comparison: {:?}",
        count, total, average
    );

    // Serialize the results.
    let ser_match_results: Vec<Vec<u8>> = match_results
        .iter()
        .map(|m| safe_serialize_item(m).unwrap())
        .collect();

    // ---- Time the decryption of results ----
    let start_decrypt = Instant::now();
    let dec_match_results: Vec<FheBool> = ser_match_results
        .iter()
        .map(|bytes| safe_deserialize_item(bytes).unwrap())
        .collect();
    let mut any_match_found = false;
    for result in &dec_match_results {
        if result.decrypt(&client_key_one) {
            any_match_found = true;
            println!("A match was detected in one of the comparisons.");
            // For the purposes of this test we can break here if one match is all we need.
            break;
        }
    }
    let decrypt_duration = start_decrypt.elapsed();
    println!(
        "Decryption of all match results took: {:?}",
        decrypt_duration
    );

    // Assert that at least one match was found.
    assert!(
        any_match_found,
        "Expected at least one match, but found none"
    );

    Ok(())
}
