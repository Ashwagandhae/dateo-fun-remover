pub const TREE_5: [(&str, &str); 3] = [
    (
        r"
   O
  / \
  N O
   / \
   N O
",
        r"
  H
 / \
 N H
  / \
  N G
",
    ),
    (
        r"
  O
 / \
 N O
  / \
  N O
",
        r"
   H
  / \
  O G
 / \
 N N
",
    ),
    (
        r"
     O
   /   \
   O   O
  / \ / \
  N N N N
",
        r"
   H
  / \
  N G

",
    ),
];

pub const TREE_4: [(&str, &str); 2] = [
    (
        r"
   O
  / \
  N N
",
        r"
  H
 / \
 N H
  / \
  N G
",
    ),
    (
        r"
   O
  / \
  N N
",
        r"
   H
  / \
  O G
 / \
 N N
",
    ),
];

pub const TREE_3: [(&str, &str); 1] = [(
    r"
      O
     / \
     N N
     ",
    r"
      H
     / \
     N G
     ",
)];

pub const TREE_2: [(&str, &str); 1] = [(
    r"
      O
     / \
     N N
     ",
    r"
      G
     ",
)];

pub const TREE_1: [(&str, &str); 1] = [(
    r"
      N
     ",
    r"
      G
     ",
)];
