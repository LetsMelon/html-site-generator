# unreleased

## Added

## Breaking

- renamed `SetHtmlAttributes::set_class` to `SetHtmlAttributes::add_class`
- changed signature from `HtmlAttributesBuilder::class<S: Into<String>>(S)` to `HtmlAttributesBuilder::class<VS: Into<Vec<String>>>(VS)`

# 0.0.1

## Added

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
