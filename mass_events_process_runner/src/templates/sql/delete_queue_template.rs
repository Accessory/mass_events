use askama::Template;

#[derive(Template)]
#[template(path = "delete_queue.sql.template", escape = "none")]
pub struct DeleteQueueTemplate<'a> {
    pub queue_name: &'a str,
}

// impl<'a> DeleteQueueTemplate<'a> {
//     pub fn new(queue_name: &'a str) -> Self {
//         Self { queue_name }
//     }
// }
