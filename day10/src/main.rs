mod oneline;
mod star1;
mod star2;
fn main() {
    println! {"Star 1:"}
    star1::star1();
    println! {"Star 2:"}
    star2::star2();
    println! {"oneliner in order:"}
    oneline::oneline();
}
