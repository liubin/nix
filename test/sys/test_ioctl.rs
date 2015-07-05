use nix::sys::ioctl::*;

// See C code for source of values for op calculations:
// https://gist.github.com/posborne/83ea6880770a1aef332e

#[test]
fn test_op_none() {
    assert_eq!(io!(b'q', 10), 0x0000710A);
    assert_eq!(io!(b'a', 255), 0x000061FF);
}

#[test]
fn test_op_write() {
    assert_eq!(iow!(b'z', 10, 1), 0x40017A0A);
    assert_eq!(iow!(b'z', 10, 512), 0x42007A0A);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_op_write_64() {
    assert_eq!(iow!(b'z', 10, (1 as u64) << 32), 0x40007A0A);
}

#[test]
fn test_op_read() {
    assert_eq!(ior!(b'z', 10, 1), 0x80017A0A);
    assert_eq!(ior!(b'z', 10, 512), 0x82007A0A);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_op_read_64() {
    assert_eq!(ior!(b'z', 10, (1 as u64) << 32), 0x80007A0A);
}

#[test]
fn test_op_read_write() {
    assert_eq!(iorw!(b'z', 10, 1), 0xC0017A0A);
    assert_eq!(iorw!(b'z', 10, 512), 0xC2007A0A);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_op_read_write_64() {
    assert_eq!(iorw!(b'z', 10, (1 as u64) << 32), 0xC0007A0A);
}
