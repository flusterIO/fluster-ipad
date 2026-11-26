#!/usr/bin/env bash
# Smoke test for the REST file system sync API against a running cdrm_server.
# Usage: ./test_fs_sync_api.sh /absolute/path/to/notes [port]
set -euo pipefail
ROOT="${1:?Pass the absolute path of a notes directory}"
PORT="${2:-${CDRM_SERVER_PORT:-3005}}"
BASE="http://127.0.0.1:${PORT}/api/v1"

echo "== health"
curl -sS "${BASE}/health"; echo

echo "== manifest"
MANIFEST=$(curl -sS -X POST "${BASE}/fs_sync/manifest" \
  -H 'Content-Type: application/json' \
  -d "$(python3 -c 'import json,sys; print(json.dumps({"root": sys.argv[1]}))' "$ROOT")")
echo "$MANIFEST" | python3 -c 'import json,sys; m=json.load(sys.stdin); print(len(m["entries"]), "entries,", len(m["skipped"]), "skipped"); [print(" ", e["relative_path"], e["content_hash"][:12]) for e in m["entries"][:10]]'

echo "== contents (first 3 files)"
PATHS=$(echo "$MANIFEST" | python3 -c 'import json,sys; print(json.dumps([e["relative_path"] for e in json.load(sys.stdin)["entries"][:3]]))')
curl -sS -X POST "${BASE}/fs_sync/contents" \
  -H 'Content-Type: application/json' \
  -d "$(python3 -c 'import json,sys; print(json.dumps({"root": sys.argv[1], "relative_paths": json.loads(sys.argv[2])}))' "$ROOT" "$PATHS")" \
  | python3 -c 'import json,sys; r=json.load(sys.stdin); [print(" ", f["relative_path"], len(f["content"]), "chars") for f in r["files"]]; print(" failures:", r["failures"])'
