# Pro-CSV

Easy to use CSV read/write library.

## List of features

- [X] Read at index
- [X] Set at index
- [X] Find line and column using headers
- [X] Insert and Append lines/columns
- [X] Remove lines/columns
- [X] Resize the table
- [X] Save to file

## Example

<details>
<summary>Code</summary>

```rust

fn main() {
    let mut csv = pro_csv::CSV::default();
    csv.load("a;b;c;d\na;b;c;d;d;d;d;d;d;d;\n"); // csv.load_from_file("demo.csv") exist

    // Debug & Output

    println!("# Debug\n\n{:?}", csv);
    println!("\n# Out\n\n{}", csv.to_string());

    // Reading

    println!(
        "\n# Column with name\n\n{:?}",
        csv.find_columns_index_with_name("a")
    );
    println!(
        "\n# Column content\n\n{:?}",
        csv.get_all_element_of_column(csv.find_column_index_with_name("a").unwrap())
    );
    println!(
        "\n# Line with name\n\n{:?}",
        csv.find_lines_index_with_name("a")
    );
    println!(
        "\n# Line that contain something\n\n{:?}",
        csv.find_lines_index_that_contains("a")
    );
    println!(
        "\n# Line content\n\n{:?}",
        csv.get_all_element_of_line(csv.find_line_index_with_name("a").unwrap())
    );
    println!(
        "\n# Line content using contains\n\n{:?}",
        csv.get_all_element_of_line(csv.find_line_index_that_contains("a").unwrap())
    );

    println!("\n# Get at specific location\n\n{:?}", csv.get(1, 1));
    println!(
        "\n# Get at specific location with headers\n\n{:?}",
        csv.get_with_headers(0, 0)
    );

    println!("\n# Get line count\n\n{:?}", csv.get_line_count());
    println!("\n# Get column count\n\n{:?}", csv.get_column_count());

    // Insert

    println!();

    println!("# Insert line");
    csv.insert_line(0, vec![String::from("insert_line")]);
    println!("# Insert column");
    csv.insert_column(0, String::from("insert_column"));

    // Append

    println!("# Append line");
    csv.append_line(vec![String::from("append_line")]);
    println!("# Append column");
    csv.append_column(String::from("append_column"));

    println!("# Correct the size");
    csv.correct_size();

    // Set

    println!("# Set an element");
    // Tips: You can set an element out of the current table
    // it will append lines/columns until it have the good size
    csv.set(0, 0, String::from("SET"));

    // Swap

    println!("# Swap two lines");
    csv.swap_lines(0, 1).unwrap();

    println!("# Swap two columns");
    csv.swap_columns(0, 1).unwrap();

    println!("# Remove line if exist");
    csv.remove_line(0);
    println!("# Remove column column if exist");
    csv.remove_column(0);

    // Save/Write to file

    csv.save("demo.csv").unwrap();
}

```
</details>

<details>
<summary>Terminal OutPut</summary>

```md
# Debug

CSV { separator: ';', buffer: [["a", "b", "c", "d"], ["a", "b", "c", "d", "d", "d", "d", "d", "d", "d"]] }

# Out

a;b;c;d;
a;b;c;d;d;d;d;d;d;d;


# Column with name

[0]

# Column content

["a", "a"]

# Line with name

[0, 1]

# Line content

["a", "b", "c", "d"]

# Get at specific location

Some("a")

# Get line count

2

# Get column count

4

# Insert line

# Insert column

# Append line

# Append column

# Correct the size

# Set an element

# Reverse two lines

# Reverse two columns

```
</details>

<details>
<summary>File OutPut</summary>

```csv
a;insert_column;b;c;d;append_column;;;;;;;
insert_line;SET;;;;append_column;;;;;;;
a;insert_column;b;c;d;d;d;d;d;d;d;append_column;
;append_line;;;;append_column;;;;;;;
```
</details>