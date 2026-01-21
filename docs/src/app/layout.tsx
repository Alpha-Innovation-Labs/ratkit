import { RootProvider } from 'fumadocs-ui/provider/next';
import './global.css';
import { JetBrains_Mono } from 'next/font/google';

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-mono',
});

export default function Layout({ children }: LayoutProps<'/'>) {
  return (
    <html lang="en" className={jetbrainsMono.variable} suppressHydrationWarning>
      <body className="flex flex-col min-h-screen font-mono">
        <RootProvider
          theme={{
            defaultTheme: 'dark',
            themes: ['light', 'dark'],
          }}
        >
          {children}
        </RootProvider>
      </body>
    </html>
  );
}
