# Changelog for [`html-site-generator`](https://github.com/LetsMelon/html-site-generator)

## [unreleased](https://github.com/LetsMelon/html-site-generator/compare/0.0.1...main)

### Added

- error handling with `thiserror`

### Breaking

- renamed `SetHtmlAttributes::set_class` to `SetHtmlAttributes::add_class`
- renamed `SetHtmlAttributes::set_id` to `SetHtmlAttributes::add_id`
- changed signature from `HtmlAttributesBuilder::class<S: Into<String>>(S)` to `HtmlAttributesBuilder::class<VS: Into<Vec<String>>>(VS)` (pseudo rust code)
- removed struct `DateTime`

## [0.0.1](https://github.com/LetsMelon/html-site-generator/compare/b2a039bfe6070bf67435eb44af76597d0e706260...0.0.1)

### Added

- html elements:
  - address
  - body
  - div
  - document
  - footer
  - head
  - hyperlink
  - image
  - line_break
  - link
  - list
  - meta
  - paragraph
  - text
  - title
- struct `HtmlAttributes` with `id` and `class`
