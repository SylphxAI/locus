/**
 * After `changeset version`, align platform packages, optionalDependencies,
 * server.json, and the Rust SERVER_VERSION with @sylphx/locus.
 *
 * Edits are textual so package.json keeps its existing indentation. A full
 * JSON.stringify rewrite would flip the repo's tab indent and fail CI.
 */
import fs from 'node:fs'
import path from 'node:path'

const repoRoot = path.resolve(import.meta.dirname, '..')
const mcpPackagePath = path.join(repoRoot, 'packages/mcp-server/package.json')
const mcpPkg = JSON.parse(fs.readFileSync(mcpPackagePath, 'utf8')) as {
	version: string
	optionalDependencies?: Record<string, string>
}

const version = mcpPkg.version
const platformDir = path.join(repoRoot, 'packages/mcp-server/npm')
const platforms = fs
	.readdirSync(platformDir, { withFileTypes: true })
	.filter((entry) => entry.isDirectory())
	.map((entry) => entry.name)

function replaceFirst(
	text: string,
	pattern: RegExp,
	replacement: string
): { text: string; changed: boolean } {
	const next = text.replace(pattern, replacement)
	return { text: next, changed: next !== text }
}

let updated = false

for (const platform of platforms) {
	const pkgPath = path.join(platformDir, platform, 'package.json')
	if (!fs.existsSync(pkgPath)) continue
	const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8')) as { name: string; version: string }
	if (!pkg.name.startsWith('@sylphx/locus-')) {
		throw new Error(`[sync-platform-versions] unexpected platform package ${pkg.name}`)
	}
	if (pkg.version === version) continue
	const raw = fs.readFileSync(pkgPath, 'utf8')
	const replaced = replaceFirst(raw, /("version"\s*:\s*")[^"]+(")/, `$1${version}$2`)
	if (!replaced.changed) {
		throw new Error(`[sync-platform-versions] no version field in ${pkgPath}`)
	}
	fs.writeFileSync(pkgPath, replaced.text)
	console.log(`[sync-platform-versions] ${pkg.name}: ${pkg.version} → ${version}`)
	updated = true
}

let mcpRaw = fs.readFileSync(mcpPackagePath, 'utf8')
for (const [name, current] of Object.entries(mcpPkg.optionalDependencies ?? {})) {
	if (!name.startsWith('@sylphx/locus-')) continue
	if (current === version) continue
	const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
	const replaced = replaceFirst(
		mcpRaw,
		new RegExp(`("${escaped}"\\s*:\\s*")[^"]+(")`),
		`$1${version}$2`
	)
	if (!replaced.changed) {
		throw new Error(`[sync-platform-versions] missing optionalDependency ${name}`)
	}
	mcpRaw = replaced.text
	console.log(`[sync-platform-versions] ${name}: ${current} → ${version}`)
	updated = true
}
if (mcpRaw !== fs.readFileSync(mcpPackagePath, 'utf8')) {
	fs.writeFileSync(mcpPackagePath, mcpRaw)
}

const serverPath = path.join(repoRoot, 'server.json')
const server = JSON.parse(fs.readFileSync(serverPath, 'utf8')) as {
	version: string
	description: string
	packages: Array<{ version: string }>
}
if ([...server.description].length > 100) {
	throw new Error(
		`[sync-platform-versions] server.json description is ${[...server.description].length} characters; MCP registry allows 100`
	)
}
if (server.version !== version || server.packages[0]?.version !== version) {
	const serverRaw = fs.readFileSync(serverPath, 'utf8')
	const replaced = serverRaw.replace(/("version"\s*:\s*")[^"]+(")/g, `$1${version}$2`)
	if (replaced === serverRaw) {
		throw new Error('[sync-platform-versions] server.json has no version field')
	}
	const parsed = JSON.parse(replaced) as { version: string; packages: Array<{ version: string }> }
	if (parsed.version !== version || parsed.packages[0]?.version !== version) {
		throw new Error('[sync-platform-versions] server.json version replace did not stick')
	}
	fs.writeFileSync(serverPath, replaced)
	console.log(`[sync-platform-versions] server.json → ${version}`)
	updated = true
}

const rustLib = path.join(repoRoot, 'crates/coderag-mcp-server/src/lib.rs')
const rustRaw = fs.readFileSync(rustLib, 'utf8')
const rustNext = rustRaw.replace(/(pub const SERVER_VERSION: &str = ")[^"]*(";)/, `$1${version}$2`)
if (rustNext === rustRaw) {
	if (!rustRaw.includes(`pub const SERVER_VERSION: &str = "${version}";`)) {
		throw new Error('[sync-platform-versions] SERVER_VERSION pattern not found')
	}
} else {
	fs.writeFileSync(rustLib, rustNext)
	console.log(`[sync-platform-versions] SERVER_VERSION → ${version}`)
	updated = true
}

if (!updated) {
	console.log(`[sync-platform-versions] Already synced at ${version}`)
} else {
	console.log(`[sync-platform-versions] Done (mcp @ ${version})`)
}
