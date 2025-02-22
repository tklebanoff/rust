// This is part of a set of tests exploring the different ways a
// `#[structural_match]` ADT might try to hold a
// non-`#[structural_match]` in hidden manner that lets matches
// through that we had intended to reject.
//
// See discussion on rust-lang/rust#62307 and rust-lang/rust#62339

// run-pass

struct NoDerive(i32);

// This impl makes NoDerive irreflexive.
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

impl Eq for NoDerive { }

#[derive(PartialEq, Eq)]
struct WrapInline<'a>(&'a &'a NoDerive);

const WRAP_DOUBLY_INDIRECT_INLINE: & &WrapInline = & &WrapInline(& & NoDerive(0));

fn main() {
    match WRAP_DOUBLY_INDIRECT_INLINE {
        WRAP_DOUBLY_INDIRECT_INLINE => { panic!("WRAP_DOUBLY_INDIRECT_INLINE matched itself"); }
        //~^ WARN must be annotated with `#[derive(PartialEq, Eq)]`
        //~| WARN will become a hard error in a future release
        _ => { println!("WRAP_DOUBLY_INDIRECT_INLINE correctly did not match itself"); }
    }
}
