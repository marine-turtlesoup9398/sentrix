# SENTRIX REST API Security Guide

Production security model and network binding reference for the **SENTRIX Axum REST API Server**.

---

## 1. Local Network Binding Policy

- **Default Address**: `127.0.0.1` (Localhost loopback only).
- **Default Port**: `7070` (Configurable via `sentrix.toml` or `sentrix serve --port <PORT>`).
- **Security Boundary**: By default, SENTRIX does NOT expose API endpoints to external public networks (`0.0.0.0`).

---

## 2. API Endpoints & Version Namespace

All REST routes are namespaced under `/api/`:

| Endpoint Route | HTTP Method | Description & Security Boundary |
|---|---|---|
| `/api/status` | `GET` | Health check & repository summary metrics. |
| `/api/graph` | `GET` | JSON representation of Software Knowledge Graph. |
| `/api/architecture` | `GET` | System architecture pattern classification & confidence. |
| `/api/hotspots` | `GET` | Engineering risk hotspots & complexity summary. |
| `/api/findings` | `GET` | Secret findings & data-flow taint analysis findings. |
| `/api/impact` | `POST` | Change impact radius calculation for target component. |
| `/api/ask` | `POST` | Grounded AI question answering with prompt injection defense. |
| `/api/evolution/predict` | `POST` | Feature-weighted predictive change risk evaluation. |
| `/api/evolution/recommend-tests` | `POST` | Call-graph backed test recommendation engine. |
| `/api/evolution/ownership` | `POST` | Historical contribution concentration metrics. |

---

## 3. CORS Policy & Request Protection

- **CORS Rules**: Configured for local Web GUI communication (`http://localhost:7070` and `http://127.0.0.1:7070`).
- **Payload Limits**: Request body size limits prevent memory exhaustion from oversized JSON payloads.
