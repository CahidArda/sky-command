import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "SkyCommand",
  description: "3D flight simulator — v0.1.0",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="h-full bg-black text-white antialiased">{children}</body>
    </html>
  );
}
