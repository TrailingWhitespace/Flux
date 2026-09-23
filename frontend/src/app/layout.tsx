import type { Metadata, Viewport } from "next";
import "./globals.css";
import { Inter, Jersey_15, Manrope } from "next/font/google";
import { QueryProvider } from "@/lib/providers/query-provider";
import { Sidebar } from "@/components/Sidebar/sidebar";

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

const fontTitle = Jersey_15({
  weight: "400",
  variable: "--font-title",
  subsets: ["latin"],
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
    <html
      lang="en"
      className={`${fontBody.variable} ${fontDisplay.variable} ${fontTitle.variable}`}
    >
      <body>
        <QueryProvider>
          <div className="app-shell">
            <Sidebar />
            <main className="app-content">{children}</main>
          </div>
        </QueryProvider>
      </body>
    </html>
  );
}
