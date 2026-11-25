import Footer from "#/features/footer";
import LandingPage from "#/features/landing_page/landing_page";
import { buildMetadata, seo } from "#/core/seo/site";

export const metadata = buildMetadata({
    title: seo.title,
    description: seo.description,
    path: "/",
});

export default function Home() {
    return (
        <>
            <LandingPage />
            <Footer />
        </>
    );
}
