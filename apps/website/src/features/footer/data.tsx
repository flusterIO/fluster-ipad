import { type ReactNode } from "react";
import { staticContent } from "#/core/static_content";
import { type LinkProps } from "next/link";

export interface FooterBannerProps {
    content: ReactNode;
    className?: string;
}

export type FooterLinkKeys = "docs" | "demos" | "funding";

interface LinkGroupItem extends LinkProps {
    label: ReactNode;
    noLink?: boolean;
}

export type LinkGroupItems = LinkGroupItem[];

export const footerLinks: Record<FooterLinkKeys, LinkGroupItems> & {
    banners?: FooterBannerProps[];
} = {
    banners: [
        {
            content: (
                <div>
                    Logo by{" "}
                    <a
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-primary"
                        href="https://www.robstruble.com/"
                    >
                        Rob Strubble
                    </a>
                </div>
            ),
        },
    ],
    demos: [
        {
            href: "/blog/by_path/on_the_gravitational_nature_of_time",
            label: "On the gravitational nature of time",
        },
    ],
    docs: [
        {
            href: staticContent.links.docs.internal.userHome,
            label: "Users",
        },
        {
            href: staticContent.links.docs.internal.developerHome,
            label: "Developers",
        },
        {
            href: "/privacy",
            label: "Privacy",
        },
    ],
    funding: [
        {
            href: staticContent.links.fund.paypalDonate,
            label: "Paypal",
        },
        {
            href: staticContent.links.fund.patreon,
            label: "Patreon",
        },
        {
            href: staticContent.links.fund.github,
            label: "Github",
        },
    ],
};
