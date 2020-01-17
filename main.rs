fn main() {
    let y = 9;
    for x in 1..8 {

    if x==0 {
       continue;
    }

    println!("{} x {} = {}", y, x, x*y);
}
    for z in 11..18 {
    if z==0 {
       continue;
    }

    println!("{} x {} = {}", y, z, y*z);
}

    for number in 10..-10 {
        if number==10{
           continue;
          number = number - 1;}

          println!("{}", number);

    }
}

