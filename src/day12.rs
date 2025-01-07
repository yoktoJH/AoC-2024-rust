use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Edge {
    Vertical(i32, i32),
    Horizontal(i32, i32),
}

fn remove_horizontal(edges: &mut HashSet<Edge>, x0: i32, y: i32, all_edges: &HashSet<Edge>) {
    let mut x = x0 + 1;

    while edges.contains(&Edge::Horizontal(x, y))
        && !all_edges.contains(&Edge::Vertical(x, y - 1))
        && !all_edges.contains(&Edge::Vertical(x, y))
    {
        edges.remove(&Edge::Horizontal(x, y));
        x += 1;
    }
    let mut x = x0 - 1;
    while edges.contains(&Edge::Horizontal(x, y))
        && !all_edges.contains(&Edge::Vertical(x + 1, y - 1))
        && !all_edges.contains(&Edge::Vertical(x + 1, y))
    {
        edges.remove(&Edge::Horizontal(x, y));
        x -= 1;
    }
}

fn remove_vertical(edges: &mut HashSet<Edge>, x: i32, y0: i32, all_edges: &HashSet<Edge>) {
    let mut y = y0 + 1;
    while edges.contains(&Edge::Vertical(x, y))
        && !all_edges.contains(&Edge::Horizontal(x - 1, y))
        && !all_edges.contains(&Edge::Horizontal(x, y))
    {
        edges.remove(&Edge::Vertical(x, y));
        y += 1;
    }

    let mut y = y0 - 1;
    while edges.contains(&Edge::Vertical(x, y))
        && !all_edges.contains(&Edge::Horizontal(x - 1, y + 1))
        && !all_edges.contains(&Edge::Horizontal(x, y + 1))
    {
        edges.remove(&Edge::Vertical(x, y));
        y -= 1;
    }
}

fn count_sides(edges: &mut HashSet<Edge>) -> usize {
    let mut count = 0;
    let all_edges = edges.clone();
    while !edges.is_empty() {
        count += 1;
        let edge = edges.iter().nth(0).unwrap().clone();
        match edge {
            Edge::Vertical(x, y) => remove_vertical(edges, x, y, &all_edges),
            Edge::Horizontal(x, y) => remove_horizontal(edges, x, y, &all_edges),
        }
        edges.remove(&edge);
    }
    count
}
fn figure_out_price(garden: &mut Vec<Vec<u8>>, x0: i32, y0: i32) -> (usize, usize) {
    let mut edges: HashSet<Edge> = HashSet::new();
    let flower = garden[y0 as usize][x0 as usize];
    let mut enqueued = HashSet::new();
    let mut stack = Vec::new();
    enqueued.insert((x0, y0));
    stack.push((x0, y0));
    let mut perimeter = 0;
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        for (xshift, yshift) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let xx = x + xshift;
            let yy = y + yshift;
            if xx < 0
                || xx as usize >= garden[0].len()
                || yy < 0
                || yy as usize >= garden.len()
                || garden[yy as usize][xx as usize] != flower
            {
                perimeter += 1;
                if yshift == 0 {
                    edges.insert(Edge::Vertical(xx.max(x), y));
                } else {
                    edges.insert(Edge::Horizontal(x, yy.max(y)));
                }
            } else if !enqueued.contains(&(xx, yy)) {
                enqueued.insert((xx, yy));
                stack.push((xx, yy));
            }
        }
    }
    for (x, y) in &enqueued {
        garden[*y as usize][*x as usize] = b'.';
    }
    let sides = count_sides(&mut edges);
    (enqueued.len() * perimeter, enqueued.len() * sides)
}

pub(crate) fn day12(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("unable to read input file");
    let mut garden: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let l = line.bytes().collect();
        garden.push(l);
    }

    let mut price = 0;
    let mut price2 = 0;
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            if garden[y][x] != b'.' {
                let (p1, p2) = figure_out_price(&mut garden, x as i32, y as i32);
                price2 += p2;
                price += p1;
            }
        }
    }

    println!("Solution for day 12 part 1: {}", price);
    println!("Solution for day 12 part 2: {}", price2);
}
