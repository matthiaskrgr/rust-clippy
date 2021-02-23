#![warn(clippy::upper_case_acronyms)]
#![allow(clippy::fully_upper_case_idents)]

struct HTTPResponse; // linted

struct CString; // not linted

enum Flags {
    NS, // linted
    CWR,
    ECE,
    URG,
    ACK,
    PSH,
    RST,
    SYN,
    FIN,
}

struct GCCLLVMSomething; // linted, beware that lint suggests `GccllvmSomething` instead of `GccLlvmSomething`

fn main() {}
