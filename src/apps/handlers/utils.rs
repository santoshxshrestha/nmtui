pub fn move_cursor_right(string: &str, cursor_pos: &mut u16) {
    // ensuring the cursor does not go beyond the string length
    *cursor_pos = cursor_pos.saturating_add(1);
    *cursor_pos = (*cursor_pos).min(string.chars().count() as u16);
}

pub fn delete_char(string: &mut String, cursor_pos: &mut u16) {
    if *cursor_pos > 0 {
        let char_index_to_delete = *cursor_pos as usize - 1;
        // getting all the chars before the char to delete
        let before_char_to_delete = string.chars().take(char_index_to_delete);

        // getting all the chars after the car to delete
        let after_char_to_delete = string.chars().skip(*cursor_pos as usize);

        *string = before_char_to_delete.chain(after_char_to_delete).collect();
    }
}

pub fn enter_char(string: &mut String, c: char, cursor_pos: &u16) {
    let index = byte_index(string, cursor_pos);
    string.insert(index, c);
}

// getting the byte index of the cursor position in the string(utf-8)
pub fn byte_index(string: &str, cursor_pos: &u16) -> usize {
    string
        .char_indices()
        .map(|(i, _)| i)
        .nth(*cursor_pos as usize)
        .unwrap_or(string.len())
}
