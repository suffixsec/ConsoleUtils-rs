# Console Utilities

[![crates.io](https://img.shields.io/crates/v/console-utilities.svg)](https://crates.io/crates/console-utilities)
[![crates.io](https://img.shields.io/crates/d/console-utilities.svg)](https://crates.io/crates/console-utilities)
[![docs.rs](https://docs.rs/console-utilities/badge.svg)](https://docs.rs/console-utilities)

_A Rust crate for handling console input and output utilities._

## Usage

```toml
[dependencies]
console-utilities = "1"
```

Once you have the dependency you can import it in your project:

```rust
use console_utilities::{
    input::{
        input,
        input_rgb
    },
    output::{
        print_rgb,
        print_gradient
    }
};
```

## Examples

### User Input

```rust, no_run
use console_utilities::{
    input::{
        input,
        input_rgb,
        input_gradient
    },
    color::RGB
};

// Prompt for user input
let user_input: String = input("Enter your name: ");

// Prompt for user input (colored)
let colored_user_input: String = input_rgb("Enter your age: ", RGB::new(255, 0, 255));

// Prompt for user input (gradient)
let gradient_user_input: String = input_gradient("Enter your favourite color: ", RGB::new(255, 255, 255) /* start color */, RGB::new(2, 2, 2) /* end color */);

println!("Name: {}\n Age: {}\n Favourite Color: {}", user_input, colored_user_input, gradient_user_input);
```

### Console Output

```rust, no_run
use console_utilities::output::{print_rgb, print_gradient};
use console_utilities::color::RGB;

// Print text in a specific color
print_rgb("This is colored text!", RGB::new(0, 255, 0));

// Print text with a gradient
print_gradient("Gradient Text", RGB::new(255, 0, 0), RGB::new(0, 0, 255));
```

### Color Utilities

```rust, no_run
use console_utilities::color::{RGB, set_text_color, reset_text_color};

// Define a custom RGB variable
let color_variable: RGB = RGB::new(255, 0, 255);

// Set text color
set_text_color(RGB::new(255, 0, 0));

println!("This text is red!");
println!("This text is still red!");

// Reset text color
reset_text_color();
```

### Cursor Utilties

```rust, no_run
use console_utilities::cursor::{Visibility, Move};

// Show and hide cursor
Visibility::show_cursor();
Visibility::hide_cursor();

// Move cursor
Move::cursor_up(2);
Move::cursor_down(2);
Move::cursor_left(2);
Move::cursor_right(2);
```

### Console Utilties

```rust, no_run
use console_utilities::console::{flush_console, clear_line, clear_lines};

// Flush the console
flush_console();

// Clear the current line
clear_line();

// Clear multiple lines
clear_lines(3);
```