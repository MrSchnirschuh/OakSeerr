# OakSeerr

All-in-one media request manager for Jellyfin. Movies, TV shows, music, books and comics — unified.

Built on the foundation of [Seerr](https://github.com/seerr-team/seerr) (MIT) with features ported from Musicseerr and extended for books and comics.

## Features

- **Movies** — Request and track via Radarr
- **TV Shows** — Request and track via Sonarr
- **Music** — Request albums and artists via Lidarr
- **Books** — Request via Readarr
- **Comics** — Request via Mylar3
- **Download Status** — Real-time progress from Sabnzbd
- **Jellyfin SSO** — Single sign-on with your Jellyfin server
- **Jellyfin Theme** — Native look & feel matching Jellyfin's dark theme
- **Custom CSS** — Inject any CSS theme (like Abyss) for full customization

## Security Model

- **JWT auth** — stateless bearer tokens, verified on every request
- **Admin middleware** — admin-only routes require a user with permission level `100`
- **First-user-only admin** — the very first user created (via Jellyfin SSO or demo mode) is granted admin permissions automatically; subsequent users start with no permissions
- **Strict CORS** — only the configured `CORS_ORIGIN` is allowed; credentials are never sent to wildcard origins

## Stack

- **Backend:** Rust (Axum), SQLite, `sqlx` migrations, `jsonwebtoken`
- **Frontend:** Next.js 15 + React 19, Tailwind CSS
- **Integration clients:** `reqwest` with Jellyfin, Radarr, Sonarr, Lidarr, Readarr, Mylar3, Sabnzbd

## Quick Start

### Docker Compose (recommended)

```bash
# Clone the repository
git clone https://github.com/MrSchnirschuh/OakSeerr.git
cd OakSeerr

# Copy and edit configuration
cp .env.example .env
nano .env

# Start
docker compose up -d
```

Open http://localhost:5055

### Manual

```bash
# Backend
cd backend
cargo run --release

# Frontend (separate terminal)
cd frontend
npm install
npm run build
```

## Configuration

See `.env.example` for all available options.

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:///app/config/oakseerr.db?mode=rwc` | SQLite database path |
| `LISTEN_ADDR` | `0.0.0.0:5055` | Server listen address |
| `JWT_SECRET` | required | Secret for JWT tokens |
| `CORS_ORIGIN` | `http://localhost:5055` | Allowed frontend origin |
| `JELLYFIN_URL` | — | Jellyfin server URL (for SSO) |
| `JELLYFIN_API_KEY` | — | Jellyfin API key |

## Integrations

Configure your media management services in Settings > Integrations:

- [Radarr](https://radarr.video/) — Movies
- [Sonarr](https://sonarr.tv/) — TV Shows
- [Lidarr](https://lidarr.audio/) — Music
- [Readarr](https://readarr.com/) — Books
- [Mylar3](https://github.com/mylar3/mylar3) — Comics
- [Sabnzbd](https://sabnzbd.org/) — Download client
- [Prowlarr](https://prowlarr.com/) — Indexer manager

## Custom CSS Themes

OakSeerr supports custom CSS injection. Add any CSS file URL in Settings to theme the UI. The default theme matches Jellyfin's native dark theme exactly.

Popular themes:
- [Abyss](https://github.com/AumGupta/abyss-jellyfin) — `@import url('https://cdn.jsdelivr.net/gh/AumGupta/abyss-jellyfin@main/abyss.css');`

## Development

Use the `Makefile` for common tasks:

```bash
make test   # backend cargo test + frontend npm test
make lint   # cargo clippy + npm run lint
make format # cargo fmt + npm run format
```

## License

MIT
