//Crossterm Backend for tui-rs
use std::io;

use structopt::StructOpt;
//tui-rs Terminal
use tui::backend::CrosstermBackend;
use tui::style::{Color, Style};
use tui::Terminal;
use tui::widgets::{Block, Borders, Chart, Widget};
use tui::widgets::Axis;
use tui::widgets::Dataset;
use tui::widgets::Marker;

#[derive(StructOpt, Debug)]
#[structopt()]
/// Input a Quadratic in the form ax²+bx+c
struct Opt {
    #[structopt(short = "a", long = "a", default_value = "0")]
    ///The variable a in ax²+bx+c
    a: f64,

    #[structopt(short = "b", long = "b", default_value = "0")]
    ///The variable b in ax²+bx+c
    b: f64,

    #[structopt(short = "c", long = "c", default_value = "0")]
    ///The variable c in ax²+bx+c
    c: f64,
}

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();

    let a: f64 = opt.a;
    let b: f64 = opt.b;
    let c: f64 = opt.c;

    let d: f64 = b / (2f64 * a);
    let e: f64 = ((b * b) / (4f64 * a)) - c;

    let tp: (f64, f64) = (d, e);
    let xt = tp.0;
    let _yt = tp.1;

    let range: f64 = 10f64;

    let xmin: f64 = xt - (range / 2f64);
    let xmax: f64 = xt + (range / 2f64);

    let y1: f64 = ((a * (xmax + d)) * (a * (xt + d))) + e;
    let y2: f64 = ((a * (xmin + d)) * (a * (xmin + d))) + e;

    fn min(y1: f64, y2: f64) -> f64 {
        if y1 < y2 {
            return y1;
        } else { return y2; }
    }

    fn max(y1: f64, y2: f64) -> f64 {
        if y1 > y2 {
            return y1;
        } else { return y2; }
    }

    let ymin = min(y1, y2);
    let ymax = max(y1, y2);

    let mut points: Vec<(f64, f64)> = vec![tp];

    let mut i = xmin;

    while i < xmax {
        i = i + 0.25;

        let y = (i + d) * (i + d) + e;

        points.push((i, y));
    }

    let backend = CrosstermBackend::new();
    let mut terminal: Terminal<CrosstermBackend> = Terminal::new(backend)?;

    terminal.clear().expect("Couldn't clear terminal");

    terminal.draw(|mut f| {
        let size = f.size();

        Chart::default()
            .block(Block::default()
                .title("Graphing Equations")
                .title_style(Style::default().fg(Color::Red))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightGreen))
            )
            .x_axis(Axis::default()
                .title("X")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::LightGreen))
                .bounds([xmin, xmax])
                .labels(&["0.0", "5.0", "10.0"])
            )
            .y_axis(Axis::default()
                .title("Y")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::LightGreen))
                .bounds([ymin, ymax])
                .labels(&["0.0", "5.0", "10.0"])
            )
            .datasets(&[Dataset::default()
                .name("Graph")
                .marker(Marker::Dot)
                .style(Style::default().fg(Color::Red))
                .data(&points)
            ])
            .render(&mut f, size);
    })
}