
pub fn split_range(start: u64, max: u64, breaks: u64) -> Vec<(u64, u64)> {
    let mut out= vec![];
    let diff = max / breaks;
    let mut end = 0;
    let mut start = start;
    for i in 0..breaks {
        end += diff;
        if i == breaks - 1 {
            end += max - end;
        }
        let new_range = (start, end);
        start += diff;
        out.push(new_range);
    }
    out
}