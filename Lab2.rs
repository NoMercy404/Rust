// 5. Zmodyfikuj kod z zadań 1-4 tak, aby stanowiły one osobne funkcje.

// 1. Napisz program, który oblicza silnię dla danej liczby.

// Bez osobnej funkcji

fn main(){
    let mut factorial = 1;
    let n = 5;
    let mut i = 1;

    while i<=n{
        factorial*=i;
        i+=1
    }
    println!("slinia z liczby {n} wynosi {factorial}");
}

// Z funkcją

fn calc_fact(n:u64)-> u64{
    let mut factorial = 1;
    let mut i = 1;
    while i<=n{
        factorial*=i;
        i+=1
    }
    return factorial;
}
fn main(){
    let n = 5;
    println!("slinia z liczby {n} wynosi {:?}",calc_fact(n));
}

// 2. Napisz program, który wyświetla cyfry danej liczby całkowitej od końca.

// Bez osobnej funkcji

fn main(){
    let mut number = 12222231;
    while number>0{
        let newnum = number%10;
        println!("{newnum}");
        number = number/10;
    }
}

// Z funkcją

fn print_numbers(mut number:i32){
    while number>0 {
        let newnum = number%10;
        number = number/10;
        println!("{newnum}");
    }
}
fn main(){
    let number = 121;
    print_numbers(number)
}

// 3. Napisz program, który oblicza sumę cyfr danej liczby całkowitej.

// Bez osobnej funkcji

fn main(){
    let mut j = 122334;
    let mut sum = 0;
    while j>0{
        sum = sum + j%10;
        j = j/10;
    }
    println!("suma {sum}");
}

// Z funkcją

fn sum_of_digits(mut n:u64)->u64{
    let mut sum = 0;
    while n>0{
        sum = sum + n%10;
        n = n/10;
    }
    return sum;
}
fn main(){
    let num = 123424;
    println!("suma {:?}",sum_of_digits(num));
}

// 4. Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana. Zakładamy, że 0 < a < b < c.

// Bez osobnej funkcji

fn main(){
    let j = 20;
    for a in 1..j{
        for b in a+1..j{
            for c in b+1..j{
                if (a*a)+(b*b)==c*c{
                    println!("{a},{b},{c}");
                }
            }
        }
    }
}

// Z funkcją

fn pythagorean_triples(n:u64){
    for a in 1..n{
        for b in a+1..n{
            for c in b+1..n{
                if (a*a)+(b*b)==c*c{
                    println!("{a},{b},{c}");
                }
            }
        }
    }
}

fn main(){
    let j = 20;
    println!("Trojki Pitagorejskie mniejsze niz {j}");
    pythagorean_triples(j);
}

// 6. Zaimplementuj wyznaczanie pierwiastków funkcji rzeczywistej f metodą Newtona w postaci funkcji, która zrealizuje liczbę 
// kroków algorytmu przekazaną w argumencie. Wyodrębnij funkcję, która zwróci znak pochodnej f' w punkcie. 
// Obliczanie wartości funkcji f zrealizuj funkcją wpisaną "na twardo"w implementację metody Newtona.

fn f(x: f64) -> f64 {
    3.0 * x - 9.0
}

fn signum(x: f64) -> i8{
    let epsilon = 0.0000001;
    if x.abs() < epsilon {
        0
    }else if x > epsilon{
        1
    }else {
        -1
    }
}

fn sgn_f_deriv(x: f64) -> i8 {
    let epsilon = 0.0000001;
    let d = f(x + epsilon) - f(x);
    signum(d)
}

fn newton(n: u64) -> f64{
    let mut x = 7.0;
    let mut delta = 1.0;

    let mut prvs_jump_left = false;
    for _i in 0..n{
        let f_val = f(x);
        let deriv_sng = sgn_f_deriv(x);

        println!("Iteracja: {}", _i+1);
        println!("f = {}", f_val);
        println!("f' = {}", deriv_sng);
        println!("x = {}", x);
        println!("delta = {} \n", delta);

        let mut jump_left: bool = false;

        if (signum(f_val) > 0 && deriv_sng < 0) || (signum(f_val) < 0 && deriv_sng > 0){
            x += delta;
        }else{
            x -= delta;
            jump_left = true;
        }

        if jump_left != prvs_jump_left{
            delta /= 2.0;
        }

        prvs_jump_left = jump_left;
    }

    x
}
fn main() {
    newton(1000);
}
