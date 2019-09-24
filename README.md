# scopefunc - use kotlin-like scope functions

Add trait defining kotlin-like scope functions.

Currently provided functions are:

-   `transform()`: transform value to another.

    ```rust
    let v = gen_rand(0, 10).transform(|v| 'x'.repeat(v));
    // => `v` is random length of "xx...".
    ```

-   `modify()`: modify the value and return.

    ```rust
    let v = vec![1, 2, 3].modify(|v| v.push(4));
    // => `v` is [1, 2, 3, 4].
    ```
