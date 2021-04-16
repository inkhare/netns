use std::rc::Rc;
use std::cell::RefCell;

use std::process::Command;

use netns::NetNS;
use netns::{apply_in_ns, ExecStatus};

fn err_test() {
    let a = netns::Error::Sys(netns::Errno::from_i32(23));
    println!("err_test a = {}", a);
}

fn apply() {
    let closeure = || {
        let output = Command::new("sh").arg("-c").arg("ip a").output();
        println!("ip a = {:?}", output);
        ExecStatus::Success(0)
    };

    apply_in_ns(2667, closeure);
}

fn ns_test() {
    let ns1 = NetNS::get();
    if !ns1.is_ok() {
        panic!("get failed {:?}", ns1);
    }

    let ref_ns1 = Rc::new(RefCell::new(ns1.unwrap()));
    println!("get ns = {:?}", *ref_ns1.borrow());

    let output = Command::new("sh").arg("-c").arg("ip a").output();
    println!("ip a = {:?}", output);

    let ns2 = NetNS::new();
    match ns2 {
        Ok(ns) => {
            println!("new ns = {:?}", ns);
        }
        Err(e) => {
            println!("new ns failed = {:?}", e);
        }
    }

    let ns3 = NetNS::get();
    match ns3 {
        Ok(ns) => {
            println!("get ns = {:?}", ns);
        }
        Err(e) => {
            println!("get ns failed = {:?}", e);
        }
    }

    println!("\n");
    let res = NetNS::set(&ref_ns1.borrow());
    match res {
        Ok(r) => {
            println!("set ns = {:?}", r);
        }
        Err(e) => {
            println!("set ns failed = {:?}", e);
        }
    }

    let output = Command::new("sh").arg("-c").arg("ip a").output();
    println!("ip a = {:?}", output);

    let ns4 = NetNS::get();
    match ns4 {
        Ok(ns) => {
            println!("get ns = {:?}", ns);
        }
        Err(e) => {
            println!("get ns failed = {:?}", e);
        }
    }

    let ns5 = NetNS::get_from_process(26671);
    if !ns5.is_ok() {
        println!("get failed {}", ns5.err().unwrap());
        return
    }

    let ref_ns5 = Rc::new(RefCell::new(ns5.unwrap()));

    let res = NetNS::set(&ref_ns5.borrow());
    match res {
        Ok(r) => {
            println!("set ns = {:?}", r);
        }
        Err(e) => {
            println!("set ns failed = {:?}", e);
        }
    }

    let output = Command::new("sh").arg("-c").arg("ip a").output();
    println!("ip a = {:?}", output);

    let res = NetNS::set(&ref_ns1.borrow());
    match res {
        Ok(r) => {
            println!("set ns = {:?}", r);
        }
        Err(e) => {
            println!("set ns failed = {:?}", e);
        }
    }

    let output = Command::new("sh").arg("-c").arg("ip a").output();
    println!("ip a = {:?}", output);
}

fn main() {
    ns_test();
    apply();
    err_test();
}