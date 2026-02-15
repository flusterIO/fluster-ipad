const lockedPackages = [];

// @ts-check
/** @type {import("syncpack").RcFile} */
const config = {
    sortPackages: true,
    formatRepository: true,
    sortFirst: [
        "name",
        "version",
        "homepage",
        "keywords",
        "funding",
        "author",
        "contributors",
        "type",
        "private",
        "publishConfig",
        "bin",
        "files",
        "scripts",
        "wireit",
        "types",
        "main",
        "exports",
        "engines",
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "prisma",
        "prettier",
        "xo",
        "ava",
        "packageManager",
        "ulld-pluginConfig",
        "pnpm",
        "release-it",
        "commitlint",
        "license",
        "dum-commit",
    ],
    dependencyTypes: ["prod", "dev", "peer"],
    semverGroups: [
        {
            label: "use exact version numbers in production",
            packages: ["**"],
            dependencyTypes: ["prod", "dev", "peer"],
            dependencies: ["**"],
            range: "",
        },
    ],
    customTypes: {
        engines: {
            path: "engines",
            strategy: "versionsByName",
        },
    },
    versionGroups: [
        {
            dependencyTypes: ["dev"],
            dependencies: ["@types/react-dom"],
            pinVersion: "19.2.2",
            label: "pin @types/react-dom",
        },
        {
            dependencyTypes: ["dev"],
            dependencies: ["@types/react"],
            pinVersion: "19.2.2",
            label: "pin @types/react",
        },
        {
            dependencies: ["react"],
            pinVersion: "19.2.0",
            label: "Pin react in packages.",
        },

        {
            dependencies: ["react-dom"],
            pinVersion: "19.2.0",
            label: "Pin react-dom in packages.",
        },
        {
            dependencies: ["@tanstack/react-table"],
            pinVersion: "8.21.3",
            label: "Pin react-table",
        },
        {
            dependencies: ["tailwindcss"],
            pinVersion: "3.4.18",
            packages: ["!@fluster/website"],
            label: "Pin tailwind version",
        },
        {
            dependencies: ["@lezer/common", "@lezer/go", "@lezer/markdown"],
            pinVersion: "^1.0.0",
            label: "Pin lezer to latest shared peer.",
            dependencyTypes: ["peer", "dev", "prod"],
        },
        ...lockedPackages.map((n) => {
            return {
                dependencies: [n.name],
                pinVersion: n.version,
                packages: ["!@fluster/website"],
                label: `Pin ${n.name}`,
            };
        }),
    ],
};

/* @ts-ignore */
module.exports = config;
