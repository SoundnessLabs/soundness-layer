use soundness_cli::zk_onboard::zk_onboard::{ZkOnboard, ZkProof};

#[test]
fn e2e_zk_onboard_register_verify() {
    let onboard = ZkOnboard::new();
    let proof = onboard.register_user("testuser").expect("register failed");
    assert_eq!(proof.username, "testuser");
    assert!(onboard.verify_proof(&proof).unwrap());
}
