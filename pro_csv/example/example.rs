fn main() {
    let mut csv = pro_csv::CSV::default();
    csv.load("a;b;c;d\na;b;c;d;d;d;d;d;d;d;\n"); // csv.load_from_file("demo.csv") exsits

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
        "\n# Line content\n\n{:?}",
        csv.get_all_element_of_line(csv.find_line_index_with_name("a").unwrap())
    );

    println!("\n# Get at specific location\n\n{:?}", csv.get(1, 1));
    println!(
        "\n# Get at specific location with headers\n\n{:?}",
        csv.get_with_headers(0, 0)
    );
    println!("\n# Get line count\n\n{:?}", csv.get_line_count());
    println!("\n# Get column count\n\n{:?}", csv.get_column_count());

    // Insert

    println!("\n# Insert line");
    csv.insert_line(0, vec![String::from("insert_line")]);
    println!("\n# Insert column");
    csv.insert_column(0, String::from("insert_column"));

    // Append

    println!("\n# Append line");
    csv.append_line(vec![String::from("append_line")]);
    println!("\n# Append column");
    csv.append_column(String::from("append_column"));

    println!("\n# Correct the size");
    csv.correct_size();

    // Set

    println!("\n# Set an element");
    // Tips: You can set an element out of the current table
    // it will append lines/columns until it have the good size
    csv.set(0, 0, String::from("SET"));

    // Swap

    println!("\n# Swap two lines");
    csv.swap_lines(0, 1).unwrap();

    println!("\n# Swap two columns");
    csv.swap_columns(0, 1).unwrap();

    // Save/Write to file

    csv.save("demo.csv").unwrap();
}
