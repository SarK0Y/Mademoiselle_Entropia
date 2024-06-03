#[derive(Copy, Clone)]
pub enum director{
    warn,
    stop,
    ok,
}
pub(crate) fn signal(state: Option<director>) -> director{
    static mut status: director = director::ok;
    if let Some(x) = state{
        unsafe{status = x}
    } unsafe{status}
}