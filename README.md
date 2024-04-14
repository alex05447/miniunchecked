# miniunchecked

A crate with some utility methods for debug unchecked operations on [`Option`](https://doc.rust-lang.org/stable/core/option/enum.Option.html), [`Result`](https://doc.rust-lang.org/stable/core/result/enum.Result.html)
and [`slice`](https://doc.rust-lang.org/std/primitive.slice.html) / [`str`](https://doc.rust-lang.org/std/primitive.str.html),
as well as a debug [`unreachable!`](https://doc.rust-lang.org/stable/core/macro.unreachable.html) alternative.

A middle ground between someting like
- calling [`Option::unwrap()`](https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.unwrap) / [`Result::unwrap()`](https://doc.rust-lang.org/stable/core/result/enum.Result.html#method.unwrap) / slice/string square brackets indexing operator, or using [`unreachable!`](https://doc.rust-lang.org/stable/core/macro.unreachable.html),
which always panic if [`None`](https://doc.rust-lang.org/stable/core/option/enum.Option.html#variant.None) / [`Err`](https://doc.rust-lang.org/stable/core/result/enum.Result.html#variant.Err) / out of bounds / reached, and
- unsafe [`Option::unwrap_unchecked()`](https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.unwrap_unchecked) [`Result::unwrap_unchecked()`](https://doc.rust-lang.org/stable/core/result/enum.Result.html#method.unwrap_unchecked) / [`[T]::get_unchecked()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked),
or using [`unreachable_unchecked()`](https://doc.rust-lang.org/stable/std/hint/fn.unreachable_unchecked.html),
which never panic and lead to UB when [`None`](https://doc.rust-lang.org/stable/core/option/enum.Option.html#variant.None) / [`Err`](https://doc.rust-lang.org/stable/core/result/enum.Result.html#variant.Err) / out of bounds,

an operation which does unsafe access in release configuration, for optimal codegen if the invariants are maintained by other means,
but also panics on [`None`](https://doc.rust-lang.org/stable/core/option/enum.Option.html#variant.None) / [`Err`](https://doc.rust-lang.org/stable/core/result/enum.Result.html#variant.Err) / out of bounds index in debug configuration / when running tests.