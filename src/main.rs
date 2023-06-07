use crate::{api::get_debug, models::Post};

mod api;
mod util;
mod models;

fn main() {
    println!("cling v0.1.0; CLI client for Ping; edit ./config.json to change your settings;\n");

    let default_prefix = "> ";
    loop {
        let input = util::read_console(default_prefix);
        let output = match input.as_str() {
            "debug" => get_debug(),
            "boards" => {
                let json = api::get_boards();

                let boards: Vec<String> = serde_json::from_str(&json).unwrap();
                let mut output = String::new();
                for board in boards.iter() {
                    output.push_str(&format!("{}\n", board));
                }
                output
            },
            "posts" => {
                let board = util::read_console("Board: ");
                let json = api::get_posts(&board);

                let posts: Vec<Post> = serde_json::from_str(&json).unwrap();
                let mut output = String::new();
                for post in posts.iter() {
                    let mut reference = String::new();
                    if post.ref_id != 0 {
                        reference.push_str(&format!(" (reply to {})", post.ref_id));
                    }
                    output.push_str(&format!("\n{}\n'{}' (ID: {})\n- by {}\n- on {}\n", reference, post.content, post.id, post.username, post.time));
                }
                output
            },
            "post" => {
                let id = util::read_console("ID: ");
                let id = id.parse::<i64>().unwrap();
                let json = api::get_post(id);

                let post: Post = serde_json::from_str(&json).unwrap();
                let mut output = String::new();

                let mut reference = String::new();
                if post.ref_id != 0 {
                    reference.push_str(&format!(" (reply to {})", post.ref_id));
                }
                output.push_str(&format!("\n{}\n'{}' (ID: {})\n- by {}/{}\n- on {}\n", reference, post.content, post.id, post.username, post.board, post.time));
                output
            },
            "comments" => {
                let id = util::read_console("ID: ");
                let id = id.parse::<i64>().unwrap();
                let json = api::get_comments(id);

                let comments: Vec<Post> = serde_json::from_str(&json).unwrap();
                let mut output = String::new();
                for comment in comments.iter() {
                    let mut reference = String::new();
                    if comment.ref_id != 0 {
                        reference.push_str(&format!(" (reply to {})", comment.ref_id));
                    }
                    output.push_str(&format!("\n{}\n'{}' (ID: {})\n- by {}\n- on {}\n", reference, comment.content, comment.id, comment.username, comment.time));
                }
                output
            },
            "create" => {
                let thumb_url = util::read_console("Thumbnail URL (optional): ");
                let content = util::read_console("Content: ");
                let ref_id = util::read_console("Reference ID (optional): ");
                let ref_id = ref_id.parse::<i64>().unwrap_or(0);
                let board = util::read_console("Board: ");
                let result = api::create_post(Some(thumb_url), content, Some(ref_id), board);
                result
            },
            "help" => {
                let mut output = String::new();
                output.push_str("debug - Get debug info\n");
                output.push_str("boards - Get list of boards\n");
                output.push_str("posts - Get list of posts in a board\n");
                output.push_str("post - Get a post by ID\n");
                output.push_str("comments - Get list of comments in a post\n");
                output.push_str("create - Create a post\n");
                output.push_str("help - Show this help message\n");
                output.push_str("exit - Exit the program\n");
                output
            },
            "exit" => break,
            _ => "Unknown command".to_string(),
        };
        println!("{}\n", output);
    }

    println!("Goodbye!");
}
