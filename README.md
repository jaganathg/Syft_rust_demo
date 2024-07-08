
Running application:

1. Run main application:
    $cargo run --bin main

2. Initialize database:
    $cargo run --bin init_db

3. To reinitialize database completely, even the existing tables get cleared.
    $cargo run --bin init_db -- --reset

3. Json to dataframe:
    $cargo run --bin json_df