# Replace function body with `todo!()`

## Usage

```bash
todo-replace <file-path> <csv-path>
```

Where <csv-path> is a comma separated value file with each row consists of two columns: `function_name` and `body`.

- `function_name`: The name of the function to be replaced.
- `body`: The new body of the function. If empty, it will be replaced with default `{ todo!() }`.
  Note that `body` should begin and end with brackets `{}`, and after replacement, formatting may be off. You may need to run `cargo fmt` to fix it.

## Example

Rust File (input.rs):

```rust
fn foo() {
    println!(\"Old body\");
}

fn bar() {
    println!(\"Another body\");
}
```

CSV File (replacements.csv):

```bash
foo,{ println!(\"New body\"); }
bar,{ println!(\"Updated body\"); }
```

Command:

```bash
todo-replacer input.rs replacements.csv
```

Resulting Rust File (input.rs):

```rust
fn foo() { println!(\"New body\"); }

fn bar() { println!(\"Updated body\"); }
```
