use std::time::Instant;

fn main() {
    //generate a new wallet with a base58 seed
    let wallet =
        iota_ed25519_addresses::Wallet::new("Gnp4f3nn8RFgEMpjsr58DEWKQHTfpGP5wvic5e8aeSBp");

    //generate a single address with index
    println!("Address: {}", wallet.address(0));

    //generate multiple addresses
    let time_start = Instant::now();
    let mut addresses = vec![];
    for i in 0..10 {
        addresses.push(format!("{}: {}", i, wallet.address(i)));
        // addresses.push(wallet.address(i));
    }
    println!("Generated addresses in {:.2?}", time_start.elapsed());
    println!("{:#?}", addresses);
}
