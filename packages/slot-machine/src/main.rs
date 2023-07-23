struct Canvas {
    height: u32,
    width: u32,
}

impl Canvas {
    fn new(height: u32, width: u32) -> Canvas {
        Canvas {
            height,
            width,
        }
    }
}

struct Paint {
    highlight: String,
    background: String,
}

impl Paint {
    fn new(highlight: &String, background: &String) -> Paint {
        Paint {
            highlight: highlight.clone(),
            background: background.clone(),
        }
    }
}

struct Point {
    start_from: u32,
    end_to: u32,
}

impl Point {
    fn new(start_from: u32, end_to: u32) -> Point {
        Point {
            start_from,
            end_to
        }
    }
}

fn draw_line(canvas: &Canvas, paint: &Paint, point: &Point) {
    let mut line: String = String::from("");

    for i in 0..canvas.width {
        if i >= point.start_from && i < point.end_to {
            line.push_str(paint.highlight.as_str());
        } else {
            line.push_str(paint.background.as_str());
        }
    }

    line.push_str("\t");

    print!("{line}\n");
}

fn draw_seven(canvas: &Canvas, paint: &Paint) {
    let mut point = Point::new(0, canvas.width);

    draw_line(&canvas, &paint, &point);
    draw_line(&canvas, &paint, &point);

    point.start_from = canvas.width.clone() - 2;
    point.end_to = canvas.width.clone();

    for _i in 0..(canvas.height - 2) {
        draw_line(&canvas, &paint, &point);

        point.start_from = point.start_from.clone() - 1;
        point.end_to = point.start_from.clone() + 2;
    }
}

fn main() {
    let canvas = Canvas::new(10, 10);
    let paint = Paint::new(&"🟥".to_string(), &"⬜️".to_string());

    draw_seven(&canvas, &paint);
}