fn main() {
    let  mut x = 5;
    println!("La valeur de x est : {}", x);
    x = 6;
    println!("La valeur de x est : {}", x);

    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;
    println!("La valeur de la const est : {}", TROIS_HEURES_EN_SECONDES);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);
    }
    println!("La valeur de x est : {}", x);

    let espaces = "   ";
    let espaces = espaces.len();

    println!("La valeur de espaces est : {}", espaces);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let cinq_cents = x.0;

    let six_virgule_quatre = x.1;

    let un = x.2;

    println!("La valeur de cinq_cents est : {}", cinq_cents);
    println!("La valeur de six_virgule_quatre est : {}", six_virgule_quatre);
    println!("La valeur de un est : {}", un);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("La valeur de a est : {}", a);

    let b = [3; 5];
    println!("La valeur de b est : {}", b);

}
