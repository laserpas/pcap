/* automatically generated by rust-bindgen */
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u_char = ::libc::c_uchar;
pub type __u_short = ::libc::c_ushort;
pub type __u_int = ::libc::c_uint;
pub type __u_long = ::libc::c_ulong;
pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_long;
pub type __uint64_t = ::libc::c_ulong;
pub type __quad_t = ::libc::c_long;
pub type __u_quad_t = ::libc::c_ulong;
pub type __dev_t = ::libc::c_ulong;
pub type __uid_t = ::libc::c_uint;
pub type __gid_t = ::libc::c_uint;
pub type __ino_t = ::libc::c_ulong;
pub type __ino64_t = ::libc::c_ulong;
pub type __mode_t = ::libc::c_uint;
pub type __nlink_t = ::libc::c_ulong;
pub type __off_t = ::libc::c_long;
pub type __off64_t = ::libc::c_long;
pub type __pid_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed1 {
    pub __val: [::libc::c_int; 2usize],
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type __fsid_t = Struct_Unnamed1;
pub type __clock_t = ::libc::c_long;
pub type __rlim_t = ::libc::c_ulong;
pub type __rlim64_t = ::libc::c_ulong;
pub type __id_t = ::libc::c_uint;
pub type __time_t = ::libc::c_long;
pub type __useconds_t = ::libc::c_uint;
pub type __suseconds_t = ::libc::c_long;
pub type __daddr_t = ::libc::c_int;
pub type __key_t = ::libc::c_int;
pub type __clockid_t = ::libc::c_int;
pub type __timer_t = *mut ::libc::c_void;
pub type __blksize_t = ::libc::c_long;
pub type __blkcnt_t = ::libc::c_long;
pub type __blkcnt64_t = ::libc::c_long;
pub type __fsblkcnt_t = ::libc::c_ulong;
pub type __fsblkcnt64_t = ::libc::c_ulong;
pub type __fsfilcnt_t = ::libc::c_ulong;
pub type __fsfilcnt64_t = ::libc::c_ulong;
pub type __fsword_t = ::libc::c_long;
pub type __ssize_t = ::libc::c_long;
pub type __syscall_slong_t = ::libc::c_long;
pub type __syscall_ulong_t = ::libc::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::libc::c_char;
pub type __intptr_t = ::libc::c_long;
pub type __socklen_t = ::libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type size_t = ::libc::size_t;
pub type ulong = ::libc::c_ulong;
pub type ushort = ::libc::c_ushort;
pub type _uint = ::libc::c_uint;
pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_long;
pub type u_int8_t = ::libc::c_uchar;
pub type u_int16_t = ::libc::c_ushort;
pub type u_int32_t = ::libc::c_uint;
pub type u_int64_t = ::libc::c_ulong;
pub type register_t = ::libc::c_long;
pub type __sig_atomic_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed2 {
    pub __val: [::libc::c_ulong; 16usize],
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Struct_Unnamed2 { unsafe { ::std::mem::zeroed() } }
}
pub type __sigset_t = Struct_Unnamed2;
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
impl ::std::default::Default for Struct_timespec {
    fn default() -> Struct_timespec { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
impl ::std::default::Default for Struct_timeval {
    fn default() -> Struct_timeval { unsafe { ::std::mem::zeroed() } }
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::libc::c_long;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed3 {
    pub __fds_bits: [__fd_mask; 16usize],
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Struct_Unnamed3 { unsafe { ::std::mem::zeroed() } }
}
pub type fd_set = Struct_Unnamed3;
pub type fd_mask = __fd_mask;
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type pthread_t = ::libc::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_pthread_attr_t {
    pub _bindgen_data_: [u64; 7usize],
}
impl Union_pthread_attr_t {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 56usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_pthread_attr_t {
    fn default() -> Union_pthread_attr_t { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_attr_t = Union_pthread_attr_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct___pthread_internal_list {
    pub __prev: *mut Struct___pthread_internal_list,
    pub __next: *mut Struct___pthread_internal_list,
}
impl ::std::default::Default for Struct___pthread_internal_list {
    fn default() -> Struct___pthread_internal_list {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __pthread_list_t = Struct___pthread_internal_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed4 {
    pub _bindgen_data_: [u64; 5usize],
}
impl Union_Unnamed4 {
    pub unsafe fn __data(&mut self) -> *mut Struct___pthread_mutex_s {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 40usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed4 {
    fn default() -> Union_Unnamed4 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct___pthread_mutex_s {
    pub __lock: ::libc::c_int,
    pub __count: ::libc::c_uint,
    pub __owner: ::libc::c_int,
    pub __nusers: ::libc::c_uint,
    pub __kind: ::libc::c_int,
    pub __spins: ::libc::c_short,
    pub __elision: ::libc::c_short,
    pub __list: __pthread_list_t,
}
impl ::std::default::Default for Struct___pthread_mutex_s {
    fn default() -> Struct___pthread_mutex_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type pthread_mutex_t = Union_Unnamed4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed5 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed5 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 4usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_int {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed5 {
    fn default() -> Union_Unnamed5 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_mutexattr_t = Union_Unnamed5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed6 {
    pub _bindgen_data_: [u64; 6usize],
}
impl Union_Unnamed6 {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed7 {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 48usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_longlong {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed6 {
    fn default() -> Union_Unnamed6 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed7 {
    pub __lock: ::libc::c_int,
    pub __futex: ::libc::c_uint,
    pub __total_seq: ::libc::c_ulonglong,
    pub __wakeup_seq: ::libc::c_ulonglong,
    pub __woken_seq: ::libc::c_ulonglong,
    pub __mutex: *mut ::libc::c_void,
    pub __nwaiters: ::libc::c_uint,
    pub __broadcast_seq: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Struct_Unnamed7 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_cond_t = Union_Unnamed6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed8 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed8 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 4usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_int {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed8 {
    fn default() -> Union_Unnamed8 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_condattr_t = Union_Unnamed8;
pub type pthread_key_t = ::libc::c_uint;
pub type pthread_once_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed9 {
    pub _bindgen_data_: [u64; 7usize],
}
impl Union_Unnamed9 {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed10 {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 56usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed9 {
    fn default() -> Union_Unnamed9 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed10 {
    pub __lock: ::libc::c_int,
    pub __nr_readers: ::libc::c_uint,
    pub __readers_wakeup: ::libc::c_uint,
    pub __writer_wakeup: ::libc::c_uint,
    pub __nr_readers_queued: ::libc::c_uint,
    pub __nr_writers_queued: ::libc::c_uint,
    pub __writer: ::libc::c_int,
    pub __shared: ::libc::c_int,
    pub __pad1: ::libc::c_ulong,
    pub __pad2: ::libc::c_ulong,
    pub __flags: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Struct_Unnamed10 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_rwlock_t = Union_Unnamed9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed11 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed11 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 8usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed11 {
    fn default() -> Union_Unnamed11 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_rwlockattr_t = Union_Unnamed11;
pub type pthread_spinlock_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed12 {
    pub _bindgen_data_: [u64; 4usize],
}
impl Union_Unnamed12 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 32usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed12 {
    fn default() -> Union_Unnamed12 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_barrier_t = Union_Unnamed12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed13 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed13 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 4usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_int {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed13 {
    fn default() -> Union_Unnamed13 { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_barrierattr_t = Union_Unnamed13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_timezone {
    pub tz_minuteswest: ::libc::c_int,
    pub tz_dsttime: ::libc::c_int,
}
impl ::std::default::Default for Struct_timezone {
    fn default() -> Struct_timezone { unsafe { ::std::mem::zeroed() } }
}
pub type __timezone_ptr_t = *mut Struct_timezone;
pub type Enum___itimer_which = ::libc::c_uint;
pub const ITIMER_REAL: ::libc::c_uint = 0;
pub const ITIMER_VIRTUAL: ::libc::c_uint = 1;
pub const ITIMER_PROF: ::libc::c_uint = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_itimerval {
    pub it_interval: Struct_timeval,
    pub it_value: Struct_timeval,
}
impl ::std::default::Default for Struct_itimerval {
    fn default() -> Struct_itimerval { unsafe { ::std::mem::zeroed() } }
}
pub type __itimer_which_t = ::libc::c_int;
pub type bpf_int32 = ::libc::c_int;
pub type bpf_u_int32 = u_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_bpf_program {
    pub bf_len: u_int,
    pub bf_insns: *mut Struct_bpf_insn,
}
impl ::std::default::Default for Struct_bpf_program {
    fn default() -> Struct_bpf_program { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_bpf_insn {
    pub code: u_short,
    pub jt: u_char,
    pub jf: u_char,
    pub k: bpf_u_int32,
}
impl ::std::default::Default for Struct_bpf_insn {
    fn default() -> Struct_bpf_insn { unsafe { ::std::mem::zeroed() } }
}
pub type FILE = Struct__IO_FILE;
pub type __FILE = Struct__IO_FILE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed14 {
    pub __count: ::libc::c_int,
    pub __value: Union_Unnamed15,
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Struct_Unnamed14 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed15 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed15 {
    pub unsafe fn __wch(&mut self) -> *mut ::libc::c_uint {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __wchb(&mut self) -> *mut [::libc::c_char; 4usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed15 {
    fn default() -> Union_Unnamed15 { unsafe { ::std::mem::zeroed() } }
}
pub type __mbstate_t = Struct_Unnamed14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed16 {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Struct_Unnamed16 { unsafe { ::std::mem::zeroed() } }
}
pub type _G_fpos_t = Struct_Unnamed16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed17 {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
impl ::std::default::Default for Struct_Unnamed17 {
    fn default() -> Struct_Unnamed17 { unsafe { ::std::mem::zeroed() } }
}
pub type _G_fpos64_t = Struct_Unnamed17;
pub type va_list = __gnuc_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub enum Struct__IO_jump_t { }
pub type _IO_lock_t = ::libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__IO_marker {
    pub _next: *mut Struct__IO_marker,
    pub _sbuf: *mut Struct__IO_FILE,
    pub _pos: ::libc::c_int,
}
impl ::std::default::Default for Struct__IO_marker {
    fn default() -> Struct__IO_marker { unsafe { ::std::mem::zeroed() } }
}
pub type Enum___codecvt_result = ::libc::c_uint;
pub const __codecvt_ok: ::libc::c_uint = 0;
pub const __codecvt_partial: ::libc::c_uint = 1;
pub const __codecvt_error: ::libc::c_uint = 2;
pub const __codecvt_noconv: ::libc::c_uint = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__IO_FILE {
    pub _flags: ::libc::c_int,
    pub _IO_read_ptr: *mut ::libc::c_char,
    pub _IO_read_end: *mut ::libc::c_char,
    pub _IO_read_base: *mut ::libc::c_char,
    pub _IO_write_base: *mut ::libc::c_char,
    pub _IO_write_ptr: *mut ::libc::c_char,
    pub _IO_write_end: *mut ::libc::c_char,
    pub _IO_buf_base: *mut ::libc::c_char,
    pub _IO_buf_end: *mut ::libc::c_char,
    pub _IO_save_base: *mut ::libc::c_char,
    pub _IO_backup_base: *mut ::libc::c_char,
    pub _IO_save_end: *mut ::libc::c_char,
    pub _markers: *mut Struct__IO_marker,
    pub _chain: *mut Struct__IO_FILE,
    pub _fileno: ::libc::c_int,
    pub _flags2: ::libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::libc::c_ushort,
    pub _vtable_offset: ::libc::c_char,
    pub _shortbuf: [::libc::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::libc::c_void,
    pub __pad2: *mut ::libc::c_void,
    pub __pad3: *mut ::libc::c_void,
    pub __pad4: *mut ::libc::c_void,
    pub __pad5: size_t,
    pub _mode: ::libc::c_int,
    pub _unused2: [::libc::c_char; 20usize],
}
impl ::std::default::Default for Struct__IO_FILE {
    fn default() -> Struct__IO_FILE { unsafe { ::std::mem::zeroed() } }
}
pub type _IO_FILE = Struct__IO_FILE;
pub enum Struct__IO_FILE_plus { }
pub type __io_read_fn =
    extern "C" fn
        (__cookie: *mut ::libc::c_void, __buf: *mut ::libc::c_char,
         __nbytes: size_t) -> __ssize_t;
pub type __io_write_fn =
    extern "C" fn
        (__cookie: *mut ::libc::c_void, __buf: *const ::libc::c_char,
         __n: size_t) -> __ssize_t;
pub type __io_seek_fn =
    extern "C" fn
        (__cookie: *mut ::libc::c_void, __pos: *mut __off64_t,
         __w: ::libc::c_int) -> ::libc::c_int;
pub type __io_close_fn =
    extern "C" fn(__cookie: *mut ::libc::c_void) -> ::libc::c_int;
pub type fpos_t = _G_fpos_t;
pub enum Struct_pcap { }
pub type pcap_t = Struct_pcap;
pub enum Struct_pcap_dumper { }
pub type pcap_dumper_t = Struct_pcap_dumper;
pub type pcap_if_t = Struct_pcap_if;
pub type pcap_addr_t = Struct_pcap_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_pcap_file_header {
    pub magic: bpf_u_int32,
    pub version_major: u_short,
    pub version_minor: u_short,
    pub thiszone: bpf_int32,
    pub sigfigs: bpf_u_int32,
    pub snaplen: bpf_u_int32,
    pub linktype: bpf_u_int32,
}
impl ::std::default::Default for Struct_pcap_file_header {
    fn default() -> Struct_pcap_file_header {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Enum_Unnamed18 = ::libc::c_uint;
pub const PCAP_D_INOUT: ::libc::c_uint = 0;
pub const PCAP_D_IN: ::libc::c_uint = 1;
pub const PCAP_D_OUT: ::libc::c_uint = 2;
pub type pcap_direction_t = Enum_Unnamed18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_pcap_pkthdr {
    pub ts: Struct_timeval,
    pub caplen: bpf_u_int32,
    pub len: bpf_u_int32,
}
impl ::std::default::Default for Struct_pcap_pkthdr {
    fn default() -> Struct_pcap_pkthdr { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents a packet header provided by pcap, including the timeval, caplen and len.
pub struct PacketHeader {
    pub ts: ::libc::timeval,
    pub caplen: u32,
    pub len: u32,
}

impl ::std::fmt::Debug for PacketHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PacketHeader {{ ts: {}.{:06}, caplen: {}, len: {} }}",
               self.ts.tv_sec, self.ts.tv_usec, self.caplen, self.len)
    }
}

#[test]
fn packet_hdr_eq() {
    use std::mem::size_of;

    assert_eq!(size_of::<PacketHeader>(), size_of::<Struct_pcap_pkthdr>())
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_pcap_stat {
    pub ps_recv: u_int,
    pub ps_drop: u_int,
    pub ps_ifdrop: u_int,
}
impl ::std::default::Default for Struct_pcap_stat {
    fn default() -> Struct_pcap_stat { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_pcap_if {
    pub next: *mut Struct_pcap_if,
    pub name: *mut ::libc::c_char,
    pub description: *mut ::libc::c_char,
    pub addresses: *mut Struct_pcap_addr,
    pub flags: bpf_u_int32,
}
impl ::std::default::Default for Struct_pcap_if {
    fn default() -> Struct_pcap_if { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_sockaddr { }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_pcap_addr {
    pub next: *mut Struct_pcap_addr,
    pub addr: *mut Struct_sockaddr,
    pub netmask: *mut Struct_sockaddr,
    pub broadaddr: *mut Struct_sockaddr,
    pub dstaddr: *mut Struct_sockaddr,
}
impl ::std::default::Default for Struct_pcap_addr {
    fn default() -> Struct_pcap_addr { unsafe { ::std::mem::zeroed() } }
}
pub type pcap_handler =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut u_char,
                               arg2: *const Struct_pcap_pkthdr,
                               arg3: *const u_char) -> ()>;
pub type __builtin_va_list = [__va_list_tag; 1usize];
pub type __va_list_tag = Struct___va_list_tag;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct___va_list_tag {
    fn default() -> Struct___va_list_tag { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn pcap_lookupdev(arg1: *mut ::libc::c_char) -> *mut ::libc::c_char;
    // pub fn pcap_lookupnet(arg1: *const ::libc::c_char, arg2: *mut bpf_u_int32,
    //                       arg3: *mut bpf_u_int32, arg4: *mut ::libc::c_char)
    //  -> ::libc::c_int;
    pub fn pcap_create(arg1: *const ::libc::c_char, arg2: *mut ::libc::c_char)
     -> *mut pcap_t;
    pub fn pcap_set_snaplen(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn pcap_set_promisc(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    // pub fn pcap_can_set_rfmon(arg1: *mut pcap_t) -> ::libc::c_int;
    pub fn pcap_set_timeout(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn pcap_set_tstamp_type(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    // pub fn pcap_set_immediate_mode(arg1: *mut pcap_t, arg2: ::libc::c_int)
    //  -> ::libc::c_int;
    pub fn pcap_set_buffer_size(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn pcap_set_tstamp_precision(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    // pub fn pcap_get_tstamp_precision(arg1: *mut pcap_t) -> ::libc::c_int;
    pub fn pcap_activate(arg1: *mut pcap_t) -> ::libc::c_int;
    // pub fn pcap_list_tstamp_types(arg1: *mut pcap_t,
    //                               arg2: *mut *mut ::libc::c_int)
    //  -> ::libc::c_int;
    // pub fn pcap_free_tstamp_types(arg1: *mut ::libc::c_int) -> ();
    // pub fn pcap_tstamp_type_name_to_val(arg1: *const ::libc::c_char)
    //  -> ::libc::c_int;
    // pub fn pcap_tstamp_type_val_to_name(arg1: ::libc::c_int)
    //  -> *const ::libc::c_char;
    // pub fn pcap_tstamp_type_val_to_description(arg1: ::libc::c_int)
    //  -> *const ::libc::c_char;
    // pub fn pcap_open_live(arg1: *const ::libc::c_char, arg2: ::libc::c_int,
    //                       arg3: ::libc::c_int, arg4: ::libc::c_int,
    //                       arg5: *mut ::libc::c_char) -> *mut pcap_t;
    pub fn pcap_open_dead(arg1: ::libc::c_int, arg2: ::libc::c_int)
      -> *mut pcap_t;
    pub fn pcap_open_dead_with_tstamp_precision(arg1: ::libc::c_int,
                                                 arg2: ::libc::c_int,
                                                 arg3: u_int) -> *mut pcap_t;
    pub fn pcap_open_offline_with_tstamp_precision(arg1:
                                                       *const ::libc::c_char,
                                                   arg2: u_int,
                                                   arg3: *mut ::libc::c_char)
     -> *mut pcap_t;
    pub fn pcap_open_offline(arg1: *const ::libc::c_char,
                             arg2: *mut ::libc::c_char) -> *mut pcap_t;
    // pub fn pcap_fopen_offline_with_tstamp_precision(arg1: *mut FILE,
    //                                                 arg2: u_int,
    //                                                 arg3: *mut ::libc::c_char)
    //  -> *mut pcap_t;
    // pub fn pcap_fopen_offline(arg1: *mut FILE, arg2: *mut ::libc::c_char)
    //  -> *mut pcap_t;
    pub fn pcap_close(arg1: *mut pcap_t) -> ();
    // pub fn pcap_loop(arg1: *mut pcap_t, arg2: ::libc::c_int,
    //                  arg3: pcap_handler, arg4: *mut u_char) -> ::libc::c_int;
    // pub fn pcap_dispatch(arg1: *mut pcap_t, arg2: ::libc::c_int,
    //                      arg3: pcap_handler, arg4: *mut u_char)
    //  -> ::libc::c_int;
    // pub fn pcap_next(arg1: *mut pcap_t, arg2: *mut Struct_pcap_pkthdr)
    //  -> *const u_char;
    pub fn pcap_next_ex(arg1: *mut pcap_t, arg2: *mut *mut Struct_pcap_pkthdr,
                        arg3: *mut *const u_char) -> ::libc::c_int;
    // pub fn pcap_breakloop(arg1: *mut pcap_t) -> ();
    pub fn pcap_stats(arg1: *mut pcap_t, arg2: *mut Struct_pcap_stat)
     -> ::libc::c_int;
    pub fn pcap_setfilter(arg1: *mut pcap_t, arg2: *mut Struct_bpf_program)
     -> ::libc::c_int;
    pub fn pcap_setdirection(arg1: *mut pcap_t, arg2: pcap_direction_t)
     -> ::libc::c_int;
    // pub fn pcap_getnonblock(arg1: *mut pcap_t, arg2: *mut ::libc::c_char)
    //  -> ::libc::c_int;
    // pub fn pcap_setnonblock(arg1: *mut pcap_t, arg2: ::libc::c_int,
    //                         arg3: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn pcap_sendpacket(arg1: *mut pcap_t, arg2: *const u_char,
                           arg3: ::libc::c_int) -> ::libc::c_int;
    // pub fn pcap_statustostr(arg1: ::libc::c_int) -> *const ::libc::c_char;
    // pub fn pcap_strerror(arg1: ::libc::c_int) -> *const ::libc::c_char;
    pub fn pcap_geterr(arg1: *mut pcap_t) -> *mut ::libc::c_char;
    // pub fn pcap_perror(arg1: *mut pcap_t, arg2: *mut ::libc::c_char) -> ();
    pub fn pcap_compile(arg1: *mut pcap_t, arg2: *mut Struct_bpf_program,
                        arg3: *const ::libc::c_char, arg4: ::libc::c_int,
                        arg5: bpf_u_int32) -> ::libc::c_int;
    // pub fn pcap_compile_nopcap(arg1: ::libc::c_int, arg2: ::libc::c_int,
    //                            arg3: *mut Struct_bpf_program,
    //                            arg4: *const ::libc::c_char,
    //                            arg5: ::libc::c_int, arg6: bpf_u_int32)
    //  -> ::libc::c_int;
    pub fn pcap_freecode(arg1: *mut Struct_bpf_program) -> ();
    // pub fn pcap_offline_filter(arg1: *const Struct_bpf_program,
    //                            arg2: *const Struct_pcap_pkthdr,
    //                            arg3: *const u_char) -> ::libc::c_int;
    pub fn pcap_datalink(arg1: *mut pcap_t) -> ::libc::c_int;
    // pub fn pcap_datalink_ext(arg1: *mut pcap_t) -> ::libc::c_int;
    pub fn pcap_list_datalinks(arg1: *mut pcap_t,
                               arg2: *mut *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn pcap_set_datalink(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn pcap_free_datalinks(arg1: *mut ::libc::c_int) -> ();
    // pub fn pcap_datalink_name_to_val(arg1: *const ::libc::c_char)
    //  -> ::libc::c_int;
    pub fn pcap_datalink_val_to_name(arg1: ::libc::c_int)
     -> *const ::libc::c_char;
    pub fn pcap_datalink_val_to_description(arg1: ::libc::c_int)
     -> *const ::libc::c_char;
    // pub fn pcap_snapshot(arg1: *mut pcap_t) -> ::libc::c_int;
    // pub fn pcap_is_swapped(arg1: *mut pcap_t) -> ::libc::c_int;
    // pub fn pcap_major_version(arg1: *mut pcap_t) -> ::libc::c_int;
    // pub fn pcap_minor_version(arg1: *mut pcap_t) -> ::libc::c_int;
    // pub fn pcap_file(arg1: *mut pcap_t) -> *mut FILE;
    pub fn pcap_fileno(arg1: *mut pcap_t) -> ::libc::c_int;
    pub fn pcap_dump_open(arg1: *mut pcap_t, arg2: *const ::libc::c_char)
     -> *mut pcap_dumper_t;
    // pub fn pcap_dump_fopen(arg1: *mut pcap_t, fp: *mut FILE)
    //  -> *mut pcap_dumper_t;
    // pub fn pcap_dump_file(arg1: *mut pcap_dumper_t) -> *mut FILE;
    // pub fn pcap_dump_ftell(arg1: *mut pcap_dumper_t) -> ::libc::c_long;
    // pub fn pcap_dump_flush(arg1: *mut pcap_dumper_t) -> ::libc::c_int;
    pub fn pcap_dump_close(arg1: *mut pcap_dumper_t) -> ();
    pub fn pcap_dump(arg1: *mut u_char, arg2: *const Struct_pcap_pkthdr,
                     arg3: *const u_char) -> ();
    pub fn pcap_findalldevs(arg1: *mut *mut pcap_if_t,
                             arg2: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn pcap_freealldevs(arg1: *mut pcap_if_t) -> ();
    // pub fn pcap_lib_version() -> *const ::libc::c_char;
    // pub fn bpf_image(arg1: *const Struct_bpf_insn, arg2: ::libc::c_int)
    //  -> *mut ::libc::c_char;
    // pub fn bpf_dump(arg1: *const Struct_bpf_program, arg2: ::libc::c_int)
    //  -> ();
    // pub fn pcap_get_selectable_fd(arg1: *mut pcap_t) -> ::libc::c_int;
}




#[cfg(target_os = "windows")]
#[link(name = "wpcap")]
extern {}

#[cfg(not(target_os = "windows"))]
#[link(name = "pcap")]
extern {
    // pub fn pcap_inject(arg1: *mut pcap_t, arg2: *const ::libc::c_void,
    //                    arg3: size_t) -> ::libc::c_int;

    pub fn pcap_set_rfmon(arg1: *mut pcap_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
}
