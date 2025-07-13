# this fork of zola

- get site config in sass files with `$cfg : zola-config();` [(preliminary discussions)](https://github.com/getzola/zola/pull/2242#issuecomment-2184041602)
- ~~if `theme` is not set in `config.toml` and current site contains a `theme.toml`, reuse the `[extra]` field in it, avoids writing them again in `config.toml` when developing a theme~~
  - reverted because setting `theme = ".."` does the same hack lol
- zola as a library
  - new type `SplitPageContent` representing a page simply split into meta and content
  - reexporting page content with meta via `SplitPageContent::export_to_string()`
  - for ease of manipulating page content

---

# zola (né Gutenberg) <img src="docs/static/logos/Zola-logo-main-coffee.svg" align="right" alt="zola logo" width="30%"/>

[![Build Status](https://dev.azure.com/getzola/zola/_apis/build/status/getzola.zola?branchName=master)](https://dev.azure.com/getzola/zola/_build/latest?definitionId=1&branchName=master)
![GitHub all releases](https://img.shields.io/github/downloads/getzola/zola/total)

A fast static site generator in a single binary with everything built-in.

To find out more see the [Zola Documentation](https://www.getzola.org/documentation/getting-started/overview/), look
in the [docs/content](docs/content) folder of this repository or visit the [Zola community forum](https://zola.discourse.group).

This tool and its template engine [tera](https://keats.github.io/tera/) were born from an intense dislike of the (insane) Golang template engine and therefore of
Hugo that I was using before for 6+ sites.

# List of features

- [Single binary](https://www.getzola.org/documentation/getting-started/cli-usage/)
- [Syntax highlighting](https://www.getzola.org/documentation/content/syntax-highlighting/)
- [Sass compilation](https://www.getzola.org/documentation/content/sass/)
- Assets co-location
- [Multilingual site support](https://www.getzola.org/documentation/content/multilingual/) (Basic currently)
- [Image processing](https://www.getzola.org/documentation/content/image-processing/)
- [Themes](https://www.getzola.org/documentation/themes/overview/)
- [Shortcodes](https://www.getzola.org/documentation/content/shortcodes/)
- [Internal links](https://www.getzola.org/documentation/content/linking/)
- [External link checker](https://www.getzola.org/documentation/getting-started/cli-usage/#check)
- [Table of contents automatic generation](https://www.getzola.org/documentation/content/table-of-contents/)
- Automatic header anchors
- [Aliases](https://www.getzola.org/documentation/content/page/#front-matter)
- [Pagination](https://www.getzola.org/documentation/templates/pagination/)
- [Custom taxonomies](https://www.getzola.org/documentation/templates/taxonomies/)
- [Search with no servers or any third parties involved](https://www.getzola.org/documentation/content/search/)
- [Live reload](https://www.getzola.org/documentation/getting-started/cli-usage/#serve)
- Deploy on many platforms easily: [Netlify](https://www.getzola.org/documentation/deployment/netlify/), [Vercel](https://www.getzola.org/documentation/deployment/vercel/), [Cloudflare Pages](https://www.getzola.org/documentation/deployment/cloudflare-pages/), etc
