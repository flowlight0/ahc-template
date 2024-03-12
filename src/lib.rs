#![allow(non_snake_case)]

use std::str::FromStr;

use proconio::{input, marker::Chars};
use svg::node::{
    element::{Circle, Group, Line, Rectangle, Text, Title},
    Text as TextContent,
};

const W: usize = 600;
const H: usize = 600;
const MARGIN: isize = 5;

#[derive(Clone, Debug)]
struct Input {
    t: usize,
    n: usize,
    vs: Vec<Vec<char>>,
    hs: Vec<Vec<char>>,
    a: Vec<Vec<i64>>,
}

#[derive(Clone, Debug)]
struct Output {
    t_start: (usize, usize),
    a_start: (usize, usize),
    moves: Vec<(bool, char, char)>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct ParseOutputError;

impl FromStr for Input {
    type Err = ParseInputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = proconio::source::once::OnceSource::from(s);
        input! {
            from f,
            t: usize,
            n: usize,
            vs: [Chars; n],
            hs: [Chars; n - 1],
            a: [[i64; n]; n],
        }
        Ok(Input { t, n, vs, hs, a })
    }
}

impl FromStr for Output {
    type Err = ParseOutputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split_whitespace();
        let (tx, ty) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        let (ax, ay) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        let mut moves = vec![];
        loop {
            let m = iter.next();
            if m.is_none() {
                break;
            }
            dbg!(m);
            let swap = match m.unwrap() {
                "0" => false,
                "1" => true,
                c => unreachable!("Detected invalid character: {}", c),
            };
            let c1 = iter.next().unwrap().parse().unwrap();
            let c2 = iter.next().unwrap().parse().unwrap();
            moves.push((swap, c1, c2));
        }

        Ok(Output {
            t_start: (tx, ty),
            a_start: (ax, ay),
            moves,
        })
    }
}

/// 0 <= val <= 1 で青から赤に変化するカラーコードを返す
pub fn color(mut val: f64) -> String {
    val = val.max(0.0).min(1.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

fn rect(x: f64, y: f64, w: f64, h: f64, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
// (y, x)
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

fn compute_cost(a: &Vec<Vec<i64>>, vs: &Vec<Vec<char>>, hs: &Vec<Vec<char>>) -> i64 {
    let mut cost = 0;
    for y in 0..a.len() {
        for x in 0..a[0].len() {
            if x + 1 < a[0].len() && vs[y][x] == '0' {
                let d = a[y][x] - a[y][x + 1];
                cost += d * d;
            }
            if y + 1 < a.len() && hs[y][x] == '0' {
                let d = a[y][x] - a[y + 1][x];
                cost += d * d;
            }
        }
    }
    cost
}

fn write_svg(input: &Input, output: &Output, turn: i32) -> (i64, String, String) {
    assert!(output.moves.len() <= 4 * input.n * input.n);
    assert!(turn <= output.moves.len() as i32);
    let mut input = input.clone();
    let mut t_pos = output.t_start;
    let mut a_pos = output.a_start;
    let initial_cost = compute_cost(&input.a, &input.vs, &input.hs);
    dbg!("foo");
    for t in 0..turn {
        let (do_swap, t_dir, a_dir) = output.moves[t as usize];
        if do_swap {
            let tmp = input.a[t_pos.1][t_pos.0];
            input.a[t_pos.1][t_pos.0] = input.a[a_pos.1][a_pos.0];
            input.a[a_pos.1][a_pos.0] = tmp;
        }

        for i in 0..DIRS.len() {
            if DIRS[i] == t_dir {
                t_pos.0 += DIJ[i].1;
                t_pos.1 += DIJ[i].0;
            }
            if DIRS[i] == a_dir {
                a_pos.0 += DIJ[i].1;
                a_pos.1 += DIJ[i].0;
            }
        }
    }
    let current_cost = compute_cost(&input.a, &input.vs, &input.hs);
    dbg!("bar");
    let score = ((1e6 * (f64::log2(initial_cost as f64) - f64::log2(current_cost as f64))).round()
        as i64)
        .max(1);
    dbg!("baz ");



    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set(
            "viewBox",
            (
                -MARGIN,
                -MARGIN,
                W + MARGIN as usize * 2,
                H + MARGIN as usize * 2,
            ),
        )
        .set("width", W + 10)
        .set("height", H + 10);

    let hd = H as f64 / input.n as f64;
    let wd = W as f64 / input.n as f64;
    for x_index in 0..input.n {
        for y_index in 0..input.n {
            doc = doc.add(rect(
                hd * x_index as f64,
                wd * y_index as f64,
                hd,
                wd,
                &color(input.a[y_index][x_index] as f64 / ((input.n * input.n) as f64)),
            ));
            doc = doc.add(
                Text::new(input.a[y_index][x_index].to_string())
                    .set("x", wd * x_index as f64 + wd * 0.5)
                    .set("y",hd * y_index as f64 + hd * 0.5)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "central")
                    .set("font-size", 20)
                    .set("fill", "black"),
            );
        }
    }

    let line_width = 2;
    for y_index in 0..input.n - 1 {
        for x_index in 0..input.n {
            if input.hs[y_index][x_index] == '1' {
                doc = doc.add(
                    Line::new()
                        .set("x1", wd * x_index as f64)
                        .set("y1", hd * (y_index + 1) as f64)
                        .set("x2", wd * (x_index + 1) as f64)
                        .set("y2", hd * (y_index + 1) as f64)
                        .set("stroke", "black")
                        .set("stroke-width", line_width),
                );
            }
        }
    }
    for y_index in 0..input.n - 1 {
        for x_index in 0..input.n - 1 {
            if input.vs[y_index][x_index] == '1' {
                doc = doc.add(
                    Line::new()
                        .set("x1", wd * (x_index + 1) as f64)
                        .set("y1", hd * y_index as f64)
                        .set("x2", wd * (x_index + 1) as f64)
                        .set("y2", hd * (y_index + 1) as f64)
                        .set("stroke", "black")
                        .set("stroke-width", line_width),
                );
            
            }
        }
    }

    doc = doc.add(
        Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", W)
            .set("y2", 0)
            .set("stroke", "black")
            .set("stroke-width", line_width)
    );
    doc = doc.add(
        Line::new()
            .set("x1", 0)
            .set("y1", H)
            .set("x2", W)
            .set("y2",H)
            .set("stroke", "black")
            .set("stroke-width", line_width)
    );
    doc = doc.add(
        Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", 0)
            .set("y2", H)
            .set("stroke", "black")
            .set("stroke-width", line_width)
    );
    doc = doc.add(
        Line::new()
            .set("x1", W)
            .set("y1", 0)
            .set("x2", W)
            .set("y2",H)
            .set("stroke", "black")
            .set("stroke-width", 1)
    );



    (score, String::new(), doc.to_string())
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct GenOption {}

#[wasm_bindgen]
pub struct VisOption {}

#[wasm_bindgen]
impl VisOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
}

#[wasm_bindgen]
pub struct Ret {
    pub score: i64,
    #[wasm_bindgen(getter_with_clone)]
    pub error: String,
    #[wasm_bindgen(getter_with_clone)]
    pub svg: String,
}

#[wasm_bindgen]
pub fn rust_visualize(input: String, output: String, turn: i32, option: &VisOption) -> Ret {
    let input = Input::from_str(&input).expect("Failed to parse input");
    let output = Output::from_str(&output).expect("Failed to parse output");
    let (score, error, svg) = write_svg(&input, &output, turn);
    Ret { score, error, svg }
}

#[wasm_bindgen]
pub fn rust_gen() -> String {
    "".to_string()
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, output: String) -> i32 {
    let output = Output::from_str(&output).expect("Failed to parse output");
    output.moves.len() as i32
}
