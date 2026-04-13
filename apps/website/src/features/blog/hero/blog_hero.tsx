"use client";
import React, { type ReactNode } from "react";

interface BlogHeroProps {
    /**
     * THe url of the image.
     */
    img: string;
    alt: string;
}

export const BlogHero = ({ img, alt }: BlogHeroProps): ReactNode => {
    return (
        <div className="w-full max-w-[1080px] max-h-[min(400px,100vh)] rounded-lg">
            <img
                className="w-full h-full object-cover rounded-lg"
                src={img}
                alt={alt}
            />
        </div>
    );
};

BlogHero.displayName = "BlogHero";
