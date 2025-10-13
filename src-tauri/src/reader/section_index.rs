//! Section indexer skeleton: detect headings/anchors and return a page map.
// TODO: implement using PDF text extraction + simple heuristics.

#[allow(dead_code)]
pub struct SectionAnchor { pub page: u32, pub title: String }

#[allow(dead_code)]
pub fn build_index(_pdf_path: &str) -> Vec<SectionAnchor> {
  // TODO: parse PDF text layer and detect candidates.
  vec![]
}
