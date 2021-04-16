use std::cell::RefCell;
use std::rc::Rc;

use super::errno::Error;
use super::netns_linux::NetNS;

#[derive(Debug)]
pub enum ExecStatus {
    Success(i32),
    Failed(Error),
}

pub fn apply_in_ns<F: Fn() -> ExecStatus>(pid: i32, f: F) -> ExecStatus {
    let current_ns = NetNS::get();
    if !current_ns.is_ok() {
        return ExecStatus::Failed(current_ns.err().unwrap());
    }

    let cn = Rc::new(RefCell::new(current_ns.unwrap()));
    let ns = NetNS::get_from_process(pid);
    if ns.is_ok() {
        let temp = ns.unwrap();
        let result = NetNS::set(&temp);
        match result {
            Ok(_v) => {}
            Err(e) => {
                return ExecStatus::Failed(e);
            }
        }

        let code_raw: i32 = 0;
        let code = Rc::new(RefCell::new(code_raw));

        let status = call_fn(f);
        match status {
            ExecStatus::Success(c) => {
                *code.borrow_mut() = c;
            }
            ExecStatus::Failed(e) => {
                return ExecStatus::Failed(e);
            }
        }

        let result1 = NetNS::set(&cn.borrow());
        match result1 {
            Ok(_v) => ExecStatus::Success(*code.borrow()),
            Err(e) => {
                return ExecStatus::Failed(e);
            }
        }
    } else {
        return ExecStatus::Failed(ns.err().unwrap());
    }
}

fn call_fn<F: Fn() -> ExecStatus>(f: F) -> ExecStatus {
    f()
}
