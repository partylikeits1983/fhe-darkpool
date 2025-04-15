use std::time::Instant;
use tfhe::prelude::*;
use tfhe::{ConfigBuilder, FheUint32, generate_keys, set_server_key};

#[tokio::test]
async fn test_fhe_vector_overlap_with_timing() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Setup TFHE configuration and generate keys.
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key); // Ensure the server key is set on the current thread.

    // 2) Define two plaintext vectors.
    // Vector A will be encrypted; Vector B remains plaintext.
    // In this example, both vectors share overlapping values.
    let vec_a: Vec<u32> = vec![42, 30, 15, 60, 22];
    let vec_b: Vec<u32> = vec![10, 41, 25, 31, 22];

    // 3) Encrypt each number in vector A.
    let encrypted_vec_a: Vec<FheUint32> = vec_a
        .iter()
        .map(|&num| FheUint32::try_encrypt(num, &client_key))
        .collect::<Result<_, _>>()?;

    // 4) In a nested loop, perform FHE equality checks between each element from encrypted_vec_a and each plaintext value in vec_b.
    let start = Instant::now();
    let mut match_results = Vec::new();
    for enc_val in &encrypted_vec_a {
        for &plain_val in &vec_b {
            let eq_ciphertext = enc_val.eq(plain_val);
            match_results.push(eq_ciphertext);
        }
    }
    let total_duration = start.elapsed();

    // 5) Calculate the average time per comparison.
    let num_comparisons = encrypted_vec_a.len() * vec_b.len();
    let avg_duration = total_duration / (num_comparisons as u32);
    println!("Performed {} comparisons", num_comparisons);
    println!("Total FHE equality comparisons took: {:?}", total_duration);
    println!("Average time per comparison: {:?}", avg_duration);

    // 6) Decrypt the results to determine if any overlapping value was found.
    let mut found_overlap = false;
    for eq_ciphertext in match_results {
        if eq_ciphertext.decrypt(&client_key) {
            found_overlap = true;
            break; // Stop once a match is detected.
        }
    }
    println!("Found overlapping equal value: {}", found_overlap);

    // Ensure at least one overlapping value exists.
    assert!(
        found_overlap,
        "Expected to find an overlapping value between the vectors."
    );

    Ok(())
}
