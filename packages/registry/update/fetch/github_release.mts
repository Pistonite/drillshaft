import type { MetaKeyValues } from "../metafile.mts";
import { GITHUB_API, parse_github_repo } from "../util.mts";

export type GithubReleaseArgs = {
    repo: string;
    tag?: (tags: string[]) => string;
    query: (repo: string, tag: string, artifacts: string[]) => Promise<MetaKeyValues> | MetaKeyValues;
};
type GithubTagsResponse = { name: string }[];
type GithubReleaseResponse = { tag_name: string; assets: { name: string }[] };

/** Fetch package updates from GitHub releases */
export const fetch_from_github_release = async ({ repo, tag: tag_picker, query }: GithubReleaseArgs): Promise<MetaKeyValues> => {
    console.log(`-- fetching from github repo: ${repo}`);
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
    console.log(`-- fetching release '${selected_tag}' from ${repo}`);

    // fetch the release for this tag to get artifacts
    const release_response = await fetch(`${GITHUB_API}repos/${repo_path}/releases/tags/${selected_tag}`);
    if (!release_response.ok) {
        throw new Error(`failed to fetch release ${selected_tag} for ${repo_path}: ${release_response.status}`);
    }
    const release_data = await release_response.json() as GithubReleaseResponse;
    const artifacts = release_data.assets.map(a => a.name);

    return await query(repo, selected_tag, artifacts);
};
