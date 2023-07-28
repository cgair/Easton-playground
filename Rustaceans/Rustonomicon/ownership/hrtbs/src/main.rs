// [Higher-Rank Trait Bounds (HRTBs)](https://doc.rust-lang.org/nomicon/hrtb.html)

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

/*
How on earth are we supposed to express the lifetimes on F's trait bound? 
This job requires The Magic of Higher-Rank Trait Bounds (HRTBs). The way we desugar this is as follows:
where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
for<'a> can be read as "for all choices of 'a", and basically produces an infinite list of trait bounds that F must satisfy.
*/
impl<F> Closure<F>
    // where F: Fn(&'??? (u8, u16)) -> &'??? u8,
    // where F: for<'a> Fn(&'a (u8, u16)) -> &'a u8,
    where F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

// fn do_it<'b>(data: &'b (u8, u16)) -> &'b u8 { &'b data.0 }
fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

fn main() {
    // 'x: {
    let clo = Closure { data: (0, 1), func: do_it };
    println!("{:p}", &clo.data.0);
    println!("{:p}", clo.call());
    println!("{}", clo.call());
    // }
}