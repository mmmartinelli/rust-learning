pub(crate) fn arrays() {
    print_scores();
    print_matrix();
}

fn print_scores() {
    let scores: [f32; 4] = [6.5; 4];

    for index in 0..scores.len() {
        println!("Score {}: {}", index + 1, scores[index]);
    }
}

fn print_matrix() {
    let matrix: [[f64; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4],
    ];

    for row in matrix {
        for col in row {
            println!("Data: {}", col);
        }
    }
}