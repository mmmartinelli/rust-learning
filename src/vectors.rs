pub(crate) fn vectors() {
    let mut scores: Vec<f32> = Vec::new();
    // let scores: Vec<f32> = vec![10.0, 6.5, 7.5];
    // let mut scores: Vec<f32> = Vec::with_capacity(3);
    scores.push(10.0);
    scores.push(6.5);
    scores.push(7.5);

    // println!("Score 1: {}", scores[0]);

    // println!("Score: {}", match scores.get(2) {
    //     Some(s) => *s,
    //     // Some(&s) => s,
    //     None => 0.0
    // });

    for s in &scores {
        println!("Score: {}", s);
    }

    println!("{:?}", scores);
}