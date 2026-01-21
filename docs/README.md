# Ratatui Toolkit Documentation

This is the documentation site for Ratatui Toolkit, generated with
[Create Fumadocs](https://github.com/fuma-nama/fumadocs).

## Development

Run development server:

```bash
npm run dev
```

Open http://localhost:3000 with your browser to see the result.

## Build

Build for production:

```bash
npm run build
```

Start production server:

```bash
npm start
```

## Project Structure

In the project, you can see:

- `lib/source.ts`: Code for content source adapter, [`loader()`](https://fumadocs.dev/docs/headless/source-api) provides the interface to access your content.
- `lib/layout.shared.tsx`: Shared options for layouts, optional but preferred to keep.
- `content/docs/`: Documentation content written in MDX format.

| Route                     | Description                                            |
| ------------------------- | ------------------------------------------------------ |
| `app/(home)`              | The route group for your landing page and other pages. |
| `app/docs`                | The documentation layout and pages.                    |
| `app/api/search/route.ts` | The Route Handler for search.                          |

### Fumadocs MDX

A `source.config.ts` config file has been included, you can customise different options like frontmatter schema.

Read the [Introduction](https://fumadocs.dev/docs/mdx) for further details.

## Documentation Content

The documentation is organized as follows:

- **Home Page (`index.mdx`)**: Overview of Ratatui Toolkit and features
- **Components (`components.mdx`)**: Component overview and quick reference
- **Component Pages**: Detailed documentation for each component:
  - ResizableSplit
  - TreeView
  - FileSystemTree
  - Button
  - Dialog
  - Toast
  - HotkeyFooter
  - MenuBar
  - MarkdownRenderer
  - StatusLineStacked
  - TermTui

## Learn More

To learn more about Next.js and Fumadocs, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.
- [Fumadocs](https://fumadocs.dev) - learn about Fumadocs
