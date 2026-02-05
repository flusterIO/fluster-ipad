import { staticContent } from "#/core/static_content";
import { z } from "zod";

const projectDataSchema = z.object({
    title: z.string().max(30),
    desc: z.string().max(120),
    repo: z.string().url(),
    hostedSite: z.string().url().optional(),
    screenShots: z
        .string()
        .array()
        .default([])
        .describe("filepaths relative to the public/assets/resume directory"),
});

const workHistoryItemSchema = z.object({
    title: z.string().max(50),
    desc: z.string().max(500),
    start: z.coerce.date(),
    end: z.coerce.date().optional(),
    location: z.string(),
});

const skillsSchema = z.record(
    z.string(),
    z.union([
        z.string().array(),
        z.record(z.string(), z.number().min(0).max(10)),
    ]),
);

const resumeDataSchema = z.object({
    intro: z.string().max(1000),
    github: z.string().url().default("https://github.com/flusterIO"),
    email: z.string().email().default("andrewMueller414@gmail.com"),
    homepage: z.string().url().default("https://flusterapp.com"),
    linkedIn: z.string().url().optional(),
    dob: z.string().default("4/19/1988"),
    avatar: z
        .string()
        .describe("Path to avatar relative to public/assets/resume directory")
        .default("avatar.png"),
    personalInterests: z.string().array(),
    projects: projectDataSchema.array(),
    projectsDesc: z.string().optional(),
    workHistory: workHistoryItemSchema.array(),
    skills: skillsSchema,
});

export type ResumeData = z.output<typeof resumeDataSchema>;

export type ResumeDataInput = z.input<typeof resumeDataSchema>;

const resumeData: ResumeDataInput = {
    intro:
        'My name is Andrew. I\'m a 37 year old developer with more than a dozen years of experience across more than 5 languages and ecosystems. You can find my most recent work, Fluster, by visiting the home page at <a href="/">flusterapp.com</a>. I am happy to provide links to my recent freelance work to prospective employers.',
    skills: {
        "Frameworks, APIs and Libraries": {
            React: 10,
            MongoDB: 10,
            Prisma: 10,
            Express: 10,
            "Next.js": 10,
            "Google Analytics": 9,
            GraphQL: 6,
            Postgres: 8.5,
            Mongoose: 9,
            "Node.js": 10,
            "Google Cloud": 8,
            AWS: 7.5,
        },
        Languages: {
            Typescript: 10,
            "Css & Sass": 10,
            Python: 9.5,
            Rust: 9,
            Golang: 8,
            Swift: 8,
            Lua: 8,
            "Bash/Zsh": 8,
            Dart: 6.5,
        },
        "Other Tools": [
            "Git",
            "Framer Motion & Gsap",
            "Three.js",
            "Socket.io",
            "OAuth",
            "Redux",
            "Webpack & Babel",
            "Jupyter",
            "Various Data Viz Libraries",
            "Jest, Vitest, Postman testing",
        ],
    },
    personalInterests: [
        "Physics & Cosmology",
        "Ancient History",
        "Cooking",
        "Reading Greek mythology to my puppy",
        "Math... <span class='italic'>I know</span>",
    ],
    projectsDesc:
        "I built Fluster partially to be an example of my recent work, but I can also provide links to recent freelance projects on request.",
    projects: [
        {
            title: "Fluster",
            desc: "An open source academic note taking framework",
            repo: staticContent.links.social.github,
            hostedSite: "https://flusterapp.com",
        },
    ],
    workHistory: [
        {
            title: "Fluster",
            desc: "After returning to freelance work, I decided to convert a personal application into a free & open source project. While Fluster was never my full-time job, it is the one project that I continue to return to, and something I plan to continue working on for the forseable future.",
            location: "Remote",
            start: new Date("03/01/2022"),
        },
        {
            title: "Direct Mailers Gorup",
            desc: "Created websites to build a subscriber database for various online newsletters. Was also responsible for html email templates, an accessible sign-up and cancellation user experience and building a portal for newsletter authors.",
            location: "Milwaukee",
            start: new Date("1/1/2019"),
            end: new Date("03/01/2022"),
        },
        {
            title: "Full-time freelance developer",
            desc: "After building a strong enough reputation, and feeling confident enough in my own abilities I took the leap and decided to work as a full-time freelance developer.",
            location: "Milwaukee & Remote",
            start: new Date("07/1/2015"),
            end: new Date("01/01/2019"),
        },
    ],
};

export const getResumeData = (): ResumeData => {
    return resumeDataSchema.parse(resumeData);
};
