# shmemfdrs2

Fork of [shmemfdrs](https://codeberg.org/valpackett/shmemfdrs).

Provides a single function

```rust
pub fn create_shmem<T: AsRef<CStr>>(name: T) -> io::Result<File>;
```

- On Linux `memfd_create` is used.
- On FreeBSD `shm_open` with `SHM_ANON` is used.
- Other platform use `shm_open` followed by `shm_unlink`.

## License

Licensed under the Apache License, Version 2.0 < LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0 > or the MIT license < LICENSE-MIT or https://opensource.org/licenses/MIT >, at your option.
