// 1. Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).

fn print_ascii(){
    for i in 33..127{
        println!("{i} : {}",i as u8 as char);
    }
}
fn main(){
    print_ascii();
}

// 2. Napisz funkcję, która dla danego całkowitego dodatniego n zwraca numer iteracji, w której osiągamy jedynkę w problemie Collatza (np. dla n=12 wynikiem jest 9).

fn how_many_iterations(mut num:i32)-> i32{
    let mut iterations = 0;
    while num!= 1{
        if num%2 == 0{
            iterations +=1;
            num = num/2;
            how_many_iterations(num);
        }
        else{
            iterations +=1;
            num = 3*num+1;
            how_many_iterations(num);
        }
    }
    return iterations
}
fn main(){
    println!("{}",how_many_iterations(12))
}

// 3. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga.

fn how_many_digits(mut n:u32)->u32{
    let mut counter = 0;
    while n!=0{
        counter+=1;
        n = n/10;
    }
    return counter
}
fn does_armstrong_number(number:u32,num_of_digist:u32)->bool{
    let mut sum:u32 = 0;
    let mut pom:u32 =number;

    while pom>0{
        let current:u32 = pom%10;
        sum += u32::pow(current,num_of_digist);
        pom = pom/10;
    }
    if number == sum{
        return  true;
    }
    else{
        return false
    }

}
fn main(){
    let n:u32 = 912312213;
    let num_of_digits:u32 = how_many_digits(n);
    println!("{}",does_armstrong_number(n,num_of_digits))
}

// 4. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą.

fn is_perfect(n:i128)->bool{
    let mut sum = 0;
    for i in 1..n{
        if n%i==0 {
            sum+=i;
        }
    }
    if sum == n{
        return true;
    }
    else{
        return false;
    }
}
fn main(){
    let n:i128 = 33550326;
    println!("{}",is_perfect(n))
}


// 5. Napisz funkcję, która wyświetli rozkład podanej liczby na czynniki pierwsze.

fn rozklad(mut n:i64){
    let mut dzielnik = 2;
    while n>1{
        if n%dzielnik == 0{
            println!("{dzielnik}");
            n = n/dzielnik;
        }
        else {
            dzielnik += 1;
        }
    }
}
fn main(){
    let n = 565548;
    println!("rozklad czynnikow liczby {n}:");
    rozklad(n);
}

// 6. Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128 która obliczy (x^n)%p w taki sposób,by działało to
// prawidłowo dla jak największych danych. W celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.
//W tym zadaniu jest błąd : thread 'main' has overflowed its stack trzeba przerobić f.rekurencyjną na funkcję iteracyjną

fn fast_power(x:u128,n:u128)->u128{
    if n == 0{
        return 1;
    }
    if n%2 == 1{
        return x*fast_power(x,u128::pow((n-1)/2,2))
    }
    return fast_power(x,u128::pow(n/2,2))
}

fn pow_mod(x: u128, n: u128, p: u128) -> u128{
    return fast_power(x,n)%p;
}

fn main(){
    println!("{}",pow_mod(3,4,9))
}
