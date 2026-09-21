# Requires DECISION_API_KEY to be set.
test-all:
    cargo test -- --ignored

# Requires DECISION_API_KEY to be set.
test-e2e:
    cargo test --test end_to_end -- --ignored

test-unit:
    cargo test
