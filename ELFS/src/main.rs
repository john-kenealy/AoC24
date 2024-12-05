mod day;

fn main() {
    println!("E.L.F.S. -> Effective Liberating Flight Squad");

    //day::one::solve();
    //day::one::solve_two();

    //day::two::solve();
    
    //day::three::solve();
    
    //day::four::solve();
    
    
    use std::time::Instant;
    let now = Instant::now();
    
    day::five::solve();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);   

}
