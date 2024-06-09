use crate::begin_download::trauma::download::Summary;

pub fn check(list_summary: &[Summary], range_headers: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let vec_opt = list_summary
        .iter()
        .filter_map(|summary| {
            let filename = &summary.download().filename;

            let summary_size = summary.size();

            let mut uncomplete_file = None;

            let num_filename = filename
                .parse::<usize>()
                .expect("ERROR : need filename to be number");

            for (i, range) in range_headers.into_iter().enumerate() {
                if i == num_filename {
                    let target_size = range.1 - range.0;

                    if summary_size != target_size {
                        uncomplete_file = Some((range.0, range.1))
                    }
                }
            }

            uncomplete_file
        })
        .collect::<Vec<(u64, u64)>>();

    vec_opt
}
