use askama::Template;

#[derive(Template)]
#[template(path = "create_queue.sql.template", escape = "none")]
pub struct CreateQueueTemplate<'a> {
    pub queue_name: &'a str,
}

impl<'a> CreateQueueTemplate<'a> {
    pub fn new(queue_name: &'a str) -> Self {
        Self { queue_name }
    }
}
