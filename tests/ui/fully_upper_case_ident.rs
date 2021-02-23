#![warn(clippy::fully_upper_case_idents)]
#![allow(clippy::upper_case_acronyms)]

struct HTTPResponse; // not linted

struct CString; // not linted

enum Flags {
    NS,  // not linted, < 3 chars
    CWR, // linted
    ECE,
    URG,
    ACK,
    PSH,
    RST,
    SYN,
    FIN,
    THISISLINTED,
    NOTLINTEd, // not linted
    nOTLINTED,
    NOTLiNTED,
}

struct GCCLLVMSomething; //t no linted

struct AAAA; // linted
fn main() {}
