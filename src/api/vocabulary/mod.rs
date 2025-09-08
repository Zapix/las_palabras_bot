pub mod create;
pub mod detail;
pub mod list;
pub mod update;
pub mod detail_word_error;
pub mod delete;

pub use create::create_word;
pub use detail::get_word;
pub use list::list_words;
pub use update::update_word;
pub use delete::delete_word;
