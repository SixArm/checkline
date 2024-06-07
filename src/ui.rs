use cursive::view::{Nameable, Resizable, Selector};

pub fn initialize(c: &mut cursive::Cursive) -> usize  {
    initialize_keys(c);
    initialize_theme(c);
    let row_count = load_rows(c);
    row_count
}

pub fn initialize_keys(c: &mut cursive::Cursive) {
    c.add_global_callback(cursive::event::Key::Esc, |c| c.quit());
}

pub fn initialize_theme(c: &mut cursive::Cursive) {
    let mut theme = c.current_theme().clone();
    theme.shadow = false;
    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette[cursive::theme::PaletteColor::Background] = cursive::theme::Color::Rgb(std::u8::MIN, std::u8::MIN, std::u8::MIN);
    theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::Rgb(std::u8::MAX, std::u8::MAX, std::u8::MAX);
    c.set_theme(theme);
}

pub fn load_rows(c: &mut cursive::Cursive) -> usize {
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

pub fn load_row(line_str: &str, row_index: usize) ->  cursive::views::LinearLayout {
    let mut row = cursive::views::LinearLayout::horizontal();
    let checkbox = cursive::views::Checkbox::new();
    let text_view = cursive::views::TextView::new(line_str.to_owned());
    row.add_child(checkbox.with_name(format!("row_{}_checkbox", row_index)));
    row.add_child(cursive::views::DummyView);
    row.add_child(text_view.with_name(format!("row_{}_text_view", row_index)).full_width());
    row
}

pub fn print_output(c: &mut cursive::Cursive, row_count: usize) {
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
