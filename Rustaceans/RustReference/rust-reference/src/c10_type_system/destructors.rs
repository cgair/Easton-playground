struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

/**
 * [Destructors](https://doc.rust-lang.org/reference/destructors.html#destructors)
 * The destructor of a type T consists of:
 * 1. If T: Drop, calling <T as std::ops::Drop>::drop
 * 2. Recursively running the destructor of all of its fields.
 *    The fields of a struct are dropped in declaration order.
 *    The fields of the active enum variant are dropped in declaration order.
 *    The fields of a tuple are dropped in order.
 *    The elements of an array or owned slice are dropped from the first element to the last.
 *    The variables that a closure captures by move are dropped in an unspecified order.
 *    Trait objects run the destructor of the underlying type.
 *    Other types don't result in any further drops.
 * 
 * If a destructor must be run manually, such as when implementing your own smart pointer, std::ptr::drop_in_place can be used.
 */
#[test]
fn run_destructor() {}


/**
 * [Drop scopes](https://doc.rust-lang.org/reference/destructors.html#drop-scopes)
 * Each variable or [temporary](https://doc.rust-lang.org/reference/expressions.html#temporaries) is associated to a drop scope. 
 * When control flow leaves a drop scope 
 * all variables associated to that scope are dropped in reverse order of 
 * declaration (for variables) or creation (for temporaries).
 */

// Drops `y`, then the second parameter, then `x`, then the first parameter
fn patterns_in_parameters(
    (x, _): (PrintOnDrop, PrintOnDrop),
    (_, y): (PrintOnDrop, PrintOnDrop),
) {}

fn patterns_in_parameters2(
    (x, y): (PrintOnDrop, PrintOnDrop),
    (p, q): (PrintOnDrop, PrintOnDrop),
) {}

#[test]
fn drop_scope() {
    // Scopes of function parameters
    // 
    // drop order is 3 2 0 1
    patterns_in_parameters(
        (PrintOnDrop("0"), PrintOnDrop("1")),
        (PrintOnDrop("2"), PrintOnDrop("3")),
    );
    // drop order is 3 2 1 0
    patterns_in_parameters2(
        (PrintOnDrop("0"), PrintOnDrop("1")),
        (PrintOnDrop("2"), PrintOnDrop("3")),
    );

    // Scopes of local variables
    // 
    let declared_first = PrintOnDrop("Dropped last in outer scope");
    {
        let declared_in_block = PrintOnDrop("Dropped in inner scope");
    }
    let declared_last = PrintOnDrop("Dropped first in outer scope");

    // Temporary scopes
    // 
    let local_var = PrintOnDrop("local var");
    // Dropped once the condition has been evaluated
    if PrintOnDrop("If condition").0 == "If condition" {
        // Dropped at the end of the block
        PrintOnDrop("If body").0
    } else {
        unreachable!()
    };

    // Dropped at the end of the statement
    (PrintOnDrop("first operand").0 == ""
    // Dropped at the )
    || PrintOnDrop("second operand").0 == "")
    // Dropped at the end of the expression
    || PrintOnDrop("third operand").0 == "";

    // Dropped at the end of the function, after local variables.
    // Changing this to a statement containing a return expression would make the
    // temporary be dropped before the local variables. Binding to a variable
    // which is then returned would also make the temporary be dropped first.
    match PrintOnDrop("Matched value in final expression") {
        // Dropped once the condition has been evaluated
        _ if PrintOnDrop("guard condition").0 == "" => (),
        _ => (),
    }
}