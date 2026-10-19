//! make_directional_channel_793.rs — regression tests for issue #793:
//! make(<-chan T) and make(chan<- T) must be rejected with HYB-E2005 (BUILDABLE).

use hy_check::{check_file, Verdict};
use hy_diag::Severity;
use hy_syntax::parse_file;

fn check(src: &str) -> hy_check::CheckResult {
    let ast = parse_file("test.hy", src).expect("test snippet must parse");
    check_file(&ast)
}

fn error_codes(r: &hy_check::CheckResult) -> Vec<String> {
    r.diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .map(|d| d.code.clone())
        .collect()
}

#[test]
fn make_directional_channels_rejected_793() {
    let cases = [
        // direct receive-only channel with size
        (
            r#"
fn main() {
    ch := make(<-chan int, 1)
    fmt.println("{ch}")
}
"#,
            "cannot make receive-only channel `<-chan int`",
        ),
        // direct send-only channel with size
        (
            r#"
fn main() {
    ch := make(chan<- int, 1)
    fmt.println("{ch}")
}
"#,
            "cannot make send-only channel `chan<- int`",
        ),
        // direct receive-only channel unbuffered (1 argument)
        (
            r#"
fn main() {
    ch := make(<-chan int)
    fmt.println("{ch}")
}
"#,
            "cannot make receive-only channel `<-chan int`",
        ),
        // direct send-only channel unbuffered (1 argument)
        (
            r#"
fn main() {
    ch := make(chan<- int)
    fmt.println("{ch}")
}
"#,
            "cannot make send-only channel `chan<- int`",
        ),
        // receive-only type alias
        (
            r#"
type in_chan = <-chan int
fn main() {
    ch := make(in_chan, 1)
    fmt.println("{ch}")
}
"#,
            "cannot make receive-only channel `in_chan`",
        ),
        // send-only type alias
        (
            r#"
type out_chan = chan<- int
fn main() {
    ch := make(out_chan, 1)
    fmt.println("{ch}")
}
"#,
            "cannot make send-only channel `out_chan`",
        ),
        // multi-level type alias
        (
            r#"
type in1 = <-chan int
type in2 = in1
fn main() {
    ch := make(in2, 1)
    fmt.println("{ch}")
}
"#,
            "cannot make receive-only channel `in2`",
        ),
    ];

    for (src, expected_msg_part) in cases {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected reject for:\n{src}");
        assert!(
            error_codes(&r).contains(&"HYB-E2005".to_string()),
            "expected HYB-E2005, got {:?} for:\n{src}",
            r.diagnostics
        );
        let diag = r
            .diagnostics
            .iter()
            .find(|d| d.code == "HYB-E2005")
            .expect("must find HYB-E2005 diagnostic");
        assert!(
            diag.message.contains(expected_msg_part),
            "expected diagnostic to contain {:?}, got {:?}",
            expected_msg_part,
            diag.message
        );
    }
}

#[test]
fn make_bidirectional_channels_accepted_controls_793() {
    let cases = [
        // bidirectional channel with buffer
        r#"
fn main() {
    ch := make(chan int, 1)
    fmt.println("{ch}")
}
"#,
        // bidirectional channel unbuffered
        r#"
fn main() {
    ch := make(chan int)
    fmt.println("{ch}")
}
"#,
        // bidirectional channel alias
        r#"
type my_chan = chan int
fn main() {
    ch := make(my_chan, 1)
    fmt.println("{ch}")
}
"#,
        // bidirectional channel carrying directional channel elements
        r#"
fn main() {
    ch1 := make(chan <-chan int, 1)
    ch2 := make(chan chan<- int, 1)
    fmt.println("{ch1} {ch2}")
}
"#,
    ];

    for src in cases {
        let r = check(src);
        assert_eq!(
            r.verdict,
            Verdict::Accept,
            "unexpected diagnostics for:\n{src}\n{:?}",
            r.diagnostics
        );
    }
}
