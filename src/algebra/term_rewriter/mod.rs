struct Rule<'a> {
    input: &'a str,
    output: &'a str,
}

//we need a tree as input and output, but include named parameters that we replace

/*

    *
   / \
  /   \     ==>  0
 a     0

*/
