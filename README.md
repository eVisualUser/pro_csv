# pro_csv
Easy to use CSV read/write library.

## Example

### Code

```rust

let mut csv = pro_csv::CSV::default();
csv.load("a;b;c;d\na;b;c;d;d;d;d;d;d;d;\n");

println!("# Debug\n\n{:?}", csv);
println!("\n# Out\n\n{}", csv.to_string());

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
    "\n# Line content\n\n{:?}",
    csv.get_all_element_of_line(csv.find_line_index_with_name("a").unwrap())
);

println!("\n# Get at specific location\n\n{:?}", csv.get(0, 1));
println!("\n# Get line count\n\n{:?}", csv.get_line_count());
println!("\n# Get column count\n\n{:?}", csv.get_column_count());

println!("\n# Insert line\n");
csv.insert_line(0, vec![String::from("insert_line")]);
println!("\n# Insert column\n");
csv.insert_column(0, String::from("insert_column"));

println!("\n# Append line\n");
csv.append_line(vec![String::from("append_line")]);
println!("\n# Append column\n");
csv.append_column(String::from("append_column"));

println!("\n# Correct the size\n");
csv.correct_size();

println!("\n# Set an element\n");
// Tips: You can set an element out of the current table
// it will append lines/columns until it have the good size
csv.set(0, 0, String::from("SET"));

println!("\n# Reverse two lines\n");
csv.swap_lines(0, 1);

println!("\n# Reverse two columns\n");
csv.swap_columns(0, 1);

csv.save("demo.csv").unwrap();

```

### Terminal

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

### File "demo.csv"

```csv
a;insert_column;b;c;d;append_column;;;;;;;
insert_line;SET;;;;append_column;;;;;;;
a;insert_column;b;c;d;d;d;d;d;d;d;append_column;
;append_line;;;;append_column;;;;;;;
```
