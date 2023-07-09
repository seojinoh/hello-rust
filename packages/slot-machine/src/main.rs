fn draw_line(start: u32, end: u32, length: u32, paint: &str, background: &str) {
    let mut line: String = String::from("");

    for i in 0..length {
        if i >= start && i < end {
            line.push_str(paint)
        } else {
            line.push_str(background)
        }
    }

    print!("{line}\n");
}

fn draw_seven(height: u32, width: u32) {
    let paint: &str = "🟥";
    let background: &str = "⬜️";

    draw_line(0, width, width, paint, background);
    draw_line(0, width, width, paint, background);

    let mut start: u32 = width - 2;
    for _i in 0..(height - 2) {
        draw_line(start, start + 2, width, paint, background);

        start = start - 1;
    }
}

fn main() {
    draw_seven(10, 10)
}
