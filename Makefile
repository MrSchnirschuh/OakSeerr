.PHONY: test lint

test:
	cd backend && cargo test --quiet
	cd frontend && npm test

lint:
	cd backend && cargo clippy --quiet -- -D warnings
	cd frontend && npm run lint

