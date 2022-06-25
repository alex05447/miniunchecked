# miniunchecked

A crate with some utility methods for debug unchecked operations on options, results and slices.

A middle ground between someting like calling `Option::unwrap()` / `Result::unwrap()` / slice square brackets indexing operator
(which always panic if `None` / `Err` / out of bounds)
and `unsafe { Option::unwrap_unchecked() }` / `unsafe { Result::unwrap_unchecked() }` / `unsafe { [T]::get_unchecked(...) }`)
(which never panic and lead to UB when `None` / `Err` / out of bounds) - an operation which does unsafe access in release configuration,
but also panics on `None` / `Err` / out of bounds index in debug configuration.
