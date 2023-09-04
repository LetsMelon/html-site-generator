# unreleased

## Added

## Breaking

- renamed `SetHtmlAttributes::set_class` to `SetHtmlAttributes::add_class`
- renamed `SetHtmlAttributes::set_id` to `SetHtmlAttributes::add_id`
- changed signature from `HtmlAttributesBuilder::class<S: Into<String>>(S)` to `HtmlAttributesBuilder::class<VS: Into<Vec<String>>>(VS)` (pseudo rust code)
- removed struct `DateTime`

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
