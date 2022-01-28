pub mod crf_search;
pub mod encode;
pub mod sample_encode;
pub mod vmaf;

pub use crf_search::crf_search;
pub use encode::encode;
pub use sample_encode::sample_encode;
pub use vmaf::vmaf;

const PROGRESS_CHARS: &str = "##-";
