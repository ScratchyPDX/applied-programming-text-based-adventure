# Overview

When learning a new programming language, I typically start with the obligatory “Hello World” project. From there, my projects usually fall into one of two categories: math calculations or game development. This particular project is the latter. Developed in Rust, it is a simple text-based game where the outcome hinges on the choices made by the player.

Since this project is designed to facilitate learning the language, the game is relatively simple, featuring four different outcomes. Only one true path leads the player to a successful conclusion. A decision tree document is available here: https://github.com/ScratchyPDX/applied-programming-text-based-adventure/blob/main/docs/text-based-story-game-flow.jpg

- The user is presented a story where the make choices.
- The users expected inputs are highlighted in green and evaluated to ensure the user's input is a valid command.
- Each choice leads to a specific outcome. Make enough correct choices and the user cn complete the story.
- The user may enter 'quit' and any prompt to end the game.

While not extensive, this project also includes a small example of how unit test are written and included ina Rust project.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

This project was written using Visual Studio Code v1.91.1 and Rust v 1.79.0. Rust is a modern systems programming language focusing on safety, speed, and concurrency. Rust's ownership model ensures memory safety and enables developers to write efficient, high-performance code without the risk of null pointers or buffer overflows. 

For this project, we utilized several Rust libraries (crates) to enhance functionality and simplify the development process:

[std::io](https://doc.rust-lang.org/std/io/): A part of Rust's standard library, used for input and output operations, including reading from stdin and writing to stdout. This library is crucial for interactive applications like text-based games.

[term_size](https://crates.io/crates/term_size): This crate is used to detect the terminal's size, allowing our application to adjust its output dynamically for a better user experience.

[textwrap](https://crates.io/crates/textwrap): A utility crate for wrapping text. In this game, it ensures that text output respects the terminal width, improving readability and user interaction.

[lazy_static](https://crates.io/crates/lazy_static): This crate provides a macro for declaring lazily evaluated statics in Rust. Unlike regular statics, `lazy_static` allows for statics that require code to be executed at runtime in order to be initialized. This is particularly useful for complex initialization that cannot be achieved with const functions or when initialization depends on runtime parameters. The `lazy_static` approach ensures that the initialization occurs exactly once and is thread-safe, making it ideal for creating global instances or configurations that are accessed from multiple threads within the application.

[`colored`](https://crates.io/crates/colored): The `colored` crate is a Rust library designed to simplify the process of coloring terminal text. It extends Rust's `String` and `&str` types with methods to set the color, background, and style (such as bold or underline) of text output to the terminal. This crate provides an easy-to-use, fluent interface that makes terminal output more visually distinctive and informative, enhancing user interaction and readability.

One of the key features of `colored` is its simplicity and ease of integration. Developers can add color and style to their text with minimal code changes, without the need to manually manage ANSI escape codes. For example, to make a string red, one simply calls `.red()` on it, like `"Error".red()`. This approach allows for quick and expressive customization of terminal output, making it particularly useful for CLI applications, logs, and any project aiming to improve its user interface with color.

The `colored` crate is also designed to be cross-platform, working on both Unix and Windows systems. This ensures that applications using `colored` can provide a consistent user experience across different environments. Additionally, it respects the terminal's color capabilities and user preferences, such as those set by the `NO_COLOR` environment variable, making it a respectful choice for developers looking to enhance their terminal applications.

These libraries and the Rust programming language's robust ecosystem allowed me to create this reliable and user-friendly text-based adventure game.

# Useful Websites

{Make a list of websites that you found helpful in this project}
- [Rust Tutorial](https://www.tutorialspoint.com/rust/index.htm)
- [First Steps with Cargo](https://doc.rust-lang.org/cargo/getting-started/first-steps.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust Unit Tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
 
# Future Work

- The game is pretty simple, so developing a larger story with more pitfalls and decision making is a must 
- Figure out how might make a reusable global struct that could then represent any object, reducing some of the repetitive code.
- Implement more unit tests
