use std::env;
use rand::Rng;
use std::time::Instant;

const MAXN: usize = 2000;

fn initialize_inputs(n: usize, a: &mut Vec<Vec<f32>>, b: &mut Vec<f32>, x: &mut Vec<f32>) {
    let mut rng = rand::thread_rng();
    for col in 0..n {
        for row in 0..n {
            a[row][col] = rng.r#gen::<f32>(); // Sem r#gen, pois Rust 2021 permite
        }
        b[col] = rng.r#gen::<f32>();
        x[col] = 0.0;
    }
}

fn print_matrix(n: usize, a: &Vec<Vec<f32>>, b: &Vec<f32>) {
    if n < 10 {
        println!("\nA =");
        for row in 0..n {
            for col in 0..n {
                print!("{:.2} ", a[row][col]);
            }
            println!();
        }
        println!("\nB =");
        for col in 0..n {
            print!("{:.2} ", b[col]);
        }
        println!();
    }
}

fn gauss(n: usize, a: &mut Vec<Vec<f32>>, b: &mut Vec<f32>, x: &mut Vec<f32>) {
    println!("Computing Serially.");
    for norm in 0..n - 1 {
        for row in norm + 1..n {
            let multiplier = a[row][norm] / a[norm][norm];
            for col in norm..n {
                a[row][col] -= a[norm][col] * multiplier;
            }
            b[row] -= b[norm] * multiplier;
        }
    }

    for row in (0..n).rev() {
        x[row] = b[row];
        for col in row + 1..n {
            x[row] -= a[row][col] * x[col];
        }
        x[row] /= a[row][row];
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <matrix_dimension>", args[0]);
        return;
    }

    let n: usize = args[1].parse().unwrap_or(0);
    if n < 1 || n > MAXN {
        eprintln!("N = {} is out of range.", n);
        return;
    }

    println!("\nMatrix dimension N = {}", n);

    // Criando matrizes e vetores dinamicamente
    let mut a: Vec<Vec<f32>> = vec![vec![0.0; n]; n];
    let mut b: Vec<f32> = vec![0.0; n];
    let mut x: Vec<f32> = vec![0.0; n];

    initialize_inputs(n, &mut a, &mut b, &mut x);
    print_matrix(n, &a, &b);

    let start = Instant::now();
    gauss(n, &mut a, &mut b, &mut x);
    let duration = start.elapsed();

    println!("\nElapsed time = {:.3} ms.", duration.as_millis());

    if n < 100 {
        print!("\nX = [");
        for row in 0..n {
            print!("{:.2} ", x[row]);
        }
        println!("]");
    }
}