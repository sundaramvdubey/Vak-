# Vāk learning site

This is a zero-dependency static site for learning the Vāk language and understanding its compiler. It is intentionally plain HTML and CSS so it can be hosted directly on Cloudflare Pages, GitHub Pages, or any static web server.

## Local preview

From the repository root:

```sh
python3 -m http.server 8080 --directory docs-site
```

Open `http://localhost:8080`.

## Cloudflare Pages

Create a Pages project connected to this repository. Set the **root directory** to `docs-site`, leave the build command empty, and set the output directory to `.`. The site has no build step or runtime dependency.

The links intentionally point to the repository's Markdown specification and status documents. If the site is deployed from a separate artifact repository, copy those documents alongside `docs-site` or update the footer links.

## Design notes

The visual language combines an editor-like monospace layer with a warm paper ground, saffron accent, deep green, and restrained Devanagari reference. The site is responsive and uses no JavaScript, image assets, or client-side framework.
