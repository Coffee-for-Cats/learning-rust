#[derive(Debug)]
enum _IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    _Quit,
    Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&mut self) {
        //match...
        dbg!(self);
    }
}

fn main() {
    //let four = _IpAddrKind::V4(127, 0, 0, 1);
    //_route(&four);

    //These both are of the type "Message", but this type can be 4 diferent things!
    //It can be a "Write" (a tuple with a String inside), or a "Move" (an object with x and y)
    let mut m = Message::Write(String::from("Hello"));
    m.call();
    //same as:
    //Message::call(&mut m);
    let mut m = Message::Move { x: 4, y: 5 };
    m.call();
}

fn _route(ip_kind: &_IpAddrKind) {
    dbg!(ip_kind);
}
