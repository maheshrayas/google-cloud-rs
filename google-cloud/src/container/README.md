cargo test --features container
cargo test --features container --nocapture
export GCP_TEST_PROJECT=admin-project-307508
 export GOOGLE_APPLICATION_CREDENTIALS=$(cat '/Users/rayas/Downloads/admin-project-307508-54152cff9333.json')