###1 Napisz program, który wyświetla informację o przestępności danego roku.

fn main (){
    let rok = 2002;
    if rok % 4 == 0 || rok % 100 == 0|| rok % 400 == 0 {
        println!("Jest przestepny")
    }
    else {
        println!("Nie jeste przestepny")
    }
}

2 Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.

fn main (){
    let rok = 2002;
    let miesiac = 12;
    if rok % 4 == 0 || rok % 100 == 0|| rok % 400 == 0 {
        if miesiac == 1 || miesiac == 3 || miesiac == 5 || miesiac == 7 || miesiac == 8 || miesiac == 10|| miesiac == 12 {
            println!("31");
        }
        else if miesiac == 2{
            println!("29");
        }
        else{
            println!("30");
        }
    }
    else {
        if miesiac == 1 || miesiac == 3 || miesiac == 5 || miesiac == 7 || miesiac == 8 || miesiac == 10|| miesiac == 12 {
            println!("31");
        }
        else if miesiac == 2{
            println!("28");
        }
        else{
            println!("30");
        }
    }
}

3/4 Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza
na stopnie w skali Fahrenheita; F=32+(9/5)C i odwrotnie
fn main(){
    let fahrenheit:f32 = 101.0;
    let celcius:f32 = 100.0;

    let new_fahrenheit =  32+(9/5)*celcius;
    let new_celcius = (fahrenheit - 32) / (9/5);

    println!("{fahrenheit} stopni fahrenheit'a to {new_celcius} stopni celcjusza\n");
    println!("{celcius} stopni celcjusza to {new_fahrenheit} stopni fahrenheita\n");
}

5 Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund)
wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).

fn main(){
    let h1 = 10;
    let h2 = 12;

    let m1 = 45;
    let m2 = 0;

    let s1 = 12;
    let s2 = 0;

    let mut h_dif =(h1-h2)%60;
    let mut m_dif =(m1-m2)%60;
    let mut s_dif =(s1-s2)%60;

    if h_dif<0 {
        h_dif = h_dif + 24;
    }
    if m_dif<0 {
        m_dif = m_dif + 60;
    }
    if s_dif<0 {
        s_dif = s_dif + 60;
    }
    println!("roznica to {h_dif} godzin {m_dif} minut {s_dif} sekund")
}
