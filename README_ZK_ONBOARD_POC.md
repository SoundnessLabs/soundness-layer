zk-Onboard PoC (mock)
---------------------

This package contains a simple proof-of-concept for a zk-Onboard CLI module.

Files included:
- soundness-cli/src/zk_onboard/mod.rs
- soundness-cli/src/zk_onboard/zk_onboard.rs
- soundness-cli/tests/zk_onboard_test.rs

Integration notes:
- Add the following dependencies to `soundness-cli/Cargo.toml`:
  serde = { version = "1", features = ["derive"] }
  sha2 = "0.10"
  hex = "0.4"
  rand = "0.8"

- To integrate into `main.rs`, add:
  mod zk_onboard;
  use zk_onboard::zk_onboard::ZkOnboard;

  // Example CLI snippet:
  // let zk = ZkOnboard::new();
  // let proof = zk.register_user("alice")?;
  // println!("proof: {}", serde_json::to_string_pretty(&proof)?);

- Tests assume crate name `soundness_cli`. Adjust paths if your crate name differs.
