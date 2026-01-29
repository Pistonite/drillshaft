/**
 * Update metadata.toml
 *
 * Usage: node --experimental-strip-types update-metadata.ts [PACKAGE]
 *
 * PACKAGE is the package to update, or all packages if omitted
 */

//* To get type checking in your IDE, install @types/node with a package manager */
/// <reference types="node" />
/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { execSync } from "node:child_process";

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const METADATA_TOML = path.join(SCRIPT_DIR, "metadata.toml");
const TEMP_DIR = path.join(SCRIPT_DIR, "update-metadata-temp");
const GITHUB_API = "https://api.github.com/";
const CRATESIO_API = "https://crates.io/api/v1/";
const ARCHLINUX_API = "https://archlinux.org/packages/search/json/";

// === CONSTANT HELPERS ======================================================
const cfg_windows = (x: string) => `'cfg(windows)'.${x}`;
const cfg_windows_arm64 = (x: string) => `'cfg(all(windows,target_arch="aarch64"))'.${x}`;
const cfg_windows_x64 = (x: string) => `'cfg(all(windows,target_arch="x86_64"))'.${x}`;
const cfg_arm64 = (x: string) => `'cfg(target_arch="aarch64")'.${x}`;
const cfg_x64 = (x: string) => `'cfg(target_arch="x86_64")'.${x}`;
const cfg_linux = (x: string) => `'cfg(target_os="linux")'.${x}`;

// === CLI ===================================================================
const main = async () => {
    try {
        await main_internal();
    } catch (e) {
        console.error(`ERROR: ${e instanceof Error ? e.message : e}`);
        process.exit(1);
    }
};

const main_internal = async () => {
    const args = process.argv.slice(2);

    if (args.includes("-h") || args.includes("--help")) {
        console.log("Usage: node --experimental-strip-types update-metadata.ts [PACKAGE]");
        console.log();
        console.log("Update metadata.toml with latest package versions.");
        console.log();
        console.log("Arguments:");
        console.log("  PACKAGE  Package to update (updates all if omitted)");
        return;
    }

    const metadata_content = load_metadata();
    const all_packages = get_package_names(metadata_content);

    // determine which packages to update
    let packages_to_update: string[];
    if (args.length > 0) {
        const pkg = args[0];
        if (!all_packages.includes(pkg)) {
            throw new Error(`package '${pkg}' not found in metadata.toml`);
        }
        packages_to_update = [pkg];
    } else {
        packages_to_update = all_packages;
    }

    const results = await Promise.allSettled(
        packages_to_update.map(pkg => fetch_by_package(metadata_content, pkg))
    );

    let has_error = false;
    const updates: PackageUpdate[] = [];
    for (let i = 0; i < results.length; i++) {
        const pkg = packages_to_update[i];
        const result = results[i];
        if (result.status === "rejected") {
            console.error(`ERROR: ${pkg}: ${result.reason}`);
            has_error = true;
        } else {
            updates.push({ [pkg]: result.value });
        }
    }
    if (has_error) {
        throw new Error("there were errors fetching package updates");
    }

    const merged = merge(...updates);
    const updated = update_metadata(metadata_content, merged);

    if (updated) {
        save_metadata(metadata_content);
        console.log("metadata.toml updated");
    } else {
        console.log("already up to date");
    }
};

// === PACKAGES ==============================================================

type PackageUpdate = Record<string, PackageEntry>;
type PackageEntry = Record<string, string>;

const fetch_by_package = async (meta: string[], pkg: string): Promise<PackageEntry> => {
    console.log(`fetching '${pkg}'...`);
    /** Fetch the latest version of a package, returns an object */
    switch (pkg) {
        case "_7z": {
            return await fetch_from_github_release({
                repo: get_metadata(meta, pkg, "REPO"),
                tag: (tags) => {
                    for (const tag of tags) { if (tag.includes("preview")) { return tag; } }
                    throw new Error("failed to find pwsh preview release");
                },
                query: async (repo, tag, artifacts) => {
                    tag = strip_v(tag);
                    const arm64_zip = `PowerShell-${tag}-win-arm64.zip`; if (!artifacts.includes(arm64_zip)) { throw new Error("failed to find pwsh arm64 artifact"); }
                    const x64_zip = `PowerShell-${tag}-win-x64.zip`; if (!artifacts.includes(x64_zip)) { throw new Error("failed to find pwsh x64 artifact"); }
                    const arm64_url = github_release_url(repo, 'v'+tag, arm64_zip);
                    const x64_url = github_release_url(repo, 'v'+tag, x64_zip);
                    const arm64_hash = await sha256_from_url(arm64_url);
                    const x64_hash = await sha256_from_url(x64_url);

                    return {
                        VERSION: tag,
                        [cfg_arm64("SHA")]: arm64_hash,
                        [cfg_x64("SHA")]: x64_hash,
                    };
                }
            });
        }
        case "pwsh": 
            return await fetch_from_github_release({
                repo: get_metadata(meta, pkg, "REPO"),
                tag: (tags) => {
                    for (const tag of tags) { if (tag.includes("preview")) { return tag; } }
                    throw new Error("failed to find pwsh preview release");
                },
                query: async (repo, tag, artifacts) => {
                    tag = strip_v(tag);
                    const arm64_zip = `PowerShell-${tag}-win-arm64.zip`; if (!artifacts.includes(arm64_zip)) { throw new Error("failed to find pwsh arm64 artifact"); }
                    const x64_zip = `PowerShell-${tag}-win-x64.zip`; if (!artifacts.includes(x64_zip)) { throw new Error("failed to find pwsh x64 artifact"); }
                    const arm64_url = github_release_url(repo, 'v'+tag, arm64_zip);
                    const x64_url = github_release_url(repo, 'v'+tag, x64_zip);
                    const arm64_hash = await sha256_from_url(arm64_url);
                    const x64_hash = await sha256_from_url(x64_url);

                    return {
                        VERSION: tag,
                        [cfg_arm64("SHA")]: arm64_hash,
                        [cfg_x64("SHA")]: x64_hash,
                    };
                }
            });
        case "cargo_binstall": return await fetch_from_cratesio({ crate: "cargo-binstall" });
        case "coreutils": {
            return {
                ...await fetch_from_cratesio({ crate: "eza", query: (v) => ({ "eza.VERSION": v }) }),
                ...await fetch_from_cratesio({ crate: "coreutils", query: (v) => ({ "uutils.VERSION": v }) }),
                ...await fetch_from_arch_linux({
                    package: "zip",
                    query: async (version) => { return { "zip.VERSION": version } }
                }),
                ...await fetch_from_arch_linux({
                    package: "unzip",
                    query: async (version) => { return { "unzip.VERSION": version } }
                }),
                ...await fetch_from_arch_linux({
                    package: "tar",
                    query: async (version) => { return { "tar.VERSION": version } }
                }),
            };
        }
        case "shellutils": {
            const repo = get_metadata(meta, pkg, "REPO");
            const commit = await fetch_latest_commit(repo);
            return {
                REPO: repo,
                COMMIT: commit,
                ...await fetch_from_cargo_toml({
                    repo, ref: commit, path: "packages/which/Cargo.toml",
                    query: (v) => ({ "which.VERSION": v })
                }),
                ...await fetch_from_cargo_toml({
                    repo, ref: commit, path: "packages/viopen/Cargo.toml",
                    query: (v) => ({ "viopen.VERSION": v })
                }),
                ...await fetch_from_cargo_toml({
                    repo, ref: commit, path: "packages/vipath/Cargo.toml",
                    query: (v) => ({ "vipath.VERSION": v })
                }),
                ...await fetch_from_cargo_toml({
                    repo, ref: commit, path: "packages/n/Cargo.toml",
                    query: (v) => ({ "n.VERSION": v })
                }),
                ...await fetch_from_cargo_toml({
                    repo, ref: commit, path: "packages/wsclip/Cargo.toml",
                    query: (v) => ({ "wsclip.VERSION": v })
                }),
            }
        }
        case "git": {
            return {
                ...await fetch_from_github_release({
                    repo: "https://github.com/microsoft/git",
                    query: async (_, tag) => {
                        tag = strip_v(tag);
                        const i = tag.indexOf(".vfs");
                        if (i===-1) { throw new Error("latest microsoft/git is not vfs"); }
                        return { [cfg_windows("VERSION")]: tag.substring(0, i) }
                    }
                }),
                ...await fetch_from_arch_linux({
                    package: "git",
                    query: async (version) => { return { [cfg_linux("VERSION")]: version } }
                }),
            }
        }
        case "perl":
            return await fetch_from_arch_linux({
                package: "perl",
                query: async (version) => { return { [cfg_linux("VERSION")]: version } }
            });
        case "curl":
            return await fetch_from_arch_linux({
                package: "curl",
                query: async (version) => { return { [cfg_linux("VERSION")]: version } }
            });
        case "wget": {
            return {
                ...await fetch_from_arch_linux({
                    package: "wget",
                    query: async (version) => { return { [cfg_linux("VERSION")]: version } }
                }),
                ...await fetch_from_github_release({
                    repo: get_metadata(meta, pkg, `'cfg(windows)'.REPO`),
                    query: async (repo, tag, artifacts) => {
                        const artifact = artifacts.find(x => x.startsWith("wget-ucrt64-xp-openssl-lite"));
                        if (!artifact) { throw new Error("could not find wget artifact for windows"); }
                        const vparts = tag.split('.');
                        if (vparts.length < 3) { throw new Error("invalid wget tag format: "+tag); }
                        const version = strip_v(vparts[0] + "." + vparts[1] + "." + vparts[2]);
                        const url = github_release_url(repo, tag, artifact);
                        const sha = await sha256_from_url(url);
                        return {
                            [cfg_windows("VERSION")]: version,
                            [cfg_windows("URL")]: url,
                            [cfg_windows("SHA")]: sha,
                        };
                    }
                })
            };
        }
        case "fzf": {
            return await fetch_from_github_release({
                repo: get_metadata(meta, pkg, "REPO"),
                query: async (repo, tag, artifacts) => {
                    tag = strip_v(tag);
                    const arm64_zip = `fzf-${tag}-windows_arm64.zip`; if (!artifacts.includes(arm64_zip)) { throw new Error("failed to find fzf arm64 artifact"); }
                    const x64_zip = `fzf-${tag}-windows_amd64.zip`; if (!artifacts.includes(x64_zip)) { throw new Error("failed to find fzf x64 artifact"); }
                    const arm64_url = github_release_url(repo, 'v'+tag, arm64_zip);
                    const x64_url = github_release_url(repo, 'v'+tag, x64_zip);
                    const arm64_hash = await sha256_from_url(arm64_url);
                    const x64_hash = await sha256_from_url(x64_url);
                    return {
                        VERSION: tag,
                        [cfg_windows_arm64("SHA")]: arm64_hash,
                        [cfg_windows_x64("SHA")]: x64_hash,
                    };
                }
            });
        }
        case "jq": {
            return await fetch_from_github_release({
                repo: get_metadata(meta, pkg, "REPO"),
                query: async (repo, tag, artifacts) => {
                    if (!tag.startsWith("jq-")) { throw new Error("invalid jq tag format: "+tag); }
                    const artifact = "jq-windows-amd64.exe"; if (!artifacts.includes(artifact)) { throw new Error("failed to find jq artifact"); }
                    const url = github_release_url(repo, tag, artifact);
                    const sha = await sha256_from_url(url);
                    return {
                        VERSION: tag.substring(3),
                        [cfg_windows("SHA")]: sha,
                    };
                }
            });
        }
        case "task": {
            return await fetch_from_github_release({
                repo: get_metadata(meta, pkg, "REPO"),
                query: async (repo, tag, artifacts) => {
                    tag = strip_v(tag);
                    const artifact_arm64 = "task_windows_arm64.zip"; if (!artifacts.includes(artifact_arm64)) { throw new Error("failed to find task arm64 artifact"); }
                    const artifact_x64 = "task_windows_amd64.zip"; if (!artifacts.includes(artifact_x64)) { throw new Error("failed to find task x64 artifact"); }
                    const artifact_linux = "task_linux_amd64.tar.gz"; if (!artifacts.includes(artifact_linux)) { throw new Error("failed to find task linux artifact"); }
                    const url_arm64 = github_release_url(repo, 'v'+tag, artifact_arm64);
                    const url_x64 = github_release_url(repo, 'v'+tag, artifact_x64);
                    const url_linux = github_release_url(repo, 'v'+tag, artifact_linux);
                    const sha_arm64 = await sha256_from_url(url_arm64);
                    const sha_x64 = await sha256_from_url(url_x64);
                    const sha_linux = await sha256_from_url(url_linux);
                    return {
                        VERSION: tag,
                        [cfg_windows_arm64("SHA")]: sha_arm64,
                        [cfg_windows_x64("SHA")]: sha_x64,
                        [cfg_linux("SHA")]: sha_linux,
                    };
                }
            });
        }
        case "bat": return await fetch_from_cratesio({ crate: "bat" });
        case "dust": return await fetch_from_cratesio({ crate: "du-dust" });
        case "fd": return await fetch_from_cratesio({ crate: "fd-find" });
        case "websocat": return await fetch_from_cratesio({ crate: "websocat" });
        case "zoxide": return await fetch_from_cratesio({ crate: "zoxide" });
        case "hack_font": {
            return {
                ...await fetch_from_arch_linux({
                    package: "ttf-hack-nerd",
                    query: (ver,rel) => ({VERSION_PACMAN:`${ver}-${rel}`})
                }),
                ...await fetch_from_github_release({
                    repo: get_metadata(meta, pkg, "REPO"),
                    query: async (repo, tag, artifacts) => {
                        tag = strip_v(tag);
                        const artifact = "Hack.zip";
                        if (!artifacts.includes(artifact)) { throw new Error("failed to find Hack.zip"); }
                        const url = github_release_url(repo, 'v'+tag, artifact);
                        const sha = await sha256_from_url(url);
                        return {
                            VERSION: tag,
                            SHA: sha,
                        };
                    }
                }),
            }
        }

        default:
            console.log(`WARNING: unknown package '${pkg}'`);
            return {};
    }
};

// === STRATEGY HELPERS ======================================================

type GithubReleaseArgs = {
    repo: string;
    tag?: (tags: string[]) => string;
    query: (repo: string, tag: string, artifacts: string[]) => Promise<PackageEntry> | PackageEntry;
};
type GithubTagsResponse = { name: string }[];
type GithubReleaseResponse = { tag_name: string; assets: { name: string }[] };
type GithubRepoResponse = { default_branch: string };
type GithubBranchResponse = { commit: { sha: string } };

/** Fetch package updates from GitHub releases */
const fetch_from_github_release = async ({ repo, tag: tag_picker, query }: GithubReleaseArgs): Promise<PackageEntry> => {
    console.log(`fetching from github repo: ${repo}`);
    const repo_path = parse_github_repo(repo);

    let selected_tag: string;

    if (tag_picker) {
        // fetch all tags and let the picker choose
        const tags_response = await fetch(`${GITHUB_API}repos/${repo_path}/tags`);
        if (!tags_response.ok) {
            throw new Error(`failed to fetch tags for ${repo_path}: ${tags_response.status}`);
        }
        const tags_data = await tags_response.json() as GithubTagsResponse;
        const tags = tags_data.map(t => t.name);
        if (tags.length === 0) {
            throw new Error(`no tags found for ${repo_path}`);
        }
        selected_tag = tag_picker(tags);
    } else {
        // fetch latest release
        const release_response = await fetch(`${GITHUB_API}repos/${repo_path}/releases/latest`);
        if (!release_response.ok) {
            throw new Error(`failed to fetch latest release for ${repo_path}: ${release_response.status}`);
        }
        const release_data = await release_response.json() as GithubReleaseResponse;
        selected_tag = release_data.tag_name;
    }
    console.log(`fetching release '${selected_tag}' from ${repo}`);

    // fetch the release for this tag to get artifacts
    const release_response = await fetch(`${GITHUB_API}repos/${repo_path}/releases/tags/${selected_tag}`);
    if (!release_response.ok) {
        throw new Error(`failed to fetch release ${selected_tag} for ${repo_path}: ${release_response.status}`);
    }
    const release_data = await release_response.json() as GithubReleaseResponse;
    const artifacts = release_data.assets.map(a => a.name);

    return await query(repo, selected_tag, artifacts);
};

/** Generate a GitHub release download URL */
const github_release_url = (repo: string, tag: string, artifact: string): string => {
    return `${repo}/releases/download/${tag}/${artifact}`;
};

/** Fetch the latest commit hash of the default branch */
const fetch_latest_commit = async (repo: string): Promise<string> => {
    const repo_path = parse_github_repo(repo);
    console.log(`fetching latest commit for ${repo_path}`);
    // get repo info to find default branch
    const repo_response = await fetch(`${GITHUB_API}repos/${repo_path}`);
    if (!repo_response.ok) {
        throw new Error(`failed to fetch repo info for ${repo_path}: ${repo_response.status}`);
    }
    const repo_data = await repo_response.json() as GithubRepoResponse;
    const default_branch = repo_data.default_branch;
    // get the latest commit on the default branch
    const branch_response = await fetch(`${GITHUB_API}repos/${repo_path}/branches/${default_branch}`);
    if (!branch_response.ok) {
        throw new Error(`failed to fetch branch ${default_branch} for ${repo_path}: ${branch_response.status}`);
    }
    const branch_data = await branch_response.json() as GithubBranchResponse;
    const commit = branch_data.commit.sha;
    console.log(`latest commit of ${repo_path} on ${default_branch}: ${commit}`);
    return commit;
};

type GithubCargoTomlArgs = {
    repo: string;
    ref: string;
    path: string;
    query?: (version: string) => Promise<PackageEntry> | PackageEntry;
};
/** Fetch package version from a Cargo.toml file on GitHub */
const fetch_from_cargo_toml = async ({ repo, ref, path, query }: GithubCargoTomlArgs): Promise<PackageEntry> => {
    query = query || ((VERSION) => ({ VERSION }));
    console.log(`fetching Cargo.toml from ${repo} @ ${ref}: ${path}`);
    const repo_path = parse_github_repo(repo);

    // fetch raw file content
    const raw_url = `https://raw.githubusercontent.com/${repo_path}/${ref}/${path}`;
    const response = await fetch(raw_url);
    if (!response.ok) {
        throw new Error(`failed to fetch ${raw_url}: ${response.status}`);
    }
    const content = await response.text();

    // parse package.version from Cargo.toml
    const version = parse_cargo_version(content);
    console.log(`parsed version from ${path}: ${version}`);

    return await query(version);
};

/** Parse the package.version from Cargo.toml content */
const parse_cargo_version = (content: string): string => {
    const lines = content.split("\n");
    let in_package_section = false;

    for (const line of lines) {
        const trimmed = line.trim();

        // check for section headers
        if (trimmed.startsWith("[")) {
            in_package_section = trimmed === "[package]";
            continue;
        }

        // look for version key in [package] section
        if (in_package_section && trimmed.startsWith("version")) {
            const match = trimmed.match(/^version\s*=\s*(.+)$/);
            if (match) {
                return from_toml_string(match[1].trim());
            }
        }
    }

    throw new Error("failed to find package.version in Cargo.toml");
};

type CratesIoArgs = {
    crate: string;
    query?: (version: string) => Promise<PackageEntry> | PackageEntry;
};
type CratesIoResponse = { crate: { newest_version: string } };
/** Fetch package updates from crates.io */
const fetch_from_cratesio = async ({ crate, query }: CratesIoArgs): Promise<PackageEntry> => {
    query = query || ((VERSION) => ({ VERSION }));
    console.log(`fetching from crates.io: ${crate}`);
    const response = await fetch(`${CRATESIO_API}crates/${crate}`, {
        headers: { "User-Agent": "shaft-registry-updater" } // crates.io requires UA
    });
    if (!response.ok) { throw new Error(`failed to fetch crate ${crate}: ${response.status}`); }
    const data = await response.json() as CratesIoResponse;
    const version = data.crate.newest_version;
    console.log(`latest version of ${crate}: ${version}`);
    return await query(version);
};

type ArchLinuxArgs = {
    package: string;
    query: (pkgver: string, pkgrel: string) => Promise<PackageEntry> | PackageEntry;
};
type ArchLinuxResponse = { results: { pkgname: string; pkgver: string; pkgrel: string }[] };
/** Fetch package version from Arch Linux official repositories */
const fetch_from_arch_linux = async ({ package: pkg, query }: ArchLinuxArgs): Promise<PackageEntry> => {
    console.log(`fetching from arch linux: ${pkg}`);
    const response = await fetch(`${ARCHLINUX_API}?name=${encodeURIComponent(pkg)}`);
    if (!response.ok) { throw new Error(`failed to fetch arch linux package ${pkg}: ${response.status}`); }
    const data = await response.json() as ArchLinuxResponse;
    const match = data.results.find(r => r.pkgname === pkg);
    if (!match) { throw new Error(`arch linux package not found: ${pkg}`); }
    const pkgver = match.pkgver;
    const pkgrel = match.pkgrel;
    console.log(`latest version of ${pkg} on arch linux: ${pkgver}-${pkgrel}`);
    return await query(pkgver, pkgrel);
};

// === CORE HELPERS ==========================================================

/** Extract owner/repo path from a GitHub URL */
const parse_github_repo = (repo: string): string => {
    const match = repo.match(/github\.com\/([^/]+\/[^/]+)/);
    if (!match) {
        throw new Error(`invalid github repo url: ${repo}`);
    }
    return match[1].replace(/\.git$/, "");
};

/** Strip leading 'v' from a version tag */
const strip_v = (version: string): string => {
    return version[0]==="v" ? version.substring(1) : version;
};

/** Download a file and compute its SHA256 hash */
const sha256_from_url = async (url: string): Promise<string> => {
    console.log(`hashing ${url}`);
    // ensure temp directory exists
    if (!fs.existsSync(TEMP_DIR)) {
        fs.mkdirSync(TEMP_DIR, { recursive: true });
    }

    // extract filename from URL
    const filename = path.basename(new URL(url).pathname);
    const filepath = path.join(TEMP_DIR, filename);

    // download the file
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error(`failed to download ${url}: ${response.status}`);
    }
    const buffer = Buffer.from(await response.arrayBuffer());
    fs.writeFileSync(filepath, buffer);
    const output = execSync(`sha256sum "${filepath}"`, { encoding: "utf-8" });
    let hash = output.split(/\s+/)[0];
    if (hash.startsWith('\\')) { hash = hash.substring(1); }
    fs.unlinkSync(filepath);
    return hash;
};

/** Merge multiple PackageUpdate objects into one */
const merge= (...updates: PackageUpdate[]): PackageUpdate => {
    const result: PackageUpdate = {};
    for (const update of updates) {
        for (const pkg in update) {
            result[pkg] = { ...result[pkg], ...update[pkg] };
        }
    }
    return result;
};

/** Update the metadata content using the update object */
const update_metadata = (
    metadata_content: string[],
    update_object: PackageUpdate
): boolean => {
    let updated = false;
    for (const package_name in update_object) {
        updated = update_metadata_core(metadata_content, package_name, update_object[package_name]) || updated;
    }
    return updated;
};

const update_metadata_core = (
    metadata_content: string[],
    package_name: string,
    update_mapping: PackageEntry,
): boolean => {
    const { start, end } = find_section_bounds(metadata_content, package_name);
    let updated = false;
    for (const key in update_mapping) {
        const new_value = update_mapping[key];
        const line_index = find_key_line(metadata_content, start, end, key);
        if (line_index === -1) {
            throw new Error(`key '${key}' not found in package '${package_name}'`);
        }
        const old_line = metadata_content[line_index].trim();
        const old_toml = old_line.slice(key.length).trimStart().slice(1).trim();
        const old_value = from_toml_string(old_toml);
        if (old_value === new_value) {
            continue;
        }
        const new_toml = to_toml_string(new_value);
        console.log(`update: [${package_name}] ${key} = ${old_toml} -> ${new_toml}`);
        metadata_content[line_index] = `${key} = ${new_toml}`;
        updated = true;
    }
    return updated;
};

/** Get a metadata value for a package, unescaping the TOML string */
const get_metadata = (
    metadata_content: string[],
    package_name: string,
    key: string
): string => {
    const { start, end } = find_section_bounds(metadata_content, package_name);
    const line_index = find_key_line(metadata_content, start, end, key);
    if (line_index === -1) {
        throw new Error(`key '${key}' not found in package '${package_name}'`);
    }
    const trimmed = metadata_content[line_index].trim();
    const value_part = trimmed.slice(key.length).trimStart().slice(1).trim();
    return from_toml_string(value_part);
};

/** Find the bounds of a package section in metadata */
const find_section_bounds = (
    metadata_content: string[],
    package_name: string
): { start: number; end: number } => {
    const section_header = `[${package_name}]`;
    const start = metadata_content.findIndex(
        (line) => line.trim() === section_header
    );
    if (start === -1) {
        throw new Error(`package '${package_name}' not found in metadata`);
    }

    let end = metadata_content.length;
    for (let i = start + 1; i < metadata_content.length; i++) {
        const trimmed = metadata_content[i].trim();
        if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
            end = i;
            break;
        }
    }

    return { start, end };
};

/** Find a key's line index within section bounds, returns -1 if not found */
const find_key_line = (
    metadata_content: string[],
    start: number,
    end: number,
    key: string
): number => {
    for (let i = start + 1; i < end; i++) {
        const trimmed = metadata_content[i].trim();
        if (trimmed === "" || trimmed.startsWith("#")) {
            continue;
        }
        if (trimmed.startsWith(key)) {
            const rest = trimmed.slice(key.length).trimStart();
            if (rest.startsWith("=")) {
                return i;
            }
        }
    }
    return -1;
};

/** Parse a TOML string value, unescaping as needed */
const from_toml_string = (toml_value: string): string => {
    // triple-quoted literal string '''...'''
    if (toml_value.startsWith("'''") && toml_value.endsWith("'''")) {
        return toml_value.slice(3, -3);
    }
    // single-quoted literal string '...' (no escaping)
    if (toml_value.startsWith("'") && toml_value.endsWith("'")) {
        return toml_value.slice(1, -1);
    }
    // double-quoted basic string "..." (with escape sequences)
    if (toml_value.startsWith('"') && toml_value.endsWith('"')) {
        const inner = toml_value.slice(1, -1);
        return inner
            .replace(/\\n/g, "\n")
            .replace(/\\r/g, "\r")
            .replace(/\\t/g, "\t")
            .replace(/\\\\/g, "\\")
            .replace(/\\"/g, '"');
    }
    // unquoted value (shouldn't happen for strings, but handle gracefully)
    return toml_value;
};

/** Format a string as a TOML value, using raw literal if escaping would be needed */
const to_toml_string = (str: string): string => {
    const needs_escaping = str.includes("\\") || str.includes('"') || str.includes("\n") || str.includes("\r") || str.includes("\t");
    if (!needs_escaping) {
        return `"${str}"`;
    }
    if (!str.includes("'")) {
        return `'${str}'`;
    }
    if (!str.includes("'''")) {
        return `'''${str}'''`;
    }
    throw new Error("why does the input have triple single quote");
};


/** Get list of package names from metadata content */
const get_package_names = (metadata_content: string[]): string[] => {
    const names: string[] = [];
    for (const line of metadata_content) {
        const trimmed = line.trim();
        if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
            names.push(trimmed.slice(1, -1));
        }
    }
    return names;
};

/** Read metadata.toml and return the lines (with line endings stripped) */
const load_metadata = (): string[] => {
    const content = fs.readFileSync(METADATA_TOML, "utf-8");
    return content.split("\n").map(x => x.trimEnd());
};

/** Save new content to metadata.toml with unix line endings */
const save_metadata = (metadata_content: string[]): void => {
    const content = metadata_content.join("\n");
    fs.writeFileSync(METADATA_TOML, content, "utf-8");
};

void main();
