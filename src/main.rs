use plotters::prelude::*;
use core::{f64, panic};
use std::env;
use meval::{Context, eval_str_with_context};

struct Vector {
    tail: (f64, f64),
    r: f64,
    theta: f64,
}

impl Vector {
    fn new(tail_val: (f64, f64), r_val: f64, theta_val: f64) -> Self {
        Vector{
            tail: tail_val,
            r: r_val,
            theta: theta_val,
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
    let path = "plot.png";
    let root = BitMapBackend::new(path, (1080, 1080)).into_drawing_area();

    // Fill the background
    root.fill(&WHITE)?;

    // Create a Cartesian 2D chart
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("F = ({}, {})", func_vec[0], func_vec[1]), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-3f64..3f64, -3f64..3f64)?;

    // Configure the mesh
    chart.configure_mesh().draw()?;
    let line_style = ShapeStyle {
        color: RGBAColor(0xff, 0x00, 0x00, 0.5),
        filled: true,
        stroke_width: 1, // Set the thickness here
    };
    // You can draw more things here (e.g., lines, points)
    let scale = 5.0;
    let range = 10;
    for i in -range..=range{
        for j in -range..=range {
            let start = ((i as f64) / scale, (j as f64) / scale);
            if i == 0 && j == 0 {
                chart.draw_series(std::iter::once(Circle::new(
                    start, // Coordinates for the point
                    2,     // Radius of the circle, effectively the size of the point
                    RED.filled(),
                )))?;
                continue
            }
            let mut ctx = Context::new();
            ctx.var("x", i as f64 / scale)
                .var("y", j as f64 / scale);
            let mut values: Vec<f64> = vec![];
            for func in func_vec.iter(){
                values.push(eval_str_with_context(func, &ctx).unwrap());
            }
            println!("{}, {}", values[0], values[1]);
            if !(values[0].to_string() == "NaN" || values[1].to_string() == "NaN" || values[0].to_string() == "inf" || values[1].to_string() == "inf"){
                let mut r_val = (values[0].powi(2) + values[1].powi(2)).sqrt();
                // scale magnitude
                r_val = 3.0 * (0.5 * (r_val + 1.0).log10());
                let theta_val = values[1].atan2(values[0]);
                let vector = Vector::new(start, r_val, theta_val);
                let indices = get_vector_line(vector);

                chart.draw_series(std::iter::once(Circle::new(
                    start, // Coordinates for the point
                    2,     // Radius of the circle, effectively the size of the point
                    RED.filled(),
                )))?;
                chart.draw_series(LineSeries::new(vec![indices[0], indices[1]], line_style))?;
                chart.draw_series(LineSeries::new(vec![indices[1], indices[2]], line_style))?;
                chart.draw_series(LineSeries::new(vec![indices[1], indices[3]], line_style))?;
            }
            // println!("{},{} -> {},{}", start.0, start.0, end.0, end.1)
        }
    }

    // Save the drawing
    root.present()?;

    Ok(())
}

fn get_vector_line (vector: Vector) -> Vec<(f64, f64)>{
    // Calculate the points for the arrowhead
    let arrowhead_length = 0.25 * vector.r; 
    let arrowhead_angle = 5.0 * std::f64::consts::PI / 6.0; 

 let arrow_point1 = polar_to_cartesian(
        arrowhead_length,
        vector.theta - arrowhead_angle,
        vector.tail.0 + vector.r * vector.theta.cos(),
        vector.tail.1 + vector.r * vector.theta.sin(),
    );
    let arrow_point2 = polar_to_cartesian(
        arrowhead_length,
        vector.theta + arrowhead_angle,
        vector.tail.0 + vector.r * vector.theta.cos(),
        vector.tail.1 + vector.r * vector.theta.sin(),
    );

    // Collect points for plotting (assuming a function to plot or collect these points)
    vec![vector.tail, (vector.tail.0 + vector.r * vector.theta.cos(), vector.tail.1 + vector.r * vector.theta.sin()) , arrow_point1, arrow_point2]
}

fn polar_to_cartesian(r: f64, theta: f64, offset_x: f64, offset_y: f64) -> (f64, f64) {
    (
        offset_x + r * theta.cos(),
        offset_y + r * theta.sin(),
    )
}
