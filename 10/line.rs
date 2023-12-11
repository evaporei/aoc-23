use std::rc::Rc;
use std::io::Read;
pub struct RcLineReaderIterator<TRead: Read> {
    reader: io::BufReader<TRead>,
    buffer: Rc<String>
}

impl<TRead: Read> RcLineReaderIterator<TRead> {
    fn new(reader: TRead) -> Self {
        Self { reader: io::BufReader::new(reader), buffer: Rc::new(String::new()) }
    }
}

impl<TRead: Read> Iterator for RcLineReaderIterator<TRead> {
    type Item = Result<std::rc::Rc<String>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf = if let Some(r) = Rc::get_mut(&mut self.buffer) {
            r.clear();
            r
        } else {
            self.buffer = Rc::new(String::new());
            Rc::get_mut(&mut self.buffer).unwrap()
        };
        match self.reader.read_line(buf) {
            Ok(n) if n > 0 => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(self.buffer.clone()))
            },
            Ok(_) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}
    let file = File::open("./simple_loop").unwrap();
    let it = RcLineReaderIterator::new(&file);

    for line in it {
        let line = line.unwrap();
        dbg!(line);
    }
    let it = RcLineReaderIterator::new(file);
    for line in it {
        let line = line.unwrap();
        dbg!(line);
    }
