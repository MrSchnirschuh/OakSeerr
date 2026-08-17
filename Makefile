.PHONY: test lint format

test:
	cd backend && cargo test --quiet
	cd frontend && npm test

lint:
	cd backend && cargo clippy --quiet -- -D warnings
	cd frontend && npm run lint

format:
	cd backend && cargo fmt
	cd frontend && npm run format
