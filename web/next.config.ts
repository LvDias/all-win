import type { NextConfig } from "next";
import createNextIntlPlugin from "next-intl/plugin";

const withNextIntl = createNextIntlPlugin("./src/shared/i18n/request.ts");

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
