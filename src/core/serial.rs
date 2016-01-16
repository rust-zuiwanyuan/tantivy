use core::global::*;
use core::schema::*;

// Trait sufficient to serialize a segment.
pub trait SerializableSegment<'a> {
    type TermCur: TermCursor<'a>; // TODO rename TermCursorImpl
    fn term_cursor(&'a mut self) -> Self::TermCur;
}

pub trait DocCursor: Iterator<Item=DocId> {
    fn doc(&self) -> DocId;
}


// TODO make iteration over Fields somehow sorted
// (Not only forms)
pub trait TermCursor<'a> {
    type DocCur: DocCursor;
    fn advance(&mut self,) -> bool;
    fn get_term(&self) -> Term<'a>;
    fn doc_cursor(&self) -> Self::DocCur;
}
