#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::messages::Message;

    #[test]
    #[serial]
    fn it_should_create_new_messages() {
        let message = Message::genesis(
            "from".to_string(),
            "to".to_string(),
            "messsagee".to_string(),
        );

        assert_eq!(message.from, "from".to_string());
        assert_eq!(message.to, "to".to_string());
    }
}
