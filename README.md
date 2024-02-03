# serde_nested_with

[![](https://img.shields.io/crates/v/serde_nested_with.svg)](https://crates.io/crates/serde_nested_with)
[![](https://docs.rs/serde_nested_with/badge.svg)](https://docs.rs/serde_nested_with)
[![.github/workflows/push.yml](https://github.com/murar8/serde_nested_with/actions/workflows/push.yml/badge.svg)](https://github.com/murar8/serde_nested_with/actions/workflows/push.yml)
[![.github/workflows/audit.yml](https://github.com/murar8/serde_nested_with/actions/workflows/audit.yml/badge.svg)](https://github.com/murar8/serde_nested_with/actions/workflows/audit.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a small procedural macro that allows you to use serde attributes with a nested module or function. This is useful when you want to use a custom (de)serializer that is defined in a different module or crate.

## Installation

```bash
cargo add serde_nested_with
```

## Example

```rust
mod example {
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use serde_nested_with::serde_nested;
    use std::collections::BTreeMap;
    use time::serde::rfc3339;
    use time::OffsetDateTime;

    #[serde_nested]
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Foo {
        #[serde_nested(sub = "OffsetDateTime", serde(with = "rfc3339"))]
        pub bar: Option<Option<OffsetDateTime>>,
        #[serde_nested(sub = "OffsetDateTime", serde(with = "rfc3339"))]
        pub bar5: Vec<(OffsetDateTime, OffsetDateTime)>,
    }
}
```

## Release process

When a [SemVer](https://semver.org/) compatible git tag is pushed to the repo a new version of the package will be published to [crates.io](https://crates.io/crates/serde_nested_with).

## Contributing

Direct push to the `main` branch is not allowed, any updates require a pull request to be opened. After all status checks pass the PR will be eligible for review and merge.

Commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) specification.

The project comes with an optional pre-configured development container with all the required tools. For more information on how to use it please refer to [containers.dev](https://containers.dev)

To make sure your changes match the project style you can install the pre-commit hooks with `pre-commit install`. This requires [pre-commit](https://pre-commit.com/) to be installed on your system.

## License

Copyright (c) 2024 Lorenzo Murarotto <lnzmrr@gmail.com>

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
