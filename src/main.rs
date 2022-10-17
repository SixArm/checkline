//! checkline: checkbox line picker for stdin line input
//!
//! Example:
//! 
//! ```sh
//! printf "%s\n%s\n%s\n" alpha bravo charlie > example.txt
//! cat example.txt | checkline
//! ```
//! 
//! You should see each line with a checkbox and text:
//! 
//! ```txt
//! [ ] alpha
//! [ ] bravo
//! [ ] charlie
//! ```
//! 
//! Move up and down in the list by typing arrow keys.
//! 
//! Check or uncheck any checkbox by typing space or return.
//! 
//! Check each line that you want, then type ESC to escape.
//! 
//! The command prints each line you want.
//! 
//! 
//! ## Purpose
//! 
//! The purpose of this command is a simple picker, that is easy to use, and
//! that work wells in on the command line such as within a pipe.
//! 
//! The purpose isn't intended to handle very long lines, or very long inputs.
//! 
//! 
//! ## Comparisons
//! 
//! See the command `vipe` that can pipe in and out of `$EDITOR`:
//! <https://github.com/juliangruber/vipe>
//! 
//! 
//! ## Feedback
//! 
//! We welcome consructive criticism and ideas for improvements.
//! 
//! 
//! ## Tracking
//! 
//! * Program: checkline
//! * Version: 1.0.0
//! * License: MIT OR BSD OR GPL-2.0 OR GPL-3.0
//! * Created: 2022-10-15T12:24:50Z
//! * Updated: 2022-10-17T21:13:13Z
//! * Website: https://github.com/sixarm/checkline
//! * Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)

use cursive::view::{Nameable, Resizable, Selector};

fn main() {
    let mut c = cursive::default();
    initialize_keys(&mut c);
    initialize_theme(&mut c);
    let row_count = load_rows(&mut c);
    c.run();
    print_output(&mut c, row_count)
}

fn initialize_keys(c: &mut cursive::Cursive) {
    c.add_global_callback(cursive::event::Key::Esc, |c| c.quit());
}

fn initialize_theme(c: &mut cursive::Cursive) {
    let mut theme = c.current_theme().clone();
    theme.shadow = false;
    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette[cursive::theme::PaletteColor::Background] = cursive::theme::Color::TerminalDefault;
    theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    c.set_theme(theme);
}

fn load_rows(c: &mut cursive::Cursive) -> usize {
    let mut row_index: usize = 0;
    let mut grid = cursive::views::LinearLayout::vertical();
    for line in std::io::stdin().lines() {
        let line = line.expect("line");
        if line.len() > 0 {
            grid.add_child(load_row(&line, row_index).full_width());
            row_index += 1;
        }
    }
    c.add_fullscreen_layer(grid.with_name("grid").full_screen());
    row_index
}

fn load_row(line_str: &str, row_index: usize) ->  cursive::views::LinearLayout {
    let mut row = cursive::views::LinearLayout::horizontal();
    let checkbox = cursive::views::Checkbox::new();
    let text_view = cursive::views::TextView::new(line_str.to_owned());
    row.add_child(checkbox.with_name(format!("row_{}_checkbox", row_index)));
    row.add_child(cursive::views::DummyView);
    row.add_child(text_view.with_name(format!("row_{}_text_view", row_index)).full_width());
    row
}

fn print_output(c: &mut cursive::Cursive, row_count: usize) {
    let mut output = String::new();
    let mut checked: bool;
    for row_index in 0..row_count {
        checked = false;
        c.call_on(&Selector::Name(&format!("row_{}_checkbox", row_index)), |checkbox: &mut cursive::views::Checkbox| {
            checked = checkbox.is_checked();
        });
        if checked {
            c.call_on(&Selector::Name(&format!("row_{}_text_view", row_index)), |text_view: &mut cursive::views::TextView| {
                output.push_str(&format!("{}\n", text_view.get_content().source()));
            });
        }
    }
    if output.len() == 0 { output = String::from("\n"); }
    print!("{}", output);
}
