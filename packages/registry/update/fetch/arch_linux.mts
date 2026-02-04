import type { MetaKeyValues } from "../metafile.mts";
import { ARCHLINUX_API, fetch_with_retry } from "../util.mts";

export type ArchLinuxArgs = {
    package: string;
    query: (pkgver: string, pkgrel: string) => Promise<MetaKeyValues> | MetaKeyValues;
};
type ArchLinuxResponse = { results: { pkgname: string; pkgver: string; pkgrel: string }[] };
/** Fetch package version from Arch Linux official repositories */
export const fetch_from_arch_linux = async ({ package: pkg, query }: ArchLinuxArgs): Promise<MetaKeyValues> => {
    console.log(`-- fetching from arch linux: ${pkg}`);
    const response = await fetch_with_retry(`${ARCHLINUX_API}?name=${encodeURIComponent(pkg)}`);
    if (!response.ok) { throw new Error(`failed to fetch arch linux package ${pkg}: ${response.status}`); }
    const data = await response.json() as ArchLinuxResponse;
    const match = data.results.find(r => r.pkgname === pkg);
    if (!match) { throw new Error(`arch linux package not found: ${pkg}`); }
    const pkgver = match.pkgver;
    const pkgrel = match.pkgrel;
    console.log(`-- -- latest version of ${pkg} on arch linux: ${pkgver}-${pkgrel}`);
    return await query(pkgver, pkgrel);
};
