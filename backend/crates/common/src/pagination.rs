use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdeaCursor {
    pub created_block_height: i64,
    pub created_event_index: i32,
    pub idea_id: Uuid,
}

pub fn parse_ideas_cursor(cursor: &str) -> Option<IdeaCursor> {
    let mut parts = cursor.split(':');
    let height_str = parts.next()?;
    let index_str = parts.next()?;
    let idea_id_str = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let height: i64 = height_str.parse().ok()?;
    let index_i64: i64 = index_str.parse().ok()?;
    let index: i32 = index_i64.try_into().ok()?;
    let idea_id = Uuid::parse_str(idea_id_str).ok()?;
    if height < 0 || index < 0 {
        return None;
    }
    Some(IdeaCursor {
        created_block_height: height,
        created_event_index: index,
        idea_id,
    })
}

pub fn format_ideas_cursor(cursor: IdeaCursor) -> String {
    format!(
        "{}:{}:{}",
        cursor.created_block_height, cursor.created_event_index, cursor.idea_id
    )
}
