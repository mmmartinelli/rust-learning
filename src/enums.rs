pub(crate) fn enums() {
    println!("It's weekend? {}", is_weekend(DayOfWeek::Wednesday));
    which_color();
}

fn is_weekend(day_of_week:DayOfWeek) -> bool {
    match day_of_week {
        DayOfWeek::Sunday | DayOfWeek::Saturday => true,
        _ => false
    }
}

#[allow(dead_code)]
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

fn which_color() {
    let color = Color::CymkColor{cyan:100, magenta:50, yellow:70, black:255};

    println!("Color: {}", match color {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
        Color::RgbColor(0, 0, 0) | Color::CymkColor { cyan: _, magenta: _, yellow: _, black: 255 } => "Black",
        Color::RgbColor(_, _, _) => "Unknown RGB",
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ } => "Unknown CYMK"
    });
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan:u8, magenta:u8, yellow:u8, black:u8}
}