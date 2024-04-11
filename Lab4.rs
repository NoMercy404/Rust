1. Napisz funkcję d2((x, y), (x, y)) -> f32, która obliczy dystans pomiędzy punktami w przestrzeni R^2.
fn d2(a:(i32,i32),b:(i32,i32))->f32{
    let diff1:f32 = (b.0 - a.0).pow(2) as f32;
    let diff2:f32 = (b.1 - a.1).pow(2) as f32;
    let sum:f32 = f32::sqrt(diff1 + diff2) as f32;
    return sum
}
fn main(){
    let x1 = 10;
    let y1 = 11;

    let x2 = 12;
    let y2 = 13;
    println!("{}",d2((x1,y1),(x2,y2)));
}

2. Napisz funkcję d3((x, y, z), (x, y, z)) -> f32, która obliczy dystans pomiędzy punktami w przestrzeni R^3.
fn d3(a:(i32,i32,i32),b:(i32,i32,i32))->f32{
    let diff1:f32 = (b.0 - a.0).pow(2) as f32;
    let diff2:f32 = (b.1 - a.1).pow(2) as f32;
    let diff3:f32 = (b.2 - a.2).pow(2) as f32;
    let sum:f32 = f32::sqrt(diff1 + diff2 + diff3) as f32;
    return sum
}
fn main(){
    let x1 = 10;
    let y1 = 11;
    let z1 = 12;

    let x2 = 13;
    let y2 = 14;
    let z2 = 15;

    println!("{}",d3((x1,y1,z1),(x2,y2,z2)));
}

3. Stwórz tablicę N elementów, którą wypełnisz resztami z dzielenia liczby 100 przez kolejne liczby naturalne.
Następnie wyświetl kolejne wartości tablicy od końca.

fn fill_array(arr:&mut[i32]){
    for i in 0..arr.len(){
        arr[i] = i as i32 % 100;
    }
}
fn display_array_in_reverse(arr:&[i32]){
    for reverse_element in arr.iter().rev(){
        println!("{}",reverse_element);
    }
}
fn main(){
    let _n:usize = 10;
    let mut array :[i32;10] = [0;10];
    fill_array(&mut array);
    display_array_in_reverse(&array);
}

4. Napisz funkcję avg(&[u32]) -> f32, która obliczy średnią arytmetyczną liczb z tablicy.

fn avg(arr:&[u32]) -> f32{
    if arr.is_empty(){
        return 0.0
    }

    let sum :u32 = arr.iter().sum();
    let count = arr.len() as f32;
    return sum as f32 /(count);
}

fn main(){
    let array = [1,2,3,4,5,5,6,7,8,9];
    let average = avg(&array);
    println!("średnia wynosi {:?}",average);
}

5. Napisz funkcję sort(... u32, ... u32, ... u32), która rosnąco posortuje przekazane jej argumenty.

fn sort(a:&mut u32 , b:&mut u32 , c:&mut u32){
    if *a>*b{
        let  temp1 = *a;
        *a = *b;
        *b = temp1;
    }
    if *b > *c {
        let  temp2 = *b;
        *b = *c;
        *c = temp2;
    }
    if *a>*b{
        let  temp3 = *a;
        *a = *b;
        *b = temp3;
    }
}

fn main(){
    let mut a =5;
    let mut b =3;
    let mut c =0;
    println!("przed sortowaniem a:{} b:{} c:{}",a,b,c);
    sort(&mut a,&mut b,&mut c);
    println!("po sortowaniu a:{} b:{} c:{}",a,b,c);
}

6. Napisz funkcję swap_range(... [u32], (a1, a2), (b1, b2)), która zamieni miejscami elementy,
leżące w podanych przedziałach; jeśli przedziały mają różną długość, ogranicz się do długości krótszego z nich.

fn swap_range(arr:&mut [u32], mut r1:(u32, u32), mut r2:(u32, u32)){
    let mut arr2 = arr.to_owned();
    println!("{:?}",arr2);
    let mut x:usize = r2.0 as usize;
    let mut y:usize = r1.0 as usize;

    for i in r1.0..r1.1{
        arr[i as usize] = arr2[x];
        x +=1;
    }
    for j in r2.0..r2.1{
        arr[j as usize] = arr2[y];
        y+=1;
    }

}

fn main() {
    let range1 = (0,3);
    let range2 = (4,7);
    let mut array:[u32;12] = [1,2,3,4,5,6,7,8,9,10,11,12];
    println!("{:?}",array);
    swap_range(&mut array,range1,range2);
    println!("{:?}",array);
}

7. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz
i podawane w pierwszym parametrze, a w parametrze drugim i trzecim,
podane będą minimalne i maksymalne wartości losowanych liczb.
Funkcja powinna mieć nagłówek: fn rand(seed: ..., min: ..., max: ...). Skorzystaj z LCG.

fn rand(seed: u64, min: u64, max: u64)->u64{
    let a: u64 = 636413;
    let c: u64 = 144269;
    let m = 2_u64.pow(32);
    let next_seed = (a * seed + c) % m;

    // Normalizacja do zakresu [min, max]
    let range = max - min + 1;
    let rand_num = min + (next_seed % range);

    rand_num
}

fn main(){
    let seed:u64 = 15;
    let min:u64 = 0;
    let max:u64 = 121;
    println!("{:?}",rand(seed,min,max));
}
