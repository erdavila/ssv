mod bytes {
    use std::ops::Deref;

    use ssv::bytes::{
        FluentWriter, Options, Position, ReadError, ReadResult, Reader, RowWriter, Token,
        Tokenizer, WithPosition, WriteError, WriteResult, Writer,
    };

    #[test]
    fn tokenizer() {
        let input = Vec::new();
        let mut tokenizer: Tokenizer<_> = Tokenizer::new(input.deref());

        match tokenizer.next() {
            Some(result) => {
                let result: ReadResult<_> = result;
                match result {
                    Ok(token_with_position) => {
                        let WithPosition {
                            value: token,
                            position,
                        } = token_with_position;
                        let _: Token = token;
                        let Position {
                            line_number: _,
                            column_number: _,
                        } = position;
                    }
                    Err(error) => {
                        let _: ReadError = error;
                    }
                }
            }
            None => (),
        }
    }

    #[test]
    fn reader() {
        let input = Vec::new();
        let mut reader: Reader<_> = Reader::new(input.deref());

        match reader.next() {
            Some(result) => {
                let result: ReadResult<_> = result;
                match result {
                    Ok(row) => {
                        let _: Vec<Vec<u8>> = row;
                    }
                    Err(error) => {
                        let _: ReadError = error;
                    }
                }
            }
            None => (),
        }
    }

    #[test]
    fn read() {
        let input = Vec::new();
        let _: Reader<_> = ssv::bytes::read(input.deref());
    }

    #[test]
    fn options() {
        let _: Options = Options::new();
    }

    #[test]
    fn fluent_writer() {
        let _: FluentWriter<_> = FluentWriter::new(Vec::new());
    }

    #[test]
    fn writer() {
        let mut writer: Writer<_> = Writer::new(Vec::new());
        let _: RowWriter<_> = writer.new_row();
    }

    #[test]
    fn write() {
        let result: WriteResult<_> = ssv::bytes::write(Vec::new(), Vec::<Vec<&[u8]>>::new());
        match result {
            Ok(()) => (),
            Err(error) => {
                let _: WriteError = error;
            }
        }
    }
}

mod chars {
    use std::ops::Deref;

    use ssv::chars::{
        FluentWriter, Options, Position, ReadError, ReadResult, Reader, RowWriter, Token,
        Tokenizer, WithPosition, WriteError, WriteResult, Writer,
    };

    #[test]
    fn tokenizer() {
        let input = Vec::new();
        let mut tokenizer: Tokenizer<_> = Tokenizer::new(input.deref());

        match tokenizer.next() {
            Some(result) => {
                let result: ReadResult<_> = result;
                match result {
                    Ok(token_with_position) => {
                        let WithPosition {
                            value: token,
                            position,
                        } = token_with_position;
                        let _: Token = token;
                        let Position {
                            line_number: _,
                            column_number: _,
                        } = position;
                    }
                    Err(error) => {
                        let _: ReadError = error;
                    }
                }
            }
            None => (),
        }
    }

    #[test]
    fn reader() {
        let input = Vec::new();
        let mut reader: Reader<_> = Reader::new(input.deref());

        match reader.next() {
            Some(result) => {
                let result: ReadResult<_> = result;
                match result {
                    Ok(row) => {
                        let _: Vec<String> = row;
                    }
                    Err(error) => {
                        let _: ReadError = error;
                    }
                }
            }
            None => (),
        }
    }

    #[test]
    fn read() {
        let input = Vec::new();
        let _: Reader<_> = ssv::chars::read(input.deref());
    }

    #[test]
    fn options() {
        let _: Options = Options::new();
    }

    #[test]
    fn fluent_writer() {
        let _: FluentWriter<_> = FluentWriter::new(Vec::new());
    }

    #[test]
    fn writer() {
        let mut writer: Writer<_> = Writer::new(Vec::new());
        let _: RowWriter<_> = writer.new_row();
    }

    #[test]
    fn write() {
        let result: WriteResult<_> = ssv::chars::write(Vec::new(), Vec::<Vec<&str>>::new());
        match result {
            Ok(()) => (),
            Err(error) => {
                let _: WriteError = error;
            }
        }
    }
}
