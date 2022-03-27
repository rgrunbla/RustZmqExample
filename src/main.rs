use zmq::{Context, Message, Error};

fn main() {
    let action = std::env::args().nth(1).expect("no action given. Expected 'server' or 'client'");
    let ctx = Context::new();
    let addr = "tcp://127.0.0.1:1234";
    match &action[..] {
        "server" => {
            let sock = ctx.socket(zmq::PULL).unwrap();
            sock.bind(addr).unwrap();
            loop {
                let flags = 0;
                let string = sock.recv_string(flags).unwrap();
                println!("Une string: {:?}", string);
                let message = sock.recv_msg(flags).unwrap();
                println!("Un message: {:?}", message);
            }

        },
        "client" => {
            let sock = ctx.socket(zmq::PUSH).unwrap();
            sock.connect(addr).unwrap();
            let flags = 0;
            sock.send_str("Hello", flags);
            let data = [4, 2, 1, 3, 3, 7];
            let msg = Message::from_slice(&data);
            sock.send_msg(msg, flags);
        },
        _ => panic!("Wrong action !")
    }
} 