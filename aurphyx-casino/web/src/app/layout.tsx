import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Aurphyx Casino',
  description: 'Provably Fair Multi-Chain Casino & Sportsbook',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}

