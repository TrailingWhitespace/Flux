import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactCompiler: false,
  allowedDevOrigins: process.env.LOCAL_DEV_IP ? [process.env.LOCAL_DEV_IP] : [],
};

export default nextConfig;
