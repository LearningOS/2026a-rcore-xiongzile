//! Process management syscalls
use crate::{
    syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TRACE, SYSCALL_YIELD},
    task::{cur_task, exit_current_and_run_next, suspend_current_and_run_next, TRACED_SYSCALLS},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    unsafe {
        TRACED_SYSCALLS[cur_task()][SYSCALL_EXIT] += 1;
    }
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    unsafe {
        TRACED_SYSCALLS[cur_task()][SYSCALL_YIELD] += 1;
    }
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    unsafe {
        TRACED_SYSCALLS[cur_task()][SYSCALL_GET_TIME] += 1;
    }
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    unsafe {
        TRACED_SYSCALLS[cur_task()][SYSCALL_TRACE] += 1;
    }
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            // read
            return unsafe { *(_id as *const isize) };
        }
        1 => {
            // write
            unsafe { *(_id as *mut usize) = _data };
            return 0;
        }
        2 => {
            // trace
            return unsafe { TRACED_SYSCALLS[cur_task()][_id].try_into().unwrap() };
        }
        3_usize.. => todo!(),
    }
}
