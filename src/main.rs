use plotters::prelude::*;
use core::panic;
use std::env;
use meval::{Context, eval_str, eval_str_with_context};

struct Vector {
    x: f64,
    y: f64,
    r: f64,
    theta: f64,
}

impl Vector {
    fn new() -> Self {
        Vector{
            x: 0.0,
            y: 0.0,
            r: 0.0,
            theta: 0.0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let expr = if args.len() > 1 {&args[1]} else {panic!("INVALID EXPRESSION")};
    let functions = expr.trim();
    let func_vec = functions
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();


    for func in func_vec.clone() {
        println!("{func}");
    }
    // Define the path to save the drawing
    let path = "cartesian_plane.png";
    let root = BitMapBackend::new(path, (1080, 1080)).into_drawing_area();

    // Fill the background
    root.fill(&WHITE)?;

    // Create a Cartesian 2D chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Cartesian Plane", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-15f64..15f64, -15f64..15f64)?;

    // Configure the mesh
    chart.configure_mesh().draw()?;
let line_style = ShapeStyle {
        color: RGBAColor(0xff, 0x00, 0x00, 0.5),
        filled: true,
        stroke_width: 1, // Set the thickness here
    };
    // You can draw more things here (e.g., lines, points)
    for i in -20..=20{
        if i == 0 {continue}
        for j in -20..=20 {
            if j == 0 {continue}
            let start = ((i as f64) / 2.0, (j as f64) / 2.0);
            let mut end = (0.0, 0.0);
            let mut ctx = Context::new();
            ctx.var("x", i as f64)
                .var("y", j as f64);
            for (idx, func) in func_vec.iter().enumerate(){
                let result = eval_str_with_context(func, &ctx).unwrap();
                match idx {
                    0 => end.0 = 3.0 * (0.5 * (result + 1.0).log10()),
                    1 => end.1 = 3.0 * (0.5 * (result + 1.0).log10()),
                    _ => {}
                }
            }
            end.0 += start.0;
            end.1 += start.1;
            let indices = get_vector(start, end);
            chart.draw_series(LineSeries::new(vec![indices[0], indices[1]], line_style))?;
            chart.draw_series(LineSeries::new(vec![indices[1], indices[2]], line_style))?;
            chart.draw_series(LineSeries::new(vec![indices[1], indices[3]], line_style))?;
            // println!("{},{} -> {},{}", start.0, start.0, end.0, end.1)
        }
    }

    // Save the drawing
    root.present()?;

    Ok(())
}

fn get_vector (start: (f64, f64), end: (f64, f64)) -> Vec<(f64, f64)>{
    let scaled_length = (0.4 * (end.0 - start.0), 0.4 * (end.1 - start.1));
    let length = (scaled_length.0.powi(2) + scaled_length.1.powi(2)).sqrt();
    let unit_direction = (scaled_length.0 / length, scaled_length.1 / length);

    // Calculate the points for the arrowhead
    let arrowhead_length = 0.25f64; // Length of the arrowhead lines
    let arrowhead_angle = std::f64::consts::PI / 6.0; // 30 degrees in radians

    // Calculate two points of the arrowhead
    let arrow_point1 = (
        end.0 - arrowhead_length * (unit_direction.0 * arrowhead_angle.cos() - unit_direction.1 * arrowhead_angle.sin()),
        end.1 - arrowhead_length * (unit_direction.1 * arrowhead_angle.cos() + unit_direction.0 * arrowhead_angle.sin()),
    );
    let arrow_point2 = (
        end.0 - arrowhead_length * (unit_direction.0 * arrowhead_angle.cos() + unit_direction.1 * arrowhead_angle.sin()),
        end.1 - arrowhead_length * (unit_direction.1 * arrowhead_angle.cos() - unit_direction.0 * arrowhead_angle.sin()),
    );

    vec![start, end, arrow_point1, arrow_point2]
}
