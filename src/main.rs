use day_one::day_one;

const DAYS: [fn(); 1] = [day_one];
fn main() {
    DAYS[0]();
}