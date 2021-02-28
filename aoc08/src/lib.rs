pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
pub struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>,
    // length of metadata + length of children + 2 for header
    len: usize,
}

impl Node {
    pub fn new(input: &[i32]) -> Result<Self> {
        if input.len() < 2 {
            return Err(From::from("invalid header"));
        }

        let (child_count, meta_count) = (input[0], input[1]);
        let mut node = Node {
            len: 2,
            ..Node::default()
        };
        for _ in 0..child_count {
            let child = Node::new(&input[node.len..])?;
            node.len += child.len;
            node.children.push(child);
        }
        for _ in 0..meta_count {
            let meta = match input.get(node.len) {
                None => return Err(From::from("No metadata matching header")),
                Some(&i) if i < 1 => return Err(From::from("invalid meta data")),
                Some(&i) => i,
            };
            node.metadata.push(meta);
            node.len += 1;
        }

        Ok(node)
    }

    pub fn sum_metadata(&self) -> i32 {
        let mut sum = self.metadata.iter().sum();

        for child in self.children.iter() {
            sum += child.sum_metadata();
        }
        sum
    }

    pub fn sum_metadata_complex(&self) -> i32 {
        if self.children.len() == 0 {
            return self.metadata.iter().sum();
        }

        let mut sum = 0;
        for &i in self.metadata.iter() {
            if let Some(child) = self.children.get(i as usize - 1) {
                sum += child.sum_metadata_complex();
            }
        }
        sum
    }
}
