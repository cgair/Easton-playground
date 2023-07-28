/**
 * Often it is necessary to prepare and process some data, but after that data are only inspected and never modified.
 * 
 * It can be done either by processing data within a nested block or by redefining the variable. (redefining the mutable variable as immutable)
 */

// Say, vector must be sorted before usage.
fn redefining_mut_as_immut() {
    // Using nested block (Preferred)
    let data = {
        let mut data = vec![0, 1, 2, 3, 4, 9, 8, 7, 6, 5];
        data.sort();
        data
    };
    // Here `data` is immutable.

    // OR
    // Using variable rebinding
    let mut data = vec![0, 1, 2, 3, 4, 9, 8, 7, 6, 5];
    data.sort();
    let data = data;
    // Here `data` is immutable.

}

