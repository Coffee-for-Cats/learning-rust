fn main() {
    let mut s1 = String::from("hello");

    let r1 = &mut s1;
    //não dá, por que não podem haver mais do que 1 referência a uma "variável complexa" ao mesmo tempo
    //let r2 = &mut s1;

    //também não da, por que o valor dela poderia ser alterado sem nenhum indicativo de tal
    //let s2 = &s1;

    change(r1);
    println!("{s1}");

    //agora é possível, pois o compilador já excluiu a primeira
    let r2 = &mut s1;
    println!("{r2}");

    //mas eu não posso fazer isso, porque se não ele não iria excluir
    //println!("{r1}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
