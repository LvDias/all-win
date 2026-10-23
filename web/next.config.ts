import type { NextConfig } from "next";
import createNextIntlPlugin from "next-intl/plugin";

const withNextIntl = createNextIntlPlugin("./src/shared/i18n/request.ts");

// The dev-only /api rewrite must never reach a production build: production routes /api at the reverse proxy.
if (process.env.NODE_ENV === "production" && process.env.ALLWIN_CORE_URL) {
  throw new Error("ALLWIN_CORE_URL is for local development only; unset it for production builds.");
}

const nextConfig: NextConfig = {
  output: "standalone",
  poweredByHeader: false,
  // Local development only: forwards API calls to a Core running elsewhere. Production routes /api at the reverse proxy.
  async rewrites() {
    const core = process.env.ALLWIN_CORE_URL;
    return core ? [{ source: "/api/:path*", destination: `${core}/api/:path*` }] : [];
  },
};

export default withNextIntl(nextConfig);
