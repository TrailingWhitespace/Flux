import type { Metadata, Viewport } from "next";
import "./globals.css";
import { Inter, Manrope } from "next/font/google";

const fontBody = Inter({
  subsets: ["latin"],
  variable: "--font-body",
  display: "swap", // uses a fallback font until this one loads then swaps
});

const fontDisplay = Manrope({
  subsets: ["latin"],
  variable: "--font-display",
  display: "swap",
});

export const metadata: Metadata = {
  title: "Flux",
  description: "Idk",
};

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" className={`${fontBody.variable} ${fontDisplay.variable}`}>
      <body>{children}</body>
    </html>
  );
}
