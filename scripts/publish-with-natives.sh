#!/usr/bin/env bash
# Publish path only. The version-PR path must not run this: it rewrites tracked
# native binaries, and changesets/action commits the whole worktree.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== multi-arch artifacts =="
ls -laR artifacts || true

bun run build
bun scripts/assemble-multiarch-natives.ts

# Stage linux x64 into bin/native as a documented linux fallback. Both natives
# are required so tools/call can resolve sibling locus-cli without a monorepo build.
mkdir -p packages/mcp-server/bin/native bin/native
for name in locus-mcp-server locus-cli; do
	if [ -f "packages/mcp-server/npm/linux-x64-gnu/${name}" ]; then
		cp "packages/mcp-server/npm/linux-x64-gnu/${name}" "packages/mcp-server/bin/native/${name}"
		cp "packages/mcp-server/npm/linux-x64-gnu/${name}" "bin/native/${name}"
		chmod +x "packages/mcp-server/bin/native/${name}" "bin/native/${name}"
	fi
done

chmod +x packages/mcp-server/bin/locus bin/locus
test -x packages/mcp-server/bin/locus
head -n 8 packages/mcp-server/bin/locus

# Fail-closed: every platform package must contain both natives.
for dir in packages/mcp-server/npm/*/; do
	test -f "${dir}package.json"
	for name in locus-mcp-server locus-cli; do
		test -x "${dir}${name}"
		test "$(wc -c < "${dir}${name}")" -gt 100000
		echo "OK $(basename "$dir") ${name} $(wc -c < "${dir}${name}") bytes"
		file "${dir}${name}" || true
	done
done

# The version PR already synced these. Repeat so a packed tarball cannot ship
# optionalDependencies that point at the previous native version.
bun scripts/sync-platform-versions.ts

sylphx-changesets-publish
