use crate::widgets::code_diff::diff_hunk::DiffHunk;
use crate::widgets::code_diff::diff_line::DiffLine;
use crate::widgets::code_diff::enums::DiffLineKind;

/// A pair of lines for side-by-side display.
///
/// In side-by-side mode, we need to align lines so that:
/// - Context lines appear on both sides at the same row
/// - Removed lines appear on the left, with empty space on the right
/// - Added lines appear on the right, with empty space on the left
#[derive(Debug, Clone)]
pub struct AlignedLinePair {
    /// The line to show on the left (old version).
    pub left: Option<DiffLine>,

    /// The line to show on the right (new version).
    pub right: Option<DiffLine>,
}

/// Builds aligned line pairs from a hunk for side-by-side display.
///
/// This function takes the lines from a hunk and pairs them up so that:
/// - Context lines have the same content on both sides
/// - Removed lines are on the left with None on the right
/// - Added lines are on the right with None on the left
/// - Consecutive removes/adds are paired up when possible
///
/// # Arguments
///
/// * `hunk` - The diff hunk to process
///
/// # Returns
///
/// A vector of aligned line pairs for rendering
pub fn build_aligned_lines(hunk: &DiffHunk) -> Vec<AlignedLinePair> {
    let mut pairs: Vec<AlignedLinePair> = Vec::new();
    let mut pending_removes: Vec<DiffLine> = Vec::new();
    let mut pending_adds: Vec<DiffLine> = Vec::new();

    for line in &hunk.lines {
        match line.kind {
            DiffLineKind::Context | DiffLineKind::HunkHeader => {
                // Flush pending removes/adds before context
                flush_pending(&mut pairs, &mut pending_removes, &mut pending_adds);

                // Context lines appear on both sides
                pairs.push(AlignedLinePair {
                    left: Some(line.clone()),
                    right: Some(line.clone()),
                });
            }
            DiffLineKind::Removed => {
                // If we have pending adds, we should flush first
                // (removes should come before adds in the grouping)
                if !pending_adds.is_empty() {
                    flush_pending(&mut pairs, &mut pending_removes, &mut pending_adds);
                }
                pending_removes.push(line.clone());
            }
            DiffLineKind::Added => {
                pending_adds.push(line.clone());
            }
        }
    }

    // Flush any remaining pending lines
    flush_pending(&mut pairs, &mut pending_removes, &mut pending_adds);

    pairs
}

/// Flushes pending removed and added lines into aligned pairs.
///
/// This pairs up removes with adds where possible, showing them
/// side by side. Any extras are shown with None on the opposite side.
fn flush_pending(
    pairs: &mut Vec<AlignedLinePair>,
    pending_removes: &mut Vec<DiffLine>,
    pending_adds: &mut Vec<DiffLine>,
) {
    let remove_count = pending_removes.len();
    let add_count = pending_adds.len();
    let max_count = remove_count.max(add_count);

    for i in 0..max_count {
        let left = pending_removes.get(i).cloned();
        let right = pending_adds.get(i).cloned();
        pairs.push(AlignedLinePair { left, right });
    }

    pending_removes.clear();
    pending_adds.clear();
}
