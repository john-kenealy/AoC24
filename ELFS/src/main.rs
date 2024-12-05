mod day;

fn main() {
    println!("E.L.F.S. -> Effective Liberating Flight Squad");

    //day::one::solve();
    //day::one::solve_two();

    //day::two::solve();
    
    //day::three::solve();
    
    //day::four::solve();
    
    
    // PART 1
    use std::time::Instant;

    let mut now = Instant::now();
    day::five::solve(true);

    let mut elapsed = now.elapsed();
    println!("Part 1 Elapsed: {:.2?}", elapsed);   



    // PART 2
    now = Instant::now();

    day::five::solve(false);

    elapsed = now.elapsed();
    println!("Part 2 Elapsed: {:.2?}", elapsed);   

}
