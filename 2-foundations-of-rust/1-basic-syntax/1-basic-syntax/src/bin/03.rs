fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut minim = input[0];
    let mut maxim = minim;
    for elem in input {
        if elem < minim {
            minim = elem;
        }
        if elem > maxim {
            maxim = elem
        }
    }

    println!("{} is largest and {} is smallest", maxim, minim);
}
