# IOTA-Ed25519-addresses
Generates base58 encoded addresses as used in https://github.com/iotaledger/goshimmer

#
Add `iota_ed25519_addresses = { git = "https://github.com/Thoralf-M/IOTA-Ed25519-addresses" }` to your dependencies in the Cargo.toml

Then you can use it like here https://github.com/Thoralf-M/IOTA-Ed25519-addresses/blob/master/src/bin/main.rs

`cargo run --release`

```
Address: QbmLnx2zrQQJ1U5JyR54qrzWgQvqiD4wNjoRKYqCjo8q
Generated addresses in 320.80µs
[
    "0: QbmLnx2zrQQJ1U5JyR54qrzWgQvqiD4wNjoRKYqCjo8q",
    "1: LbNeQyMtf2HF1D6oQWabsrd6wPX1CUhgacz8htoN6vJs",
    "2: PDAfUpQhNmj5wBSYTQ8oRFxgbzv4A8HAL65XSnYCPEbp",
    "3: MMWpuFkQimmzgrhAufRPf9dYMBn3qMpe9hQj5D4Q3nJY",
    "4: MqFRrkwTLbYHPkQUN2UKT6MPo8otK1GPSKM6n2VgJm5b",
    "5: aYJLGyrySQo3SmejWoiU7oovCWFbpinA8mWFVrzYs43A",
    "6: MFh4ehsr4CNCxZVbtSEGNi7YroKg2TqdKntrAQqNm3Ez",
    "7: XCX5R9Jr1nGe5MxiK7fdJpGUWqmeCsgKNLMGi6Pbe6Fq",
    "8: PMfn9to216w6njE5KHLBNGf2VNKPcsNVzCtPBs4H1DDh",
    "9: N7ZiPAJKCiZFVDgwigXs2UCFhvDodZ5yRRE4YfXuEXpn",
]
```
