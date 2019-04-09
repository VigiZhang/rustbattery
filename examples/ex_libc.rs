use libc;

struct People {
    name: String,
    age: u8,
}

fn main() {
    let shm_id = unsafe {
        libc::shmget(libc::IPC_PRIVATE, 4096, libc::IPC_CREAT)
    };
    if shm_id == -1 {
        println!("shmget error");
    }
    let p_map: *mut People = unsafe {
        libc::shmat(shm_id, std::ptr::null(), 0) as *mut People
    };
}
