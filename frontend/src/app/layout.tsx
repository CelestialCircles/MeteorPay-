import type { Metadata } from "next";
import { Providers } from "@/components/providers";
import { getThemeBootstrapScript } from "@/lib/theme";
import "./globals.css";

export const metadata: Metadata = {
  title: "MeteorPay | Merchant Payments at QR Speed",
  description:
    "MeteorPay gives merchants instant QR-based crypto payments, real-time treasury visibility, and premium settlement tooling.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="h-full antialiased" suppressHydrationWarning>
      <head>
        <script
          dangerouslySetInnerHTML={{ __html: getThemeBootstrapScript() }}
        />
      </head>
      <body className="min-h-full font-sans text-foreground">
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
