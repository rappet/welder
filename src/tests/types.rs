use crate::types::{ TaskIdentifier, TaskIdentifierErrorKind };

#[test]
fn parse_task_identifier() {
    let v = [
        // local
        (
            Ok(TaskIdentifier::new("debug_var", "example")),
            "debug_var.example"
        ),
        // scoped
        (
            Ok(TaskIdentifier::with_scope("base", "debug_var", "example")),
            "base.debug_var.example"
        ),
        // wrong
        (
            Err(TaskIdentifierErrorKind::WrongNumberDelimiters.into()),
            "foo"
        ),
        (
            Err(TaskIdentifierErrorKind::WrongNumberDelimiters.into()),
            "foo.bar.buzz.fizz"
        ),
    ];

    for (identifier, s) in &v {
        assert_eq!(identifier, &s.parse());
    }
}
