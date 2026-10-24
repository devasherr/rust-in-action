use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }

    fn deliver(&mut self, recipent: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipent.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

impl CubeSat {
    // fn new(id: u64) -> CubeSat {
    //     CubeSat {
    //         id,
    //         mailbox: MailBox {
    //             messages: Vec::new(),
    //         },
    //     }
    // }

    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

impl GroundStation {
    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: MailBox {
                messages: Vec::new(),
            },
        }
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = MailBox { messages: vec![] };
    let base = GroundStation { radio_freq: 101.1 };
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let _ = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("Hello"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }

    // Reference Counting: std::rc::Rc
    // use when multiple parts of a program must own the same value (example: Graph, Tree ...)

    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 101.1 }));
    println!("base: {:?}", base);

    {
        let mut base2 = base.borrow_mut(); // points to the same place as base
        base2.radio_freq -= 91.1;
        println!("base2: {:?}", base2);
    }

    println!("base: {:?}", base);

    let mut base3 = base.borrow_mut(); // also points to the same place as base
    base3.radio_freq += 11.2;
    println!("base3: {:?}", base3);

    println!("base: {:?}", base);
}
