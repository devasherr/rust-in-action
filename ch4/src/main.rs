type Message = String;

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id,
            mailbox: MailBox {
                messages: Vec::new(),
            },
        }
    }

    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat::new(0);

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: {:?}", msg)
}
