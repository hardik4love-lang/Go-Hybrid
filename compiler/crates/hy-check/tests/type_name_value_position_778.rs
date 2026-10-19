//! type_name_value_position_778.rs — regression tests for issue #778:
//! a type name used in a value position must be rejected with HYB-E1002 (BUILDABLE).

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
fn type_name_in_value_positions_rejected_778() {
    let cases = [
        // b1_assign: assignment rhs
        r#"
struct holder {
    n int
}
fn main() {
    x := holder
    fmt.println("{x}")
}
"#,
        // b2_arg: call argument
        r#"
struct holder {
    n int
}
fn take(h holder) int {
    return h.n
}
fn main() {
    v := take(holder)
}
"#,
        // b3_arith: binary arithmetic operand
        r#"
struct holder {
    n int
}
fn main() {
    y := holder + 1
}
"#,
        // b4_return: return value
        r#"
struct holder {
    n int
}
fn make_it() holder {
    return holder
}
fn main() {}
"#,
        // primitive type in value position
        r#"
fn main() {
    x := int
}
"#,
        // type alias in value position
        r#"
type my_int = int
fn main() {
    x := my_int
}
"#,
        // enum in value position
        r#"
enum color {
    red,
    green
}
fn main() {
    x := color
}
"#,
        // assignment target
        r#"
struct holder {
    n int
}
fn main() {
    holder = 5
}
"#,
        // struct field access on bare type
        r#"
struct holder {
    n int
}
fn main() {
    x := holder.n
}
"#,
    ];

    for src in cases {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected reject for:\n{src}");
        assert!(
            error_codes(&r).contains(&"HYB-E1002".to_string()),
            "expected HYB-E1002, got {:?} for:\n{src}",
            r.diagnostics
        );
        let diag = r
            .diagnostics
            .iter()
            .find(|d| d.code == "HYB-E1002")
            .expect("must find HYB-E1002 diagnostic");
        assert!(
            diag.message.contains("is a type, not a value")
                || diag.message.contains("is a type, not a variable"),
            "unexpected message: {}",
            diag.message
        );
    }
}

#[test]
fn valid_type_usage_accepted_controls_778() {
    let cases = [
        // Struct literal
        r#"
struct holder {
    n int
}
fn main() {
    x := holder{ n: 1 }
    fmt.println(x.n)
}
"#,
        // Type conversion
        r#"
fn main() {
    x := int(5)
    fmt.println(x)
}
"#,
        // Builtin `new` and `make` type arguments
        r#"
struct holder {
    n int
}
fn main() {
    p := new(holder)
    ch := make(chan int, 1)
    fmt.println(p, ch)
}
"#,
        // Enum variant as value
        r#"
enum color {
    red,
    green
}
fn main() {
    c := red
    c2 := color.red
    fmt.println(c, c2)
}
"#,
        // Shadowed local variable named same as type
        r#"
fn main() {
    var int = 42
    x := int
    fmt.println(x)
}
"#,
        // Function reference as value
        r#"
fn take(x int) int {
    return x
}
fn main() {
    f := take
    fmt.println(f(1))
}
"#,
        // Associated function on struct
        r#"
struct greeting {
    name string
}
fn greeting.build(base string) greeting {
    return greeting{ name: base }
}
fn main() {
    g := greeting.build("Hybrid")
    fmt.println(g.name)
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
