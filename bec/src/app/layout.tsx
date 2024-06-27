import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import {Foot} from "../components/footer"
const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "BEC",
  description: "Biblioteca Estación Central",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="container mx-auto">
        {children}
        <Foot ></Foot>
        </div>
        
        </body>
    </html>
  );
}
