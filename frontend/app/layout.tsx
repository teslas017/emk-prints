import type { Metadata } from "next";
import "./globals.css";
import "./brand.css";
import "./track.css";
import "./admin.css";
import "./shop.css";

export const metadata: Metadata = {
  title: "EMK PRINTS | Football Jerseys",
  description: "Home, away and third football kits from EMK PRINTS, available in sizes S to XL.",
  icons: {
    icon: "/favicon.svg",
    shortcut: "/favicon.svg",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className="antialiased">{children}</body>
    </html>
  );
}
