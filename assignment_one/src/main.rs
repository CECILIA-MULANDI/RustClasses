
// Print out an items in reverse from 50 to 1 (hint: range operator)
fn main() {
   reverse();
}

fn reverse(){
    for i in (1..=50).rev(){
        println!("{}",i);
    }
}
